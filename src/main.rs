mod commands;
mod utils;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::commands::create::create;
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
    }
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
            insert(path);
        }
    }
}
