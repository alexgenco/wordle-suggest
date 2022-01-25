use std::{collections::BinaryHeap, fmt::Display};

use crate::attempt::Attempt;

mod weights {
    include!(concat!(env!("OUT_DIR"), "/weights.rs"));
}

#[derive(Debug, Ord, Eq, PartialEq)]
struct WeightedWord {
    s: String,
    weight: usize,
}

impl WeightedWord {
    fn new(s: &str, weight: usize) -> Self {
        Self {
            s: s.to_string(),
            weight,
        }
    }
}

impl Into<String> for WeightedWord {
    fn into(self) -> String {
        self.s
    }
}

impl Display for WeightedWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl PartialOrd for WeightedWord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

pub fn filter_words(attempts: &Vec<Attempt>) -> BinaryHeap<impl Into<String> + Display> {
    weights::WEIGHTS
        .iter()
        .filter(|(word, _)| attempts.iter().all(|a| a.matches(word)))
        .map(|(word, weight)| WeightedWord::new(word, *weight))
        .collect()
}
