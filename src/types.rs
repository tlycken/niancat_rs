pub struct Puzzle(pub String);
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Word(pub String);

use regex::Regex;

impl Word {
    // Normalize a word by removing all non-alpha characters.
    pub fn normalize(&Word(ref w): &Word) -> Word {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[^A-Za-zåäöÅÄÖ]").unwrap();
        }

        Word(RE.replace_all(w.as_str(), "").to_uppercase())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const NORMALIZATION_TESTS: &'static [(&'static str, &'static str)] = &[
        ("GALLTJUTA", "GALLTJUTA"),
        ("galltjuta", "GALLTJUTA"),
        ("DATORSPEL", "DATORSPEL"),
        ("datorspel", "DATORSPEL"),
        ("dator spel", "DATORSPEL"),
        ("dator-spel", "DATORSPEL"),
        ("  dator-spel\n", "DATORSPEL"),
        ("abcdefåäö", "ABCDEFÅÄÖ")
    ];


    #[test]
    fn normalization_test() {
        for (input, expected) in NORMALIZATION_TESTS.iter().map(|x| (Word(x.0.to_string()), Word(x.1.to_string()))) {
            let actual = Word::normalize(&input);
            assert!(actual == expected, "Actual: {:?}, Expected: {:?}", actual, expected);
        }
    }
}
