use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lambda", about = "Browse special Unicode symbols", version)]
pub struct Cli {
    /// Show the Greek alphabet
    #[arg(short = 'g', long)]
    pub greek: bool,

    /// Show mathematical symbols
    #[arg(short = 'm', long)]
    pub math: bool,

    /// Show physical symbols
    #[arg(short = 'p', long)]
    pub physics: bool,

    /// Show your custom symbols
    #[arg(short = 'c', long)]
    pub custom: bool,

    /// Show every category
    #[arg(short = 'a', long)]
    pub all: bool,

    /// Convert letters to uppercase (also accepts -Up)
    #[arg(short = 'U', long = "uppercase")]
    pub uppercase: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Show symbols together with their names
    List {
        /// Show only this category
        #[arg(short, long)]
        category: Option<String>,
    },

    /// Show available categories
    Categories,

    /// Add or update a custom symbol
    Add {
        /// A single Unicode character
        symbol: String,

        /// Keyword used to describe the symbol
        keyword: String,
    },

    /// Remove a custom symbol by keyword
    Remove {
        /// Keyword of the symbol to remove
        keyword: String,
    },

    /// Show custom symbols only
    Custom,
}
