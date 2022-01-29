use std::{collections::BinaryHeap, iter};

use wordle_suggest::{match_guess, match_rule, Guess, Rule, Word};

mod weights {
    include!(concat!(env!("OUT_DIR"), "/weights.rs"));
}

#[derive(Debug, Ord, Eq, PartialEq)]
struct WeightedWord {
    word: Word,
    weight: usize,
}

impl PartialOrd for WeightedWord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Into<String> for WeightedWord {
    fn into(self) -> String {
        String::from_iter(self.word)
    }
}

pub fn filtered_words(
    guesses: &Vec<Guess>,
    rules: &Vec<Rule>,
    limit: Option<usize>,
) -> impl Iterator<Item = String> {
    let mut heap: BinaryHeap<WeightedWord> = weights::WEIGHTS
        .into_iter()
        .filter_map(|(word, weight)| {
            if guesses.iter().all(|a| match_guess(a, word))
                && rules.iter().all(|r| match_rule(r, word))
            {
                Some(WeightedWord { word, weight })
            } else {
                None
            }
        })
        .collect();

    let limit = limit.unwrap_or_else(|| heap.len());

    iter::from_fn(move || heap.pop().map(Into::into)).take(limit)
}
