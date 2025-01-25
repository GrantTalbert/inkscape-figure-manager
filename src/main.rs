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
    // Creates a file
    #[command(name = "--create")]
    Create {
        title: String,
        #[arg(value_parser = clap::value_parser!(std::path::PathBuf))]
        path: PathBuf,
    },

    // Inserts LaTeX code for a file
    #[command(name = "--insert")]
    Insert {
        path: PathBuf
    },

    // Edits a file
    #[command(name = "--edit")]
    Edit {
        path: PathBuf
    },

    // Starts [kills] the daemon
    #[command(name = "--daemon")]
    Daemon {
        option: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { title, path } => {
            create(title, path);
        }
        Commands::Insert { path } => {
            println!("{}", insert(path));
        }
        Commands::Edit { path } => {
            edit(path);
        }
        Commands::Daemon { option }=> {
            match option.as_str() {
                "kill" => {
                    daemon::daemon::kill_daemon()
                }
                _ => {
                    daemon::daemon::start_daemon()
                }
            }
        }
    }
}
