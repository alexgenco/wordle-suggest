use std::{collections::BinaryHeap, iter};

use crate::attempt::Attempt;

mod weights {
    include!(concat!(env!("OUT_DIR"), "/weights.rs"));
}

#[derive(Debug, Ord, Eq, PartialEq)]
struct Word {
    s: String,
    weight: usize,
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Into<String> for Word {
    fn into(self) -> String {
        self.s
    }
}

pub fn filtered_words(
    attempts: &Vec<Attempt>,
    limit: Option<usize>,
) -> Box<dyn Iterator<Item = String>> {
    let mut heap: BinaryHeap<Word> = weights::WEIGHTS
        .into_iter()
        .filter_map(|(word, weight)| {
            if attempts.iter().all(|a| a.matches(word)) {
                Some(Word {
                    s: word.to_string(),
                    weight,
                })
            } else {
                None
            }
        })
        .collect();

    let it = iter::from_fn(move || heap.pop().map(Into::into));

    match limit {
        Some(n) => Box::new(it.take(n)),
        None => Box::new(it),
    }
}
