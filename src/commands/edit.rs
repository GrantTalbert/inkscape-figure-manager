use std::path::PathBuf;
use crate::utils::general::{communicate_daemon, get_latex_code, open_file, parse_dir_for_extension};
use crate::utils::rofi::rofi::rofi;

pub fn edit(directory: PathBuf) -> String {
    let files = parse_dir_for_extension("svg", &directory);

    if files.is_empty() {
        println!("No files to insert!");
        return String::new();
    }

    let file_names: Vec<&str> = files.iter().map(|file_name| file_name.as_str()).collect();

    let selected_file = rofi(&file_names);
    if selected_file.is_empty() {
        println!("No file selected!");
        return String::new();
    }

    communicate_daemon(PathBuf::from(selected_file.clone()));
    open_file(directory.join(PathBuf::from(selected_file.clone())));
    get_latex_code(&selected_file)
}