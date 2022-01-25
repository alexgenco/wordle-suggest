use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    println!("cargo:rerun-if-changed=words.txt");

    let mut words = Vec::new();
    let mut char_weights = HashMap::new();

    let words_file = File::open("words.txt").unwrap();
    let rd = BufReader::new(words_file);

    for line in rd.lines() {
        let word = line.unwrap();

        for (i, c) in word.chars().enumerate() {
            char_weights.entry(c).or_insert([0; 5])[i] += 1;
        }

        words.push(word);
    }

    let mut rust = format!(
        "pub static WEIGHTS: [([char; 5], usize); {}] = [",
        words.len()
    );

    for word in words {
        let weight = word.chars().enumerate().fold(0, |acc, (i, c)| {
            acc + char_weights.get(&c).map(|arr| arr[i]).unwrap_or(0)
        });

        let charstr = word
            .chars()
            .map(|c| format!("'{}'", c))
            .collect::<Vec<String>>()
            .join(", ");

        rust.push_str(&format!("\n    ([{}], {}),", charstr, weight));
    }

    rust.push_str("\n];");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("weights.rs");

    fs::write(&dest_path, rust).unwrap();
}
