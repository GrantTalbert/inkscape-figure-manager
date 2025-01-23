use std::fs;
use std::path::PathBuf;
use std::process::{Command, exit};
use crate::utils::general::{communicate_daemon, get_template_file, open_file};

pub fn create(title: String, path: PathBuf) {
    let target_file = path.join(format!("{}.svg", title));

    println!("Creating file: {}", target_file.display());

    if target_file.exists() {
        println!("File already exists, exiting");
        return;
    }
    println!("File does not exist!");

    fs::copy(get_template_file(), &target_file).expect("failed to copy file");

    println!("File {} created successfully", format!("{}/{}.svg", path.display(), title));

    communicate_daemon(target_file.clone());
    open_file(target_file);
}