use std::path::PathBuf;
use crate::utils::general::{get_latex_code, parse_dir_for_extension};
use crate::utils::rofi::rofi::rofi;

pub fn insert(directory: PathBuf) -> String {
    let files = parse_dir_for_extension("pdf_tex", &directory);

    if files.is_empty() {
        return String::new();
    }

    let file_names: Vec<&str> = files.iter().map(|file_name| file_name.as_str()).collect();

    let selected_file = rofi(&file_names);
    if selected_file.is_empty() {
        return String::new();
    }

    get_latex_code(&selected_file)
}