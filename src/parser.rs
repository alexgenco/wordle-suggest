use std::io::BufRead;

use anyhow::Result;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, map_res},
    multi::count,
    sequence::preceded,
    IResult,
};
use wordle_suggest::{CharGuess, Guess};

fn parse_line<'a>(line: &'a str) -> IResult<&'a str, Guess> {
    map_res(
        count(
            alt((
                map(preceded(tag("^"), anychar), CharGuess::Here),
                map(preceded(tag("?"), anychar), CharGuess::Elsewhere),
                map(anychar, CharGuess::Nowhere),
            )),
            5,
        ),
        |cas| cas.try_into(),
    )(line)
}

pub fn parse_reader<'a>(rd: Box<dyn BufRead>) -> Result<Vec<Guess>> {
    let mut guesses = Vec::new();

    for line in rd.lines() {
        let (_, guess) = parse_line(&line?).map_err(|e| e.to_owned())?;
        guesses.push(guess);
    }

    Ok(guesses)
}

#[cfg(test)]
mod test {
    use std::io::{BufRead, BufReader};

    use super::{parse_reader, CharGuess};

    #[test]
    fn test_parse_reader_empty() {
        let guesses = parse_reader(rd("")).unwrap();
        assert!(guesses.is_empty());
    }

    #[test]
    fn test_parse_reader_ok() {
        let guesses = parse_reader(rd("^boa?ts\ns^a?les\n")).unwrap();

        assert_eq!(
            guesses,
            vec![
                [
                    CharGuess::Here('b'),
                    CharGuess::Nowhere('o'),
                    CharGuess::Nowhere('a'),
                    CharGuess::Elsewhere('t'),
                    CharGuess::Nowhere('s'),
                ],
                [
                    CharGuess::Nowhere('s'),
                    CharGuess::Here('a'),
                    CharGuess::Elsewhere('l'),
                    CharGuess::Nowhere('e'),
                    CharGuess::Nowhere('s'),
                ]
            ],
        );
    }

    #[test]
    fn test_parse_reader_blank_line() {
        let guesses = parse_reader(rd("^boa?ts\n\n")).unwrap();

        assert_eq!(
            guesses,
            vec![[
                CharGuess::Here('b'),
                CharGuess::Nowhere('o'),
                CharGuess::Nowhere('a'),
                CharGuess::Elsewhere('t'),
                CharGuess::Nowhere('s'),
            ],]
        );
    }

    #[test]
    fn test_parse_reader_incomplete_line() {
        todo!()
    }

    #[test]
    fn test_parse_reader_invalid_character() {
        todo!()
    }

    fn rd(content: &'static str) -> Box<dyn BufRead> {
        let rd = BufReader::new(content.as_bytes());
        Box::new(rd)
    }
}
