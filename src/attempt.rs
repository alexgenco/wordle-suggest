pub type Attempt = [CharAttempt; 5];

#[derive(Clone, Debug, PartialEq)]
pub enum CharAttempt {
    Here(char),
    Elsewhere(char),
    Nowhere(char),
}

pub fn matches(attempt: &Attempt, chars: [char; 5]) -> bool {
    attempt.iter().enumerate().all(|(i, ca)| match ca {
        CharAttempt::Here(c) => chars[i] == *c,
        CharAttempt::Elsewhere(c) => chars.contains(c) && chars[i] != *c,
        CharAttempt::Nowhere(c) => {
            // This isn't a strict `!word.contains(*c)` because in the case of repeated
            // characters, one of the repeats can be marked `Nowhere` if the other is marked
            // `Elsewhere`.
            chars[i] != *c
        }
    })
}

#[cfg(test)]
mod test {
    use super::{matches, CharAttempt};

    #[test]
    fn test_attempt_matches() {
        let attempt = [
            CharAttempt::Nowhere('i'),
            CharAttempt::Elsewhere('c'),
            CharAttempt::Here('i'),
            CharAttempt::Nowhere('n'),
            CharAttempt::Nowhere('g'),
        ];

        assert!(matches(&attempt, ['c', 'r', 'i', 'm', 'p']));
        assert!(!matches(&attempt, ['c', 'r', 'u', 's', 't']));
    }
}
