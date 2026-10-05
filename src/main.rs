use clap::Parser;
use lambda::cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::List { category } => println!("list, category = {:?}", category),
        Command::Categories => println!("categories"),
    }
}
