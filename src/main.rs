use std::{
    collections::BinaryHeap,
    fmt::Display,
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;
use eyre::{eyre, Result};
use parser::parse_attempt;

mod parser;
mod weights;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(short = 'f', long, parse(from_os_str))]
    attempts_file: Option<PathBuf>,

    #[clap(short = 'n', default_value = "10", conflicts_with = "all")]
    number: usize,

    #[clap(short, long, conflicts_with = "number")]
    all: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Attempt(
    CharAttempt,
    CharAttempt,
    CharAttempt,
    CharAttempt,
    CharAttempt,
);

impl Attempt {
    fn matches(&self, word: &'static str) -> bool {
        [&self.0, &self.1, &self.2, &self.3, &self.4]
            .iter()
            .enumerate()
            .all(|(i, ca)| ca.matches(i, word))
    }
}

#[derive(Clone, Debug, PartialEq)]
enum CharAttempt {
    Here(char),
    Elsewhere(char),
    Nowhere(char),
}

impl CharAttempt {
    fn matches(&self, i: usize, word: &'static str) -> bool {
        match self {
            CharAttempt::Here(c) => word.chars().nth(i).unwrap() == *c,
            CharAttempt::Elsewhere(c) => word.contains(*c) && word.chars().nth(i).unwrap() != *c,
            CharAttempt::Nowhere(c) => {
                // This isn't a strict `!word.contains(*c)` because in the case of
                // repeated characters, one of the repeats will be marked `Nowhere`.
                word.chars().nth(i).unwrap() != *c
            }
        }
    }
}

impl WeightedWord {
    fn new(s: &str, weight: usize) -> Self {
        Self {
            s: s.to_string(),
            weight,
        }
    }
}

#[derive(Debug, Ord, Eq, PartialEq)]
struct WeightedWord {
    s: String,
    weight: usize,
}

impl Display for WeightedWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl PartialOrd for WeightedWord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

fn load_words(attempts: &Vec<Attempt>) -> BinaryHeap<WeightedWord> {
    weights::WEIGHTS
        .iter()
        .filter(|(word, _)| attempts.iter().all(|a| a.matches(word)))
        .map(|(word, weight)| WeightedWord::new(word, *weight))
        .collect()
}

fn load_attempts(path: PathBuf) -> Result<Vec<Attempt>> {
    let mut attempts = Vec::new();
    let rd: Box<dyn BufRead>;

    if path.to_string_lossy() == "-" {
        rd = Box::new(BufReader::new(stdin()));
    } else {
        let file = File::open(path)?;
        rd = Box::new(BufReader::new(file));
    }

    for line in rd.lines() {
        let (_, attempt) = parse_attempt(&line?).map_err(|e| eyre!("Parse error: {}", e))?;
        attempts.push(attempt);
    }

    Ok(attempts)
}

fn main() -> Result<()> {
    let Opts {
        attempts_file,
        number,
        all,
    } = Opts::parse();

    let attempts = match attempts_file {
        Some(path) => load_attempts(path)?,
        None => Vec::new(),
    };

    let words = load_words(&attempts);
    let n = if all { words.len() } else { number };

    for word in words.iter().take(n) {
        println!("{}", word);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{Attempt, CharAttempt};

    #[test]
    fn test_attempt_filter() {
        let attempt = Attempt(
            CharAttempt::Nowhere('i'),
            CharAttempt::Elsewhere('c'),
            CharAttempt::Here('i'),
            CharAttempt::Nowhere('n'),
            CharAttempt::Nowhere('g'),
        );

        assert!(attempt.matches("crimp"));
    }
}
