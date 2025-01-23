use std::path::PathBuf;
use crate::utils::general::{get_latex_code, open_file, parse_dir_for_extension};
use crate::utils::rofi::rofi::rofi;

pub fn edit(directory: PathBuf) -> String {
    let files = parse_dir_for_extension("svg", &directory);

    if files.is_empty() {
        println!("No files to insert!");
        return String::new();
    }

    let file_names: Vec<&str> = files.iter().map(|file_name| file_name.as_str()).collect();

    let selected_file = rofi(&file_names);

    open_file(directory.join(PathBuf::from(format!("{}.svg", selected_file))));
    get_latex_code(&selected_file)
}