use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;

mod attempt;
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
        help = "Path to attempts file. Pass `-` for STDIN."
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
}

fn main() -> Result<()> {
    let Opts { file, limit, all } = Opts::parse();

    let attempts = match file {
        Some(path) => parser::parse_reader(input_reader(path)?)?,
        None => Vec::new(),
    };

    let limit = if all { None } else { Some(limit) };

    for word in words::filtered_words(&attempts, limit) {
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
