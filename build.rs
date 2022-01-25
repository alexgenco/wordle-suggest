use std::{collections::HashMap, env, fs, path::Path};

fn main() {
    let words_str = include_str!("words.txt");
    let mut words = Vec::new();
    let mut char_weights = HashMap::new();

    for word in words_str.lines() {
        words.push(word);

        for c in word.chars() {
            *char_weights.entry(c).or_insert(0) += 1
        }
    }

    let mut rust = format!(
        "pub static WEIGHTS: [(&'static str, usize); {}] = [",
        words.len()
    );

    for word in words {
        let char_weights = word
            .chars()
            .fold(0, |acc, c| acc + char_weights.get(&c).unwrap());
        rust.push_str(&format!("\n    ({:?}, {}),", word, char_weights));
    }

    rust.push_str("\n];");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("weights.rs");

    fs::write(&dest_path, rust).unwrap();
}
