use std::{
    collections::{BinaryHeap, HashSet},
    iter,
};

mod weights {
    include!(concat!(env!("OUT_DIR"), "/weights.rs"));
}

pub type Guess = [CharGuess; 5];
pub type Word = [char; 5];

#[derive(Clone, Debug, PartialEq)]
pub enum CharGuess {
    Here(char),
    Elsewhere(char),
    Nowhere(char),
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
    unique: bool,
    singular: bool,
    limit: Option<usize>,
) -> impl Iterator<Item = String> {
    let mut heap: BinaryHeap<WeightedWord> = weights::WEIGHTS
        .into_iter()
        .filter_map(|(word, weight)| {
            if satisfies_guesses(word, guesses)
                && satisfies_uniqueness(word, unique)
                && satisfies_singular(word, singular)
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

fn satisfies_guesses(word: Word, guesses: &Vec<Guess>) -> bool {
    guesses.iter().all(|guess| {
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
    })
}

fn satisfies_uniqueness(word: Word, unique: bool) -> bool {
    if unique {
        HashSet::<char>::from_iter(word).len() == word.len()
    } else {
        true
    }
}

fn satisfies_singular(word: Word, singular: bool) -> bool {
    if singular {
        word[4] != 's'
    } else {
        true
    }
}
