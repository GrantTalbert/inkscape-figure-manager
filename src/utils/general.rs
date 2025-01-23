//! General utility functions
use std::fs;
use std::path::PathBuf;
use std::process::{exit, Command};
use dirs::home_dir;

/// Returns the template file
/// The template file is saved at ~/.config/inkscape-figure-manager/template.svg
pub fn get_template_file() -> PathBuf {
    let home = home_dir();
    let config_path = home.unwrap().join(".config")
        .join("inkscape-figure-manager")
        .join("template.svg");

    config_path
}

/// Returns the LaTeX code for inserting the figure
pub fn get_latex_code(selected_file: &str) -> String {
    ["\\begin{figure}[ht]",
        "    \\centering",
        format!("    \\incfig{}{}{}", "{", selected_file, "}").as_str(),
        format!("    \\caption{}{}{}\\label{}{}{}", "{", selected_file, "}", "{fig:", selected_file, "}").as_str(),
        "\\end{figure}"
    ].join("\n")
}

/// Searches directory for all files with the given extension
/// Returns them as a Vec<String>
pub fn parse_dir_for_extension(extension: &str, directory: &PathBuf) -> Vec<String> {
    fs::read_dir(&directory)
        .expect("Could not read directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| {
            entry.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == extension)
                .unwrap_or(false)
        })
        .map(|entry| entry.file_name().to_string_lossy().splitn(2, '.').next().unwrap().to_string())
        .collect::<Vec<String>>()
}

/// Attempts to open a file at the given path in inkscape
pub fn open_file(path: PathBuf){
    let open_command = Command::new("inkscape")
        .arg(path.to_str().unwrap())
        .status()
        .expect("Failed to open inkscape");

    if !open_command.success() {
        eprintln!("Failed to open inkscape");
        exit(1);
    }
}