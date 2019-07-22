use crate::wordlist::MoneroWordlist;
use wagu_model::wordlist::Wordlist;

const LOJBAN: &str = include_str!("./dictionary/lojban.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lojban;

impl Wordlist for Lojban {}

impl MoneroWordlist for Lojban {
    /// Returns the word list as a string.
    fn get_all() -> Vec<&'static str> {
        LOJBAN.lines().collect::<Vec<&str>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_WORD: &str = "mansa";
    const VALID_WORD_INDEX: usize = 783;
    const INVALID_WORD: &str = "a";
    const INVALID_WORD_INDEX: usize = 3400;

    #[test]
    fn get() {
        // Valid case
        assert_eq!(VALID_WORD, Lojban::get(VALID_WORD_INDEX).unwrap());
        // Invalid case
        assert!(Lojban::get(INVALID_WORD_INDEX).is_err());
    }

    #[test]
    fn get_index() {
        // Valid case
        assert_eq!(VALID_WORD_INDEX, Lojban::get_index(VALID_WORD).unwrap());
        // Invalid case
        assert!(Lojban::get_index(INVALID_WORD).is_err());
    }

    #[test]
    fn get_all() {
        let list = Lojban::get_all();
        assert_eq!(1626, list.len());
        assert_eq!(VALID_WORD, list[VALID_WORD_INDEX]);
    }
}
