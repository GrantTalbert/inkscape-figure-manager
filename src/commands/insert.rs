use std::fs;
use std::path::PathBuf;
use crate::utils::rofi::rofi::rofi;

pub fn insert(directory: PathBuf) {
    let files = fs::read_dir(&directory)
        .expect("Could not read directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| {
            entry.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "svg")
                .unwrap_or(false)
        })
        .map(|entry| entry.file_name().to_string_lossy().splitn(2, '.').next().unwrap().to_string())
        .collect::<Vec<String>>();

    if files.is_empty() {
        println!("No files to insert!");
        return;
    }

    let file_names: Vec<&str> = files.iter().map(|file_name| file_name.as_str()).collect();

    let selected_file = rofi(&file_names);

    println!("{}", selected_file);
}