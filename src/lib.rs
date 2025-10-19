use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    iter,
};

use rand::{rngs::StdRng, Rng};

mod weights {
    include!(concat!(env!("OUT_DIR"), "/weights.rs"));
}

pub const MARK_HERE: &str = "^";
pub const MARK_ELSEWHERE: &str = "~";

pub type Hint = [CharHint; 5];
pub type Word = [char; 5];

#[derive(Clone, Debug, PartialEq)]
pub enum CharHint {
    Here(char),
    Elsewhere(char),
    Nowhere(char),
}

#[derive(Debug, Eq, PartialEq)]
struct WeightedWord {
    word: Word,
    weight: usize,
    common: bool,
}

impl Ord for WeightedWord {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.common == other.common {
            self.weight.cmp(&other.weight)
        } else {
            self.common.cmp(&other.common)
        }
    }
}

impl PartialOrd for WeightedWord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<WeightedWord> for String {
    fn from(w: WeightedWord) -> Self {
        String::from_iter(w.word)
    }
}

pub fn suggestions(
    hints: &[Hint],
    unique: bool,
    mut random: Option<StdRng>,
    limit: Option<usize>,
) -> impl Iterator<Item = String> {
    let mut heap: BinaryHeap<WeightedWord> = weights::WEIGHTS
        .into_iter()
        .filter(|(word, _, _)| satisfies_uniqueness(word, unique))
        .filter(|(word, _, _)| satisfies_hints(word, hints))
        .map(|(word, weight, common)| {
            if let Some(rng) = random.as_mut() {
                WeightedWord {
                    word,
                    weight: rng.gen::<usize>(),
                    common: true,
                }
            } else {
                WeightedWord {
                    word,
                    weight,
                    common,
                }
            }
        })
        .collect();

    let limit = limit.unwrap_or(heap.len());

    iter::from_fn(move || heap.pop().map(Into::into)).take(limit)
}

fn satisfies_hints(word: &Word, hints: &[Hint]) -> bool {
    hints.iter().all(|hint| satisfies_hint(word, hint))
}

fn satisfies_hint(word: &Word, hint: &Hint) -> bool {
    let mut counts = HashMap::new();

    hint.iter().enumerate().all(|(i, ch)| match ch {
        CharHint::Here(c) => {
            *counts.entry(c).or_insert(0) += 1;
            word[i] == *c
        }
        CharHint::Elsewhere(c) => {
            *counts.entry(c).or_insert(0) += 1;
            word[i] != *c
        }
        CharHint::Nowhere(_) => true,
    }) && hint.iter().all(|ch| match ch {
        CharHint::Nowhere(c) | CharHint::Elsewhere(c) => {
            let exp = counts.get(c).cloned().unwrap_or(0);
            let act = word.iter().filter(|&wc| *wc == *c).count();

            exp == act
        }
        CharHint::Here(_) => true,
    })
}

fn satisfies_uniqueness(word: &Word, unique: bool) -> bool {
    if unique {
        HashSet::<char>::from_iter(*word).len() == word.len()
    } else {
        true
    }
}

#[cfg(test)]
mod test {
    use crate::{satisfies_hint, CharHint, WeightedWord};

    #[test]
    fn test_weighted_word_ord() {
        let mut lhs = WeightedWord {
            word: ['a', 'b', 'c', 'd', 'e'],
            weight: 0,
            common: false,
        };

        let mut rhs = WeightedWord {
            word: ['a', 'b', 'c', 'd', 'e'],
            weight: 1,
            common: false,
        };

        assert!(lhs < rhs);

        lhs.common = true;
        assert!(lhs > rhs);

        rhs.common = true;
        assert!(lhs < rhs);

        rhs.weight = lhs.weight;
        assert!(lhs == rhs);
    }

    #[test]
    fn test_all_nones() {
        assert!(
            satisfies_hint(
                &['m', 'o', 'n', 'e', 'y'],
                &[
                    CharHint::Nowhere('q'),
                    CharHint::Nowhere('x'),
                    CharHint::Nowhere('p'),
                    CharHint::Nowhere('z'),
                    CharHint::Nowhere('r'),
                ]
            ),
            "All `None`s are satisfied by a word containing none of those letters"
        );
    }

    #[test]
    fn test_all_heres() {
        assert!(
            satisfies_hint(
                &['m', 'o', 'n', 'e', 'y'],
                &[
                    CharHint::Here('m'),
                    CharHint::Here('o'),
                    CharHint::Here('n'),
                    CharHint::Here('e'),
                    CharHint::Here('y'),
                ]
            ),
            "All `Here`s are satisified by the matching word"
        );
    }

    #[test]
    fn test_elsewhere() {
        assert!(
            satisfies_hint(
                &['m', 'o', 'n', 'e', 'y'],
                &[
                    CharHint::Elsewhere('y'),
                    CharHint::Nowhere('x'),
                    CharHint::Nowhere('p'),
                    CharHint::Nowhere('z'),
                    CharHint::Nowhere('r'),
                ]
            ),
            "An `Elsewhere` is satisfied with a letter in a different position"
        );
    }

    #[test]
    fn test_single_none() {
        assert!(
            !satisfies_hint(
                &['a', 'p', 'n', 'i', 'c'],
                &[
                    CharHint::Elsewhere('p'),
                    CharHint::Nowhere('a'), // <-
                    CharHint::Here('n'),
                    CharHint::Here('i'),
                    CharHint::Here('c'),
                ]
            ),
            "A single `None` rejects words containing that letter"
        );
    }

    #[test]
    fn test_repeated_hint_chars() {
        assert!(
            satisfies_hint(
                &['b', 'o', 'a', 't', 's'],
                &[
                    CharHint::Here('b'),
                    CharHint::Elsewhere('a'),
                    CharHint::Nowhere('b'),
                    CharHint::Nowhere('b'),
                    CharHint::Nowhere('y'),
                ]
            ),
            "Repeated hint characters can be marked `None`"
        );
    }

    #[test]
    fn test_here_after_none() {
        assert!(satisfies_hint(
            &['f', 'r', 'a', 'm', 'e'],
            &[
                CharHint::Nowhere('e'),
                CharHint::Here('r'),
                CharHint::Here('a'),
                CharHint::Nowhere('s'),
                CharHint::Here('e'),
            ]
        ),);
    }

    #[test]
    fn test_belle() {
        assert!(!satisfies_hint(
            &['b', 'e', 'l', 'l', 'e'],
            &[
                CharHint::Here('b'),
                CharHint::Nowhere('e'),
                CharHint::Nowhere('l'),
                CharHint::Here('l'),
                CharHint::Here('e'),
            ]
        ),);
    }

    #[test]
    fn test_here_elsewhere_none() {
        assert!(satisfies_hint(
            &['a', 'b', 'a', 'b', 'c'],
            &[
                CharHint::Here('a'),
                CharHint::Elsewhere('a'),
                CharHint::Nowhere('x'),
                CharHint::Nowhere('a'),
                CharHint::Nowhere('x'),
            ]
        ),);
    }
}
