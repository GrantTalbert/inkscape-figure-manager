mod commands;
mod utils;
mod daemon;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::commands::create::create;
use crate::commands::edit::edit;
use crate::commands::insert::insert;

#[derive(Parser)]
#[command(
    name = "CLI tool for managing inkscape figures",
    version = "0.0.1",
    author = "Hyperion",
    about = "Implements various commands for opening Inkscape at certain directories and closing on figure save"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Starts the daemon
    #[command(name = "--watch")]
    Watch,

    // Creates a file
    #[command(name = "--create")]
    Create {
        title: String,
        #[arg(value_parser = clap::value_parser!(std::path::PathBuf))]
        path: PathBuf,
    },

    #[command(name = "--insert")]
    Insert {
        path: PathBuf
    },

    #[command(name = "--edit")]
    Edit {
        path: PathBuf
    },

    #[command(name = "--daemon")]
    Daemon {
        option: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Watch => {
            println!("Watching...");
        }
        Commands::Create { title, path } => {
            println!("Creating...");
            create(title, path);
        }
        Commands::Insert { path } => {
            println!("{}", insert(path));
        }
        Commands::Edit { path } => {
            println!("{}", edit(path));
        }
        Commands::Daemon { option }=> {
            match option.as_str() {
                "kill" => {
                    daemon::daemon::kill_daemon()
                }
                _ => {
                    std::process::Command::new(std::env::current_exe().unwrap())
                        .arg("--daemon start")
                        .spawn()
                        .expect("Could not start daemon");
                    println!("Daemon started");
                }
            }
        }
    }
}
