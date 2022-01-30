use std::{
    collections::{HashMap, HashSet},
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    println!("cargo:rerun-if-changed=words");

    let mut words = Vec::new();
    let mut common_words = HashSet::new();
    let mut char_weights = HashMap::new();

    let common_words_file = File::open("words/common.txt").unwrap();
    let rd = BufReader::new(common_words_file);

    for line in rd.lines() {
        common_words.insert(line.unwrap());
    }

    let words_file = File::open("words/all.txt").unwrap();
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

    let word_count = words.len();

    for word in &words {
        let init = if common_words.contains(word) {
            // Ensure common words are always on top
            word_count
        } else {
            0
        };

        let weight = word.chars().enumerate().fold(init, |acc, (i, c)| {
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
