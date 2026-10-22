use std::ffi::OsString;
use std::process::ExitCode;

use clap::Parser;
use lambda::cli::{Cli, Command};
use lambda::data::{self, Category, Symbol};
use lambda::{render, store, sysinfo};

fn main() -> ExitCode {
    let cli = Cli::parse_from(normalize_args(std::env::args_os()));

    match cli.command {
        Some(command) => run_command(command, cli.uppercase),
        None => run_selection(&cli),
    }
}

fn normalize_args(args: impl IntoIterator<Item = OsString>) -> Vec<OsString> {
    args.into_iter()
        .map(|arg| match arg.to_str() {
            Some("-Up" | "-up" | "--Up") => OsString::from("--uppercase"),
            _ => arg,
        })
        .collect()
}

fn run_selection(cli: &Cli) -> ExitCode {
    let mut categories = Vec::new();

    if cli.all {
        categories.extend(Category::BUILTIN);
        if !store::load().is_empty() {
            categories.push(Category::Custom);
        }
    } else {
        if cli.greek {
            categories.push(Category::Greek);
        }
        if cli.math {
            categories.push(Category::Math);
        }
        if cli.physics {
            categories.push(Category::Physics);
        }
        if cli.custom {
            categories.push(Category::Custom);
        }
    }

    if categories.is_empty() {
        sysinfo::print_banner();
        return ExitCode::SUCCESS;
    }

    let mut chars: Vec<char> = categories
        .into_iter()
        .flat_map(|category| symbols_for(category).into_iter().map(|symbol| symbol.ch))
        .collect();

    if cli.uppercase {
        chars = chars.into_iter().map(render::uppercase).collect();
    }

    render::grid(&chars);
    ExitCode::SUCCESS
}

fn run_command(command: Command, uppercase: bool) -> ExitCode {
    match command {
        Command::List { category } => {
            let symbols = match category {
                Some(name) => match Category::parse(&name) {
                    Some(category) => symbols_for(category),
                    None => {
                        eprintln!("unknown category: {name}");
                        return ExitCode::from(2);
                    }
                },
                None => {
                    let mut symbols = data::builtins();
                    symbols.extend(symbols_for(Category::Custom));
                    symbols
                }
            };

            print_symbols(&symbols, uppercase);
            ExitCode::SUCCESS
        }
        Command::Categories => {
            for category in Category::ALL {
                println!(
                    "{:<10} {} symbols",
                    category.name(),
                    symbols_for(category).len()
                );
            }
            ExitCode::SUCCESS
        }
        Command::Add { symbol, keyword } => {
            let ch = match single_char(&symbol) {
                Ok(ch) => ch,
                Err(message) => {
                    eprintln!("{message}");
                    return ExitCode::from(2);
                }
            };

            match store::add(ch, &keyword) {
                Ok(true) => {
                    println!("updated {keyword} -> {ch}");
                    ExitCode::SUCCESS
                }
                Ok(false) => {
                    println!("added {keyword} -> {ch}");
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("could not save symbol: {error}");
                    ExitCode::FAILURE
                }
            }
        }
        Command::Remove { keyword } => match store::remove(&keyword) {
            Ok(true) => {
                println!("removed {keyword}");
                ExitCode::SUCCESS
            }
            Ok(false) => {
                eprintln!("no custom symbol with keyword {keyword}");
                ExitCode::from(2)
            }
            Err(error) => {
                eprintln!("could not save symbols: {error}");
                ExitCode::FAILURE
            }
        },
        Command::Custom => {
            print_symbols(&symbols_for(Category::Custom), uppercase);
            ExitCode::SUCCESS
        }
    }
}

fn symbols_for(category: Category) -> Vec<Symbol> {
    match category {
        Category::Custom => store::load()
            .into_iter()
            .map(|symbol| Symbol {
                ch: symbol.ch,
                name: symbol.keyword,
                category: Category::Custom,
            })
            .collect(),
        other => data::for_category(other),
    }
}

fn print_symbols(symbols: &[Symbol], uppercase: bool) {
    if symbols.is_empty() {
        println!("no symbols");
        return;
    }

    for symbol in symbols {
        let ch = if uppercase {
            render::uppercase(symbol.ch)
        } else {
            symbol.ch
        };
        println!("{ch}  {:<38} [{}]", symbol.name, symbol.category);
    }
}

fn single_char(value: &str) -> Result<char, String> {
    let mut chars = value.chars();
    match (chars.next(), chars.next()) {
        (Some(ch), None) => Ok(ch),
        _ => Err(format!(
            "expected exactly one character for the symbol, got {value:?}"
        )),
    }
}
