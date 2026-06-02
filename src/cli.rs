use crate::{models::Entry, storage};
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        message: String,
        #[arg(long)]
        tag: Option<String>,
    },
    Today,
    List {
        #[arg(long)]
        tag: Option<String>,
    },
}
/// Ajoute une nouvelle entrée dans le journal
///
/// # Arguments
/// * `message` - Le texte de l'entrée
/// * `tag` - Tag optionnel pour catégoriser
pub fn add(message: String, tag: Option<String>) {
    let entries = storage::load();
    let id = entries.len() as u32 + 1;
    let entry = Entry {
        id,
        date: chrono::Local::now().date_naive(),
        message,
        tag,
    };
    storage::save(entry);
}

pub fn today() {
    storage::load()
        .iter()
        .filter(|entry| entry.date == chrono::Local::now().date_naive())
        .for_each(|entry| {
            println!(
                "{} {} {}",
                entry.date.to_string().dimmed(),
                entry.message.green(),
                entry.tag.as_deref().unwrap_or("").yellow()
            )
        })
}
pub fn list(tag: Option<String>) {
    storage::load()
        .iter()
        .filter(|entry| tag.is_none() || entry.tag == tag)
        .for_each(|entry| {
            println!(
                "{} {} {}",
                entry.date.to_string().dimmed(),
                entry.message.green(),
                entry.tag.as_deref().unwrap_or("").yellow()
            )
        })
}
