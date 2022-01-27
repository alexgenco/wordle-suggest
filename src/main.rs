use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use wordle_suggest::Rule;

mod parser;
mod words;

#[derive(Debug, Parser)]
#[clap(about = "Get word suggestions for Wordle")]
struct Opts {
    #[clap(
        short,
        long,
        parse(from_os_str),
        display_order = 0,
        help = "Path to guesses file. Pass `-` for STDIN."
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
        arg_enum,
        short,
        long,
        multiple_occurrences = true,
        display_order = 3,
        help = "Additional filtering rules"
    )]
    rules: Vec<Rule>,
}

fn main() -> Result<()> {
    let Opts { file, limit, all, rules } = Opts::parse();

    let guesses = match file {
        Some(path) => parser::parse_reader(input_reader(path)?)?,
        None => Vec::new(),
    };

    let rules = Rule::defaults(rules, guesses.len());
    let limit = if all { None } else { Some(limit) };

    for word in words::filtered_words(&guesses, &rules, limit) {
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
