use std::collections::HashSet;

pub type Guess = [CharGuess; 5];
pub type Word = [char; 5];

#[derive(Clone, Debug, PartialEq)]
pub enum CharGuess {
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

pub fn default_rules(specified: Vec<Rule>, nguesses: usize) -> Vec<Rule> {
    // On first guess, default to no repeating characters
    if nguesses == 0 && specified.is_empty() {
        vec![Rule::Unique]
    } else {
        specified
    }
}

pub fn match_guess(guess: &Guess, word: Word) -> bool {
    guess.iter().enumerate().all(|(i, ca)| match ca {
        CharGuess::Here(c) => word[i] == *c,
        CharGuess::Elsewhere(c) => word.contains(c) && word[i] != *c,
        CharGuess::Nowhere(c) => {
            // This isn't a strict `!word.contains(*c)` because in the case of repeated
            // characters, one of the repeats can be marked `Nowhere` if the other is marked
            // `Elsewhere`.
            word[i] != *c
        }
    })
}

pub fn match_rule(rule: &Rule, word: Word) -> bool {
    match rule {
        Rule::Unique => word.into_iter().collect::<HashSet<char>>().len() == word.len(),
    }
}

#[cfg(test)]
mod test {
    use crate::{match_rule, Rule};

    use super::{match_guess, CharGuess};

    #[test]
    fn test_guess_matches() {
        let guess = [
            CharGuess::Nowhere('i'),
            CharGuess::Elsewhere('c'),
            CharGuess::Here('i'),
            CharGuess::Nowhere('n'),
            CharGuess::Nowhere('g'),
        ];

        assert!(match_guess(&guess, ['c', 'r', 'i', 'm', 'p']));
        assert!(!match_guess(&guess, ['c', 'r', 'u', 's', 't']));
    }

    #[test]
    fn test_rule_matches() {
        assert!(match_rule(&Rule::Unique, ['c', 'r', 'i', 'm', 'p']));
        assert!(!match_rule(&Rule::Unique, ['c', 'r', 'i', 'c', 'k']));
    }
}
