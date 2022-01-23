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

#[cfg(test)]
mod test {
    use super::{parse_attempt, Attempt, CharAttempt};

    #[test]
    fn test_parse_attempt() {
        match parse_attempt("^boa?ts") {
            Ok((rest, attempts)) => {
                assert_eq!(
                    attempts,
                    Attempt(
                        CharAttempt::Here('b'),
                        CharAttempt::Nowhere('o'),
                        CharAttempt::Nowhere('a'),
                        CharAttempt::Elsewhere('t'),
                        CharAttempt::Nowhere('s'),
                    )
                );

                assert_eq!("", rest);
            }
            e => panic!("{:?}", e),
        }
    }
}
