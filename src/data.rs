use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Symbol {
    pub ch: char,
    pub name: String,
    pub category: String,
}

pub fn load() -> Vec<Symbol> {
    serde_json::from_str(include_str!("../data/symbols.json"))
        .expect("data/symbols.json is invalid")
}
