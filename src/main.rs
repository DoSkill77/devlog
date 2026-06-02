mod cli;
mod models;
mod storage;
use clap::Parser;

use crate::cli::{Cli, add, list, today};
fn main() {
    let cli = Cli::parse();
    match cli.command {
        cli::Commands::Add { message, tag } => add(message, tag),
        cli::Commands::Today => today(),
        cli::Commands::List { tag } => list(tag),
    }
}
