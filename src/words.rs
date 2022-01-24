const WORDS: &str = include_str!("words.txt");

pub fn iter_words() -> impl Iterator<Item = &'static str> {
    WORDS.lines()
}
