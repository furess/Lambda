fn main() {
    let symbols = Lambda::data::load();
    for s in &symbols {
        println!("{}  {}  [{}]", s.ch, s.name, s.category);
    }
}
