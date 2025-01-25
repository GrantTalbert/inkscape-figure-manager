use std::fs;
use std::path::PathBuf;
use std::process::{Command, exit};
use crate::utils::general::{communicate_daemon, get_latex_code, get_template_file, open_file};

pub fn create(title: String, path: PathBuf) {
    let target_file = path.join(format!("{}.svg", title));

    if target_file.exists() { return; }

    fs::copy(get_template_file(), &target_file).expect("failed to copy file");

    communicate_daemon(target_file.clone(), "add");
    open_file(target_file);
    println!("{}", get_latex_code(&title));
}