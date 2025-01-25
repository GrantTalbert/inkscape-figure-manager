use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc};
use std::process::Command;
use std::time::{Duration, Instant};
use clap::builder::PathBufValueParser;
use notify::{recommended_watcher, Event, RecommendedWatcher, RecursiveMode, Watcher};
use crate::utils::general::{close_inkscape, communicate_daemon, remove_from_ipc};

static IPC_FILE: &str = "/tmp/inkscape_figure_manager_ipc";
const DEBOUNCE_DURATION: Duration = Duration::from_millis(300);
pub fn start_daemon() {
    let watched_files: Arc<Mutex<HashSet<PathBuf>>> = Arc::new(Mutex::new(HashSet::new()));
    let watched_files_clone = Arc::clone(&watched_files);
    let first_events: Arc<Mutex<HashSet<PathBuf>>> = Arc::new(Mutex::new(HashSet::new()));
    let first_events_clone = Arc::clone(&first_events);
    let last_update_time: Arc<Mutex<HashMap<PathBuf, Instant>>> = Arc::new(Mutex::new(HashMap::new()));
    let last_update_time_clone = Arc::clone(&last_update_time);
    let last_events: Arc<Mutex<HashSet<PathBuf>>> = Arc::new(Mutex::new(HashSet::new()));

    if !Path::new(IPC_FILE).exists() {
        File::create(IPC_FILE).expect("Failed to create IPC socket");
        println!("IPC socket created");
    }

    let mut watcher = RecommendedWatcher::new(move |res: Result<Event, notify::Error>| match res {
        Ok(event) => {
            if let Some(path) = event.paths.first() {

                let now = Instant::now();
                let mut last_update_time = last_update_time_clone.lock().unwrap();
                let mut first_events = first_events_clone.lock().unwrap();
                let last_update = last_update_time.get(path).cloned();
                let mut last_events = last_events.lock().unwrap();
                last_update_time.insert(path.clone(), now);

                // opening the file spawns a singleton event
                // saving the file spawns multiple
                // this logic implements a debounce time; it returns if it doesn't detect 2 events in short succession
                if let Some(last_update) = last_update {
                    if now.duration_since(last_update) > DEBOUNCE_DURATION {
                        println!("Debounce of {:?}", path);
                        return;
                    }
                }

                // There is a single case of bypassing debounce
                // We ignore the first non-debounced case
                if first_events.contains(path) {
                    first_events.remove(path);
                    last_events.insert(path.clone());
                    println!("Ignoring first non-debounced event: {:?}", path);
                    return;
                }

                //if last_events.contains(path) {
                //    return;
                //}

                println!("Non-debounced event detected: {:?}, now processing", path);

                if let Some(filename) = path.file_stem() {
                    let output_filename = path.with_file_name(filename);

                    let inkscape_status = Command::new("inkscape")
                        .arg(path.to_str().unwrap())
                        .arg("--export-area-page")
                        .arg("--export-dpi")
                        .arg("300")
                        .arg("--export-type=pdf")
                        .arg("--export-latex")
                        .arg("--export-filename")
                        .arg(output_filename.to_str().unwrap())
                        .status();

                    match inkscape_status {
                        Ok(status) if status.success() => {
                            println!("Pdf exported!");
                            communicate_daemon(path.clone(), "kill");
                            close_inkscape();
                            std::thread::sleep(Duration::from_secs(1));
                        }
                        Ok(status) => {
                           println!("Failed with status: {}", status);
                        }
                        Err(error) => {
                            eprintln!("failed with error: {}", error);
                        }
                    }
                    return;
                }
            }
        }
        Err(error) => {
            eprintln!("watch error: {:?}", error);
        }
    }, notify::Config::default()
    )
        .expect("Failed to create watcher");

    println!("Daemon started successfully!");

    loop {
        if let Ok(mut file) = OpenOptions::new().read(true).write(true).open(IPC_FILE) {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).expect("failed to read from IPC file");

            if buffer.trim().is_empty() {
                //std::thread::sleep(Duration::from_millis(500));
                continue;
            }

            let commands: Vec<&str> = buffer.trim().split_whitespace().collect();
            if commands.is_empty() {
                continue;
            }

            let path = PathBuf::from(commands[0]);
            let status = String::from(commands[1]);
            let mut watched_files = watched_files.lock().unwrap();

            if status == "kill" {
                watched_files.remove(&path);
                watcher.unwatch(&path).expect("failed to unwatch");
                println!("KILLED");
            } else if watched_files.insert(path.clone()) {
                watcher
                    .watch(&path, RecursiveMode::NonRecursive)
                    .expect("Failed to watch file");
                println!("Now watching file {:?}", path);
                first_events.lock().unwrap().insert(path);
            } else {
                println!("Already watching {:?}", path);
            }

            file.set_len(0).expect("Failed to truncate the IPC file");
        } else {
            println!("Daemon stopped successfully!");
            break;
        }
    }
}

pub fn kill_daemon() {
    if std::fs::remove_file(IPC_FILE).is_ok() {
        println!("Daemon killed");
    } else {
        eprintln!("Daemon IPC not found");
    }
}