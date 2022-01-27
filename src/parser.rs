use std::io::BufRead;

use anyhow::{bail, Context, Result};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::anychar, combinator::map, multi::count,
    sequence::preceded, IResult,
};

use crate::attempt::{CharAttempt, Attempt};

fn parse_line<'a>(line: &'a str) -> IResult<&'a str, Attempt> {
    map(
        count(
            alt((
                map(preceded(tag("^"), anychar), CharAttempt::Here),
                map(preceded(tag("?"), anychar), CharAttempt::Elsewhere),
                map(anychar, CharAttempt::Nowhere),
            )),
            5,
        ),
        |cas| cas.try_into().unwrap(),
    )(line)
}

pub fn parse_reader(rd: Box<dyn BufRead>) -> Result<Vec<Attempt>> {
    let mut attempts = Vec::new();

    for (i, line) in rd.lines().enumerate() {
        let line = line.context("Failed to read line")?;

        match parse_line(&line) {
            Ok((_, attempt)) => attempts.push(attempt),
            Err(nom::Err::Incomplete(_)) => bail!("Parse error: EOF on line {}", i + 1),
            Err(nom::Err::Failure(e)) => bail!("Parse error: failure on line {}: {:?}", i + 1, e),
            Err(nom::Err::Error(e)) => eprintln!("[WARN] parsing: {:?}", e),
        }
    }

    Ok(attempts)
}

#[cfg(test)]
mod test {
    use super::{parse_line, CharAttempt};

    #[test]
    fn test_parse_attempt() {
        match parse_line("^boa?ts") {
            Ok((rest, attempts)) => {
                assert_eq!(
                    attempts,
                    [
                        CharAttempt::Here('b'),
                        CharAttempt::Nowhere('o'),
                        CharAttempt::Nowhere('a'),
                        CharAttempt::Elsewhere('t'),
                        CharAttempt::Nowhere('s'),
                    ]
                );

                assert_eq!("", rest);
            }
            e => panic!("{:?}", e),
        }
    }
}
