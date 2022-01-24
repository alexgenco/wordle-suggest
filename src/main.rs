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
use rand::{prelude::SmallRng, Rng, SeedableRng};

mod parser;
mod words;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(short = 'f', long, parse(from_os_str))]
    attempts_file: Option<PathBuf>,

    #[clap(short = 'n', default_value = "10", conflicts_with = "all")]
    number: usize,

    #[clap(short, long, conflicts_with = "number")]
    all: bool,

    #[clap(short, long)]
    seed: Option<u64>,
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
    fn matches(&self, word: &Word) -> bool {
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
    fn matches(&self, i: usize, word: &Word) -> bool {
        let word = &word.s;

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

#[derive(Debug, Ord, Eq, PartialEq)]
struct Word {
    s: String,
    score: usize,
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Word {
    fn new(s: &str, rng: &mut SmallRng) -> Self {
        let mut score = rng.gen_range(0..10);

        // Most common letters
        for c in ['e', 't', 'a', 'i', 'o', 'n', 's', 'h', 'r'] {
            if s.contains(c) {
                score += 1
            }
        }

        // Most common starting letters
        for c in ['t', 'a', 'o', 'd', 'w'] {
            if s.starts_with(c) {
                score += 1
            }
        }

        // Most common ending letters
        for c in ['e', 's', 'd', 't'] {
            if s.ends_with(c) {
                score += 1
            }
        }

        Self {
            s: s.to_string(),
            score,
        }
    }
}

fn load_words(attempts: &Vec<Attempt>, rng: &mut SmallRng) -> Result<BinaryHeap<Word>> {
    let mut words = BinaryHeap::new();

    for s in words::iter_words() {
        let word = Word::new(s, rng);

        if attempts.iter().all(|a| a.matches(&word)) {
            words.push(word);
        }
    }

    Ok(words)
}

fn pick_suggestions(words: BinaryHeap<Word>, n: Option<usize>) -> BinaryHeap<Word> {
    match n {
        Some(n) => words.into_iter().take(n).collect(),
        None => words,
    }
}

fn main() -> Result<()> {
    let Opts {
        attempts_file,
        number: nsuggestions,
        all,
        seed,
    } = Opts::parse();

    let attempts = match attempts_file {
        Some(path) => load_attempts(path)?,
        None => Vec::new(),
    };

    let n = if all { None } else { Some(nsuggestions) };
    let mut rng = match seed {
        Some(s) => SmallRng::seed_from_u64(s),
        None => SmallRng::from_entropy(),
    };

    let words = load_words(&attempts, &mut rng)?;
    let suggestions = pick_suggestions(words, n);

    for suggestion in suggestions {
        println!("{}", suggestion);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::Word;

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

        let word = Word {
            s: "crimp".to_string(),
            score: 0,
        };

        assert!(attempt.matches(&word));
    }
}
