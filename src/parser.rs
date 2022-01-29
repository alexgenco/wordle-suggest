use std::io::BufRead;

use anyhow::{anyhow, Result};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::satisfy,
    combinator::{eof, map, map_res},
    multi::count,
    sequence::terminated,
    IResult,
};
use wordle_suggest::{CharGuess, Guess};

pub fn parse_reader<'a>(rd: Box<dyn BufRead>) -> Result<Vec<Guess>> {
    let mut guesses = Vec::new();

    for (i, line) in rd.lines().enumerate() {
        let line = line?.trim().to_string();

        if line.is_empty() {
            continue;
        }

        let (_, guess) =
            parse_line(&line).map_err(|_| anyhow!("Parse error on line {}: {:?}", i + 1, line))?;

        guesses.push(guess);
    }

    Ok(guesses)
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Guess> {
    terminated(map_res(count(parse_char, 5), |cas| cas.try_into()), eof)(input)
}

fn parse_char<'a>(input: &'a str) -> IResult<&'a str, CharGuess> {
    alt((
        map(terminated(any_alpha, tag("^")), CharGuess::Here),
        map(terminated(any_alpha, tag("?")), CharGuess::Elsewhere),
        map(any_alpha, CharGuess::Nowhere),
    ))(input)
}

fn any_alpha<'a>(input: &'a str) -> IResult<&'a str, char> {
    satisfy(|c| c.is_alphabetic())(input)
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
        let guesses = parse_reader(rd("b^oat?s\nsa^l?es\n")).unwrap();

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
    fn test_parse_reader_no_newline() {
        let guesses = parse_reader(rd("b^oat?s")).unwrap();

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
    fn test_parse_reader_blank_line() {
        let guesses = parse_reader(rd("b^oat?s\n\n")).unwrap();

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
        let error = parse_reader(rd("b^oat?\n")).unwrap_err();

        assert_eq!(error.to_string(), "Parse error on line 1: \"b^oat?\"");
    }

    #[test]
    fn test_parse_reader_too_long_line() {
        let error = parse_reader(rd("b^oat?sx\n")).unwrap_err();

        assert_eq!(error.to_string(), "Parse error on line 1: \"b^oat?sx\"");
    }

    #[test]
    fn test_parse_reader_invalid_character() {
        let error = parse_reader(rd("b^oat!s\n")).unwrap_err();

        assert_eq!(error.to_string(), "Parse error on line 1: \"b^oat!s\"");
    }

    fn rd(content: &'static str) -> Box<dyn BufRead> {
        let rd = BufReader::new(content.as_bytes());
        Box::new(rd)
    }
}
