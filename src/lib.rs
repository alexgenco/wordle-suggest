use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    iter,
};

mod weights {
    include!(concat!(env!("OUT_DIR"), "/weights.rs"));
}

pub type Hint = [CharHint; 5];
pub type Word = [char; 5];

#[derive(Clone, Debug, PartialEq)]
pub enum CharHint {
    Here(char),
    Elsewhere(char),
    None(char),
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
    hints: &Vec<Hint>,
    unique: bool,
    singular: bool,
    limit: Option<usize>,
) -> impl Iterator<Item = String> {
    let mut heap: BinaryHeap<WeightedWord> = weights::WEIGHTS
        .into_iter()
        .filter_map(|(word, weight)| {
            if satisfies_singular(word, singular)
                && satisfies_uniqueness(word, unique)
                && satisfies_hints(word, hints)
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

fn satisfies_hints(word: Word, hints: &Vec<Hint>) -> bool {
    hints.iter().all(|hint| satisfies_hint(word, hint))
}

fn satisfies_hint(word: Word, hint: &Hint) -> bool {
    let matched_char_indices =
        hint.into_iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, cg)| {
                match cg {
                    CharHint::Here(c) | CharHint::Elsewhere(c) => {
                        acc.entry(*c).or_insert(Vec::new()).push(i);
                    }
                    CharHint::None(_) => {}
                }
                acc
            });

    hint.iter().enumerate().all(|(i, cg)| match cg {
        CharHint::Here(c) => word[i] == *c,
        CharHint::Elsewhere(c) => word.contains(c) && word[i] != *c,
        CharHint::None(c) => {
            if let Some(is) = matched_char_indices.get(c) {
                !is.into_iter().any(|j| *j == i)
            } else {
                !word.contains(c)
            }
        }
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

#[cfg(test)]
mod test {
    use crate::{satisfies_hint, CharHint};

    #[test]
    fn test_satisfies_hint() {
        assert!(
            satisfies_hint(
                ['m', 'o', 'n', 'e', 'y'],
                &[
                    CharHint::None('q'),
                    CharHint::None('x'),
                    CharHint::None('p'),
                    CharHint::None('z'),
                    CharHint::None('r'),
                ]
            ),
            "All `None`s are satisfied by a word containing none of those letters"
        );

        assert!(
            satisfies_hint(
                ['m', 'o', 'n', 'e', 'y'],
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

        assert!(
            satisfies_hint(
                ['m', 'o', 'n', 'e', 'y'],
                &[
                    CharHint::Elsewhere('y'),
                    CharHint::None('x'),
                    CharHint::None('p'),
                    CharHint::None('z'),
                    CharHint::None('r'),
                ]
            ),
            "An `Elsewhere` is satisfied with a letter in a different position"
        );

        assert!(
            !satisfies_hint(
                ['a', 'p', 'n', 'i', 'c'],
                &[
                    CharHint::Elsewhere('p'),
                    CharHint::None('a'), // <-
                    CharHint::Here('n'),
                    CharHint::Here('i'),
                    CharHint::Here('c'),
                ]
            ),
            "A single `None` rejects words containing that letter"
        );

        assert!(
            satisfies_hint(
                ['b', 'o', 'a', 't', 's'],
                &[
                    CharHint::Here('b'),
                    CharHint::Elsewhere('a'),
                    CharHint::None('b'),
                    CharHint::None('b'),
                    CharHint::None('y'),
                ]
            ),
            "Repeated hints can be marked `None`"
        );
    }
}
