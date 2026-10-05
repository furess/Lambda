use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lambda", about = "Browse special Unicode symbols")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Show all symbols
    List {
        /// Show only this category
        #[arg(short, long)]
        category: Option<String>,
    },
    /// Show available categories
    Categories,
}
