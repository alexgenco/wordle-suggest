use std::{
    collections::BinaryHeap,
    fmt::Display,
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;
use eyre::{eyre, Result};
use parser::parse_attempt;

mod parser;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(
        short,
        long,
        parse(from_os_str),
        default_value = "/usr/share/dict/words"
    )]
    words_file: PathBuf,

    #[clap(short = 'f', long, parse(from_os_str))]
    attempts_file: Option<PathBuf>,

    #[clap(short = 'n', default_value = "3", conflicts_with = "all")]
    nsuggestions: usize,

    #[clap(short, long, conflicts_with = "nsuggestions")]
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
                // repeated characters, one of the repeats with be marked `Nowhere`.
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

impl FromStr for Word {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 5 && s.starts_with(|ch: char| ch.is_ascii_lowercase()) {
            let mut score = 0;

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

            Ok(Self {
                s: s.to_string(),
                score,
            })
        } else {
            Err(())
        }
    }
}

fn load_words(path: PathBuf, attempts: &Vec<Attempt>) -> Result<BinaryHeap<Word>> {
    let mut words = BinaryHeap::new();
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();

    for line in lines {
        match Word::from_str(&line?) {
            Ok(word) if attempts.iter().all(|a| a.matches(&word)) => {
                words.push(word);
            }
            _ => {}
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
        words_file,
        attempts_file,
        nsuggestions,
        all,
    } = Opts::parse();

    let attempts = match attempts_file {
        Some(path) => load_attempts(path)?,
        None => Vec::new(),
    };

    let n = if all { None } else { Some(nsuggestions) };

    let words = load_words(words_file, &attempts)?;
    let suggestions = pick_suggestions(words, n);

    for suggestion in suggestions {
        println!("{}", suggestion);
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

        assert!(attempt.matches(&"crimp".to_string()));
    }
}
