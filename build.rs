use std::{collections::HashMap, env, fs, path::Path};

fn main() {
    let words_str = include_str!("words.txt");
    let mut words = Vec::new();
    let mut char_weights = HashMap::new();

    for word in words_str.lines() {
        words.push(word);

        for (i, c) in word.chars().enumerate() {
            char_weights.entry(c).or_insert([0; 5])[i] += 1;
        }
    }

    let mut rust = format!(
        "pub static WEIGHTS: [(&'static str, usize); {}] = [",
        words.len()
    );

    for word in words {
        let weight = word.chars().enumerate().fold(0, |acc, (i, c)| {
            acc + char_weights.get(&c).map(|arr| arr[i]).unwrap_or(0)
        });

        rust.push_str(&format!("\n    ({:?}, {}),", word, weight));
    }

    rust.push_str("\n];");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("weights.rs");

    fs::write(&dest_path, rust).unwrap();
}
