use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;
use eyre::Result;

mod attempt;
mod parser;
mod words;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(short, long, parse(from_os_str))]
    file: Option<PathBuf>,

    #[clap(short, long, default_value = "10", conflicts_with = "all")]
    number: usize,

    #[clap(short, long, conflicts_with = "number")]
    all: bool,
}

fn main() -> Result<()> {
    let Opts { file, number, all } = Opts::parse();

    let attempts = match file {
        Some(path) => parser::parse_reader(input_reader(path)?)?,
        None => Vec::new(),
    };

    let limit = if all { None } else { Some(number) };

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
