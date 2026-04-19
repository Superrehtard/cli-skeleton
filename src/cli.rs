use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "filetool")]
#[command(version = "0.1.0")]
#[command(about = "A demo file processing CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Count {
        input: PathBuf,

        #[arg(short, long)]
        words: bool,
    },
    Search {
        input: PathBuf,
        pattern: String,

        #[arg(short = 'i', long)]
        case_insensitive: bool,
    },
    Stats {
        input: PathBuf,
    },
}
