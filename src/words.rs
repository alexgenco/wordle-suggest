use std::{collections::BinaryHeap, iter};

use wordle_suggest::{match_attempt, Attempt, Rule, match_rule};

mod weights {
    include!(concat!(env!("OUT_DIR"), "/weights.rs"));
}

#[derive(Debug, Ord, Eq, PartialEq)]
struct WeightedWord {
    chars: [char; 5],
    weight: usize,
}

impl PartialOrd for WeightedWord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Into<String> for WeightedWord {
    fn into(self) -> String {
        String::from_iter(self.chars)
    }
}

pub fn filtered_words(
    attempts: &Vec<Attempt>,
    rules: &Vec<Rule>,
    limit: Option<usize>,
) -> impl Iterator<Item = String> {
    let mut heap: BinaryHeap<WeightedWord> = weights::WEIGHTS
        .into_iter()
        .filter_map(|(chars, weight)| {
            if attempts.iter().all(|a| match_attempt(a, chars))
                && rules.iter().all(|r| match_rule(r, chars))
            {
                Some(WeightedWord { chars, weight })
            } else {
                None
            }
        })
        .collect();

    let limit = limit.unwrap_or_else(|| heap.len());

    iter::from_fn(move || heap.pop().map(Into::into)).take(limit)
}
