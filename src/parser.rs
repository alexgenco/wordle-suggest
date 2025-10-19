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
use wordle_suggest::{CharHint, Hint, MARK_ELSEWHERE, MARK_HERE};

pub fn try_from_reader(rd: Box<dyn BufRead>) -> Result<Vec<Hint>> {
    let mut hints = Vec::new();

    for (i, line) in rd.lines().enumerate() {
        let line = line?.trim().to_string();

        if line.is_empty() {
            continue;
        }

        let (_, hint) = parse_line(&line)
            .map_err(|_| anyhow!("Invalid hint syntax on line {}: {:?}", i + 1, line))?;

        hints.push(hint);
    }

    Ok(hints)
}

pub fn try_from_str(input: &str) -> Result<Hint> {
    let (_, hint) = parse_line(input).map_err(|_| anyhow!("Invalid hint syntax: {:?}", input))?;

    Ok(hint)
}

fn parse_line(input: &str) -> IResult<&str, Hint> {
    terminated(map_res(count(parse_char, 5), |cas| cas.try_into()), eof)(input)
}

fn parse_char(input: &str) -> IResult<&str, CharHint> {
    alt((
        map(terminated(any_alpha, tag(MARK_HERE)), CharHint::Here),
        map(terminated(any_alpha, tag(MARK_ELSEWHERE)), CharHint::Elsewhere),
        map(any_alpha, CharHint::Nowhere),
    ))(input)
}

fn any_alpha(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_alphabetic())(input)
}

#[cfg(test)]
mod test {
    use std::io::{BufRead, BufReader};

    use super::{try_from_reader, CharHint};

    #[test]
    fn test_parse_reader_empty() {
        let hints = try_from_reader(rd("")).unwrap();
        assert!(hints.is_empty());
    }

    #[test]
    fn test_parse_reader_ok() {
        let hints = try_from_reader(rd("b^oat~s\nsa^l~es\n")).unwrap();

        assert_eq!(
            hints,
            vec![
                [
                    CharHint::Here('b'),
                    CharHint::Nowhere('o'),
                    CharHint::Nowhere('a'),
                    CharHint::Elsewhere('t'),
                    CharHint::Nowhere('s'),
                ],
                [
                    CharHint::Nowhere('s'),
                    CharHint::Here('a'),
                    CharHint::Elsewhere('l'),
                    CharHint::Nowhere('e'),
                    CharHint::Nowhere('s'),
                ]
            ],
        );
    }

    #[test]
    fn test_parse_reader_no_newline() {
        let hints = try_from_reader(rd("b^oat~s")).unwrap();

        assert_eq!(
            hints,
            vec![[
                CharHint::Here('b'),
                CharHint::Nowhere('o'),
                CharHint::Nowhere('a'),
                CharHint::Elsewhere('t'),
                CharHint::Nowhere('s'),
            ],]
        );
    }

    #[test]
    fn test_parse_reader_blank_line() {
        let hints = try_from_reader(rd("b^oat~s\n\n")).unwrap();

        assert_eq!(
            hints,
            vec![[
                CharHint::Here('b'),
                CharHint::Nowhere('o'),
                CharHint::Nowhere('a'),
                CharHint::Elsewhere('t'),
                CharHint::Nowhere('s'),
            ],]
        );
    }

    #[test]
    fn test_parse_reader_incomplete_line() {
        let error = try_from_reader(rd("b^oat~\n")).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Invalid hint syntax on line 1: \"b^oat~\""
        );
    }

    #[test]
    fn test_parse_reader_too_long_line() {
        let error = try_from_reader(rd("b^oat~sx\n")).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Invalid hint syntax on line 1: \"b^oat~sx\""
        );
    }

    #[test]
    fn test_parse_reader_invalid_character() {
        let error = try_from_reader(rd("b^oat!s\n")).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Invalid hint syntax on line 1: \"b^oat!s\""
        );
    }

    fn rd(content: &'static str) -> Box<dyn BufRead> {
        let rd = BufReader::new(content.as_bytes());
        Box::new(rd)
    }
}
