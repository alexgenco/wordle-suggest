use nom::{
    branch::alt, bytes::complete::tag, character::complete::anychar, combinator::map,
    sequence::tuple, IResult,
};

use crate::{CharAttempt, Attempt};

fn parse_char_attempt<'a>(input: &'a str) -> IResult<&'a str, CharAttempt> {
    map(
        alt((
            tuple((tag("^"), anychar)),
            tuple((tag("?"), anychar)),
            tuple((tag(""), anychar)),
        )),
        |(tag, ch)| match tag {
            "^" => CharAttempt::Here(ch),
            "?" => CharAttempt::Elsewhere(ch),
            "" => CharAttempt::Nowhere(ch),
            _ => unreachable!(),
        },
    )(input)
}

pub fn parse_attempt<'a>(input: &'a str) -> IResult<&'a str, Attempt> {
    map(
        tuple((
            parse_char_attempt,
            parse_char_attempt,
            parse_char_attempt,
            parse_char_attempt,
            parse_char_attempt,
        )),
        |(ca0, ca1, ca2, ca3, ca4)| Attempt(ca0, ca1, ca2, ca3, ca4),
    )(input)
}
