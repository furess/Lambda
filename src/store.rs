use std::env;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomSymbol {
    pub ch: char,
    pub keyword: String,
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn config_dir() -> PathBuf {
    if let Some(base) = env::var_os("XDG_CONFIG_HOME").filter(|value| !value.is_empty()) {
        return PathBuf::from(base).join("lambda");
    }

    if let Some(home) = env::var_os("HOME").filter(|value| !value.is_empty()) {
        return PathBuf::from(home).join(".config").join("lambda");
    }

    PathBuf::from(".lambda")
}

pub fn path() -> PathBuf {
    config_dir().join("symbols.json")
}

pub fn load() -> Vec<CustomSymbol> {
    let path = path();
    let Ok(text) = fs::read_to_string(path) else {
        return Vec::new();
    };

    if text.trim().is_empty() {
        return Vec::new();
    }

    serde_json::from_str(&text).unwrap_or_default()
}

fn save(symbols: &[CustomSymbol]) -> Result<()> {
    let path = path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let text = serde_json::to_string_pretty(symbols)?;
    fs::write(path, text)?;
    Ok(())
}

/// Returns `true` when an existing keyword was replaced.
pub fn add(ch: char, keyword: &str) -> Result<bool> {
    let keyword = keyword.trim();
    if keyword.is_empty() {
        return Err("keyword cannot be empty".into());
    }

    let mut symbols = load();
    let replaced = if let Some(existing) = symbols
        .iter_mut()
        .find(|symbol| symbol.keyword.eq_ignore_ascii_case(keyword))
    {
        existing.ch = ch;
        true
    } else {
        symbols.push(CustomSymbol {
            ch,
            keyword: keyword.to_string(),
        });
        false
    };

    save(&symbols)?;
    Ok(replaced)
}

pub fn remove(keyword: &str) -> Result<bool> {
    let keyword = keyword.trim();
    let mut symbols = load();
    let before = symbols.len();
    symbols.retain(|symbol| !symbol.keyword.eq_ignore_ascii_case(keyword));

    if symbols.len() == before {
        return Ok(false);
    }

    save(&symbols)?;
    Ok(true)
}
