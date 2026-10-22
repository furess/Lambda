pub const COLUMNS: usize = 7;

pub fn grid(chars: &[char]) {
    if chars.is_empty() {
        println!("no symbols");
        return;
    }

    for row in chars.chunks(COLUMNS) {
        let mut line = String::new();
        for ch in row {
            line.push(*ch);
            line.push_str("   ");
        }
        println!("{}", line.trim_end());
    }
}

pub fn uppercase(ch: char) -> char {
    ch.to_uppercase().next().unwrap_or(ch)
}
