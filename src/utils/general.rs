//! General utility functions
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::{Command};
use std::ptr::read;
use dirs::home_dir;
use crate::commands::create::create;

pub fn get_config_dir() -> PathBuf {
    let home = home_dir().expect("Failed to get your home directory");
    home.join(".config").join("inkscape-figure-manager")
}

/// Returns the template file
/// The template file is saved at ~/.config/inkscape-figure-manager/template.svg
pub fn get_template_file() -> PathBuf {
    let home = home_dir().expect("Failed to get your home directory");
    let config_path = home.join(".config")
        .join("inkscape-figure-manager")
        .join("template.svg");

    config_path
}

/// Returns the LaTeX code for inserting the figure
pub fn get_latex_code(selected_file: &str) -> String {
    ["\\begin{figure}[ht]",
        format!("    \\incfig{{{}}}", selected_file).as_str(),
        "    \\centering",
        format!("    \\caption{{{}}}\\label{{fig:{}}}", selected_file, selected_file).as_str(),
        "    \\centering",
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
pub fn open_file(mut path: PathBuf){
    if path.extension() == None { path = PathBuf::from(format!("{}.svg", path.to_string_lossy())) }
    let open_command = Command::new("inkscape")
        .arg(path.to_string_lossy().as_ref())
        .status()
        .expect("Failed to open inkscape");

    if !open_command.success() {}
}

pub fn communicate_daemon(mut path: PathBuf, status: &str){
    match OpenOptions::new().append(true).create(true).open("/tmp/inkscape_figure_manager_ipc") {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            if path.extension().is_none() {
                path.set_extension("svg");
            }
            if let Err(_err) = writeln!(writer, "{} {}", path.display(), status) {}
        }
        Err(_err) => {},
    }
}

pub fn close_inkscape() {
    let output = Command::new("killall")
        .arg("-e")
        .arg("inkscape")
        .status();
}