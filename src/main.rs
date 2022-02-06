use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Result;
use clap::{ColorChoice, Parser};
use rand::{rngs::StdRng, SeedableRng};
use wordle_suggest::Hint;

mod parser;

#[derive(Debug, Parser)]
#[clap(about = "Get word suggestions for Wordle", color = ColorChoice::Never)]
struct Opts {
    #[clap(
        short,
        long,
        parse(from_os_str),
        display_order = 0,
        help = "Path to hints file, or `-` for STDIN"
    )]
    file: Option<PathBuf>,

    #[clap(
        short = 'H',
        long = "hint",
        value_name = "HINT",
        parse(try_from_str = parser::try_from_str),
        multiple_occurrences = true,
        display_order = 1,
        help = "Specify a single hint (additive)"
    )]
    hints: Vec<Hint>,

    #[clap(
        short = 'n',
        long,
        default_value = "10",
        conflicts_with = "all",
        display_order = 2,
        help = "Limit the number of words returned"
    )]
    limit: usize,

    #[clap(
        short,
        long,
        conflicts_with = "limit",
        display_order = 3,
        help = "Do not limit the number of words returned"
    )]
    all: bool,

    #[clap(
        short,
        long,
        display_order = 4,
        default_missing_value = "true",
        help = "Exclude words with repeated characters"
    )]
    unique: Option<bool>,

    #[clap(
        short,
        long,
        display_order = 5,
        value_name = "SEED",
        help = "Randomize suggestions with optional seed"
    )]
    random: Option<Option<u64>>,
}

fn main() -> Result<()> {
    let Opts {
        file,
        hints,
        limit,
        all,
        unique,
        random,
    } = Opts::parse();

    let hints = [
        match file {
            Some(path) => parser::try_from_reader(input_reader(path)?)?,
            None => Vec::new(),
        },
        hints,
    ]
    .concat();

    let first_guess = hints.is_empty();
    let unique = unique.unwrap_or(first_guess);
    let limit = if all { None } else { Some(limit) };

    let rng = match random {
        Some(Some(seed)) => Some(StdRng::seed_from_u64(seed)),
        Some(None) => Some(StdRng::from_entropy()),
        None => None,
    };

    for word in wordle_suggest::suggestions(&hints, unique, rng, limit) {
        println!("{}", word);
    }

    Ok(())
}

fn input_reader(path: PathBuf) -> Result<Box<dyn BufRead>> {
    if path.to_string_lossy() == "-" {
        Ok(Box::new(BufReader::new(stdin())))
    } else {
        let file = File::open(path)?;
        Ok(Box::new(BufReader::new(file)))
    }
}
