# lambda

A small CLI tool written in Rust for browsing special Unicode symbols: Greek letters, math, physics and your own symbols. A learning project for practicing Rust.

The built-in symbol tables are compiled into the binary, so no internet connection or external files are needed.

## Usage

```
lambda                          # ASCII lambda + CPU, RAM and GPU info
lambda -g                       # Greek alphabet, 7 symbols per row
lambda -g -Up                   # same, in uppercase (also works as -U)
lambda -m                       # mathematical symbols
lambda -p                       # physical symbols
lambda -c                       # your custom symbols
lambda -a                       # everything
lambda -g -m -Up                # combine categories and flags

lambda list                     # symbols with names and categories
lambda list --category math     # one category only
lambda categories               # available categories and counts
```

Example:

```
$ lambda -g
α   β   γ   δ   ε   ζ   η
θ   ι   κ   λ   μ   ν   ξ
ο   π   ρ   σ   τ   υ   φ
χ   ψ   ω
```

Running `lambda` with no arguments prints a small lambda next to live system information (CPU model, core count, CPU usage, RAM and GPU).

## Adding your own symbols

```
lambda add λ lambda
lambda add √ root
lambda custom
lambda remove root
```

Custom symbols are stored in `~/.config/lambda/symbols.json` (or `$XDG_CONFIG_HOME/lambda/symbols.json`). Adding a keyword that already exists updates its symbol instead of creating a duplicate.

To add built-in symbols, edit the `GREEK`, `MATH` or `PHYSICS` arrays in `src/data.rs`.

## Installation

Requires [Rust](https://rustup.rs) to be installed.

```
git clone https://github.com/furess/Lambda/
cd lambda
cargo install --path .
```
