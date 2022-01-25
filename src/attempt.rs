#[derive(Clone, Debug, PartialEq)]
pub struct Attempt(
    pub CharAttempt,
    pub CharAttempt,
    pub CharAttempt,
    pub CharAttempt,
    pub CharAttempt,
);

impl Attempt {
    pub fn matches(&self, word: &'static str) -> bool {
        let chars: Vec<char> = word.chars().collect();

        [&self.0, &self.1, &self.2, &self.3, &self.4]
            .iter()
            .enumerate()
            .all(|(i, ca)| ca.matches(i, &chars))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CharAttempt {
    Here(char),
    Elsewhere(char),
    Nowhere(char),
}

impl CharAttempt {
    fn matches(&self, i: usize, chars: &Vec<char>) -> bool {
        match self {
            CharAttempt::Here(c) => chars[i] == *c,
            CharAttempt::Elsewhere(c) => chars.contains(c) && chars[i] != *c,
            CharAttempt::Nowhere(c) => {
                // This isn't a strict `!word.contains(*c)` because in the case of repeated
                // characters, one of the repeats can be marked `Nowhere` if the other is marked
                // `Elsewhere`.
                chars[i] != *c
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Attempt, CharAttempt};

    #[test]
    fn test_attempt_matches() {
        let attempt = Attempt(
            CharAttempt::Nowhere('i'),
            CharAttempt::Elsewhere('c'),
            CharAttempt::Here('i'),
            CharAttempt::Nowhere('n'),
            CharAttempt::Nowhere('g'),
        );

        assert!(attempt.matches("crimp"));
    }
}
