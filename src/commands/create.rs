use std::fmt::format;
use std::path::PathBuf;
use std::process::{Command, exit};
use crate::utils::general::get_template_file;

pub fn create(title: String, path: PathBuf) {
    let target_file = path.join(format!("{}.svg", title));

    println!("Creating file: {}", target_file.display());

    if target_file.exists() {
        println!("File already exists, exiting");
        return;
    }
    println!("File does not exist!");

    let copy_command = Command::new("cp")
        .arg(get_template_file().to_str().unwrap())
        .arg(target_file.to_str().unwrap())
        .status()
        .expect("failed to copy template");

    if !copy_command.success() {
        eprintln!("Failed to copy template");
        exit(1);
    }

    println!("File {} created successfully", format!("{}/{}.svg", path.display(), title));

    let open_command = Command::new("inkscape")
        .arg(target_file.to_str().unwrap())
        .status()
        .expect("Failed to open inkscape");

    if !open_command.success() {
        eprintln!("Failed to open inkscape");
        exit(1);
    }
}