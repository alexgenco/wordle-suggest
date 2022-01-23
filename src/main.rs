use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;
use eyre::{eyre, Result};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::anychar, combinator::map,
    sequence::tuple, IResult,
};
use rand::{prelude::SmallRng, seq::SliceRandom, SeedableRng};

#[derive(Debug, Parser)]
struct Opts {
    #[clap(
        short,
        long,
        parse(from_os_str),
        default_value = "/usr/share/dict/words"
    )]
    words_file: PathBuf,

    #[clap(short, long, parse(from_os_str))]
    attempts_file: Option<PathBuf>,

    #[clap(short = 'n', default_value = "3")]
    nsuggestions: usize,

    #[clap(short, long)]
    seed: Option<u64>,
}

#[derive(Clone, Debug, PartialEq)]
struct Attempt(
    CharAttempt,
    CharAttempt,
    CharAttempt,
    CharAttempt,
    CharAttempt,
);

impl Attempt {
    fn matches(&self, word: &String) -> bool {
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
    fn matches(&self, i: usize, word: &String) -> bool {
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

fn parse_char_attempt<'a>(input: &'a str) -> IResult<&'a str, CharAttempt> {
    map(
        alt((
            tuple((tag("^"), anychar)),
            tuple((tag("?"), anychar)),
            tuple((tag(""), anychar)),
        )),
        |(tag, ch)| match tag {
            "^" => CharAttempt::Here(ch),
            "?" => CharAttempt::Elsewhere(ch),
            "" => CharAttempt::Nowhere(ch),
            _ => unreachable!(),
        },
    )(input)
}

fn parse_attempt<'a>(input: &'a str) -> IResult<&'a str, Attempt> {
    map(
        tuple((
            parse_char_attempt,
            parse_char_attempt,
            parse_char_attempt,
            parse_char_attempt,
            parse_char_attempt,
        )),
        |(ca0, ca1, ca2, ca3, ca4)| Attempt(ca0, ca1, ca2, ca3, ca4),
    )(input)
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

fn load_words(path: PathBuf, attempts: &Vec<Attempt>) -> Result<Vec<String>> {
    let mut words = Vec::new();
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();

    for line in lines {
        match line {
            Ok(word)
                if word.len() == 5
                    && word.starts_with(|ch: char| ch.is_ascii_lowercase())
                    && attempts.iter().all(|attempt| attempt.matches(&word)) =>
            {
                words.push(word);
            }
            _ => {}
        }
    }

    Ok(words)
}

fn pick_suggestions(words: &Vec<String>, nsuggestions: usize, seed: Option<u64>) -> Vec<String> {
    let mut rng = seed
        .map(SmallRng::seed_from_u64)
        .unwrap_or_else(SmallRng::from_entropy);
    words
        .choose_multiple(&mut rng, nsuggestions)
        .cloned()
        .collect()
}

fn main() -> Result<()> {
    let Opts {
        words_file,
        attempts_file,
        nsuggestions,
        seed,
    } = Opts::parse();

    let attempts = match attempts_file {
        Some(path) => load_attempts(path)?,
        None => Vec::new(),
    };

    let words = load_words(words_file, &attempts)?;
    let suggestions = pick_suggestions(&words, nsuggestions, seed);

    for suggestion in suggestions {
        println!("{}", suggestion);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{parse_attempt, Attempt, CharAttempt};

    #[test]
    fn test_parse_attempt() {
        match parse_attempt("^boa?ts") {
            Ok((rest, attempts)) => {
                assert_eq!(
                    attempts,
                    Attempt(
                        CharAttempt::Here('b'),
                        CharAttempt::Nowhere('o'),
                        CharAttempt::Nowhere('a'),
                        CharAttempt::Elsewhere('t'),
                        CharAttempt::Nowhere('s'),
                    )
                );

                assert_eq!("", rest);
            }
            e => panic!("{:?}", e),
        }
    }

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