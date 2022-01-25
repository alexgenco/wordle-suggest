use std::collections::BinaryHeap;

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

pub struct FilteredWords {
    heap: BinaryHeap<Word>,
    limit: Option<usize>,
    taken: usize,
}

impl Iterator for FilteredWords {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self {
                limit: Some(n),
                taken,
                ..
            } if *taken >= *n => None,
            Self { heap, taken, .. } => match heap.pop() {
                Some(word) => {
                    *taken += 1;
                    Some(word.s)
                }
                None => None,
            },
        }
    }
}

pub fn filtered_words(attempts: &Vec<Attempt>, limit: Option<usize>) -> FilteredWords {
    let heap = weights::WEIGHTS
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

    FilteredWords {
        heap,
        limit,
        taken: 0,
    }
}
