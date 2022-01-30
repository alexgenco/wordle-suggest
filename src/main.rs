use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;

mod parser;

#[derive(Debug, Parser)]
#[clap(about = "Get word suggestions for Wordle")]
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
        short = 'n',
        long,
        default_value = "10",
        conflicts_with = "all",
        display_order = 1,
        help = "Limit the number of words returned"
    )]
    limit: usize,

    #[clap(
        short,
        long,
        conflicts_with = "limit",
        display_order = 2,
        help = "Do not limit the number of words returned"
    )]
    all: bool,

    #[clap(
        short,
        long,
        display_order = 3,
        default_missing_value = "true",
        help = "Exclude words with repeated characters"
    )]
    unique: Option<bool>,

    #[clap(
        short,
        long,
        display_order = 4,
        default_missing_value = "true",
        help = "Exclude words that end in 's'"
    )]
    singular: Option<bool>,
}

fn main() -> Result<()> {
    let Opts {
        file,
        limit,
        all,
        unique,
        singular,
    } = Opts::parse();

    let hints = match file {
        Some(path) => parser::parse_reader(input_reader(path)?)?,
        None => Vec::new(),
    };

    let first_guess = hints.is_empty();
    let unique = unique.unwrap_or(first_guess);
    let singular = singular.unwrap_or(first_guess);
    let limit = if all { None } else { Some(limit) };

    for word in wordle_suggest::filtered_words(&hints, unique, singular, limit) {
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
