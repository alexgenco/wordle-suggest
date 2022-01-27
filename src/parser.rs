use std::io::BufRead;

use anyhow::{bail, Context, Result};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::anychar, combinator::map, multi::count,
    sequence::preceded, IResult,
};
use wordle_suggest::{CharGuess, Guess};

fn parse_line<'a>(line: &'a str) -> IResult<&'a str, Guess> {
    map(
        count(
            alt((
                map(preceded(tag("^"), anychar), CharGuess::Here),
                map(preceded(tag("?"), anychar), CharGuess::Elsewhere),
                map(anychar, CharGuess::Nowhere),
            )),
            5,
        ),
        |cas| cas.try_into().unwrap(),
    )(line)
}

pub fn parse_reader(rd: Box<dyn BufRead>) -> Result<Vec<Guess>> {
    let mut guesses = Vec::new();

    for (i, line) in rd.lines().enumerate() {
        let line = line.context("Failed to read line")?;

        match parse_line(&line) {
            Ok((_, guess)) => guesses.push(guess),
            Err(nom::Err::Incomplete(_)) => bail!("Parse error: EOF on line {}", i + 1),
            Err(nom::Err::Failure(e)) => bail!("Parse error: failure on line {}: {:?}", i + 1, e),
            Err(nom::Err::Error(e)) => eprintln!("[WARN] parsing: {:?}", e),
        }
    }

    Ok(guesses)
}

#[cfg(test)]
mod test {
    use super::{parse_line, CharGuess};

    #[test]
    fn test_parse_guess() {
        match parse_line("^boa?ts") {
            Ok((rest, guesses)) => {
                assert_eq!(
                    guesses,
                    [
                        CharGuess::Here('b'),
                        CharGuess::Nowhere('o'),
                        CharGuess::Nowhere('a'),
                        CharGuess::Elsewhere('t'),
                        CharGuess::Nowhere('s'),
                    ]
                );

                assert_eq!("", rest);
            }
            e => panic!("{:?}", e),
        }
    }
}
