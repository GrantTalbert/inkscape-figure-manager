use std::path::PathBuf;
use dirs::home_dir;

pub fn get_template_file() -> PathBuf {
    let home = home_dir();
    let config_path = home.unwrap().join(".config")
        .join("inkscape-figure-manager")
        .join("template.svg");

    config_path
}