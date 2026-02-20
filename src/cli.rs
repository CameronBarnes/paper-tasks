use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version = "0.1", about = "An application to manage daily tasks by printing them out as receipts", long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "PATH")]
    pub config_dir: Option<PathBuf>,

    /// Path for the sqlite db file
    #[arg(short, long)]
    pub database_path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Pipe out all pending tasks to stdio
    Pipe,
    /// Print all pending tasks to the configured printer
    Print,
    /// Draw a Text User Interface to manage tasks
    Show
}

impl Default for Commands {
    fn default() -> Self {
        Self::Show
    }
}


