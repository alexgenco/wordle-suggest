use std::collections::HashSet;

pub type Attempt = [CharAttempt; 5];
pub type Word = [char; 5];

#[derive(Clone, Debug, PartialEq)]
pub enum CharAttempt {
    Here(char),
    Elsewhere(char),
    Nowhere(char),
}

#[derive(Clone, Debug, PartialEq, clap::ArgEnum)]
#[clap(rename_all = "lower")]
pub enum Rule {
    #[clap(aliases(["norepeats", "nr"]))]
    Unique,
}

impl Rule {
    pub fn defaults(rules: Vec<Self>, nattempts: usize) -> Vec<Self> {
        // On first attempt, default to no repeating characters
        if nattempts == 0 && rules.is_empty() {
            vec![Rule::Unique]
        } else {
            rules
        }
    }
}

pub fn match_attempt(attempt: &Attempt, chars: [char; 5]) -> bool {
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

pub fn match_rule(rule: &Rule, chars: [char; 5]) -> bool {
    match rule {
        Rule::Unique => chars.into_iter().collect::<HashSet<char>>().len() == chars.len(),
    }
}

#[cfg(test)]
mod test {
    use crate::{Rule, match_rule};

    use super::{match_attempt, CharAttempt};

    #[test]
    fn test_attempt_matches() {
        let attempt = [
            CharAttempt::Nowhere('i'),
            CharAttempt::Elsewhere('c'),
            CharAttempt::Here('i'),
            CharAttempt::Nowhere('n'),
            CharAttempt::Nowhere('g'),
        ];

        assert!(match_attempt(&attempt, ['c', 'r', 'i', 'm', 'p']));
        assert!(!match_attempt(&attempt, ['c', 'r', 'u', 's', 't']));
    }

    #[test]
    fn test_rule_matches() {
        assert!(match_rule(&Rule::Unique, ['c', 'r', 'i', 'm', 'p']));
        assert!(!match_rule(&Rule::Unique, ['c', 'r', 'i', 'c', 'k']));
    }
}
