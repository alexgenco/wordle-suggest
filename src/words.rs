use std::{collections::BinaryHeap, iter};

use crate::attempt::{matches, Attempt};

mod weights {
    include!(concat!(env!("OUT_DIR"), "/weights.rs"));
}

#[derive(Debug, Ord, Eq, PartialEq)]
struct Word {
    chars: [char; 5],
    weight: usize,
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Into<String> for Word {
    fn into(self) -> String {
        String::from_iter(self.chars)
    }
}

pub fn filtered_words(attempts: &Vec<Attempt>, n: Option<usize>) -> impl Iterator<Item = String> {
    let mut heap: BinaryHeap<Word> = weights::WEIGHTS
        .into_iter()
        .filter_map(|(chars, weight)| {
            if attempts.iter().all(|a| matches(a, chars)) {
                Some(Word { chars, weight })
            } else {
                None
            }
        })
        .collect();

    let limit = n.unwrap_or_else(|| heap.len());

    iter::from_fn(move || heap.pop().map(Into::into)).take(limit)
}
