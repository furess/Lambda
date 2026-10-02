# lambda

A small CLI tool written in rust for browsing special Unicode symbols: math, alphabets, currencies and more. A learning project for practicing Rust.
The symbol database is embedded into the binary, so no internet connection or external files are needed.(this is the point)

## Usage

```
lambda list                      # show all symbols
lambda list --category math      # show symbols from one category only
lambda categories                # list available categories
```

Example output:

```
λ  greek small letter lambda  [alphabets]
∑  n-ary summation            [math]
→  rightwards arrow           [arrows]
```
## Installation

Requires [Rust](https://rustup.rs) to be installed.

```
git clone <repository url>
cd lambda
cargo install --path .
```

After that, the `lambda` command is available from any directory.

## Adding symbols

Open `data/symbols.json` and add an entry:

```json
{ "ch": "√", "name": "square root", "category": "math" }
```

rebuild the project and the symbol will show up in the list

## Status

Work in progress. Done: data loading. Planned: `list` and `categories` commands, column and output

## Built with

- [clap](https://crates.io/crates/clap): argument parsing
- [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json): JSON loading

(dayum my first time writing a normal readme file)
