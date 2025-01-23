use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::process::Command;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};

static IPC_FILE: &str = "/tmp/inkscape_figure_manager_ipc";
pub fn start_daemon() {
    let watched_files: Arc<Mutex<HashSet<PathBuf>>> = Arc::new(Mutex::new(HashSet::new()));
    let watched_files_clone = Arc::clone(&watched_files);

    let mut watcher = RecommendedWatcher::new(move |res: Result<Event, notify::Error>| match res {
        Ok(event) => {
            if let Some(path) = event.paths.first() {
                println!("File event detected: {:?}", path);

                let output = Command::new("echo")
                    .arg(format!("File updated: {:?}", path))
                    .output()
                    .expect("failed to execute process");
                println!("{}", String::from_utf8_lossy(&output.stdout));
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
        if let Ok(mut file) = OpenOptions::new().read(true).open(IPC_FILE) {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).unwrap();
            file.set_len(0).unwrap();

            let commands: Vec<&str> = buffer.trim().split_whitespace().collect();
            if commands.len() < 2 {
                continue;
            }

            let action = commands[0];
            let path = PathBuf::from(commands[1]);
            
            match action {
                "create" | "edit" => {
                    let mut files = watched_files_clone.lock().unwrap();

                    if files.insert(path.clone()) {
                        watcher
                            .watch(&path, RecursiveMode::NonRecursive)
                            .expect("Failed to watch file");
                        println!("Now watching file {:?}", path);
                    } else {
                        println!("Already watching {:?}", path);
                    }
                }
                _ => eprintln!("Unknown action: {}", action),
            }
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