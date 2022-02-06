use rand::seq::SliceRandom;
use std::fs::read_to_string;

use crate::words::WORDS;

pub struct Dict {
    pub known_words: Vec<String>,
}

impl Dict {
    pub fn new() -> Self {
        let dict_file = read_to_string("/usr/share/dict/words").unwrap_or(String::new());
        let known_words: Vec<String> = dict_file.split("\n").map(|f| f.to_string()).collect();

        Self { known_words }
    }

    pub fn select_word(&self) -> &str {
        WORDS
            .choose(&mut rand::thread_rng())
            .expect("could not select word from word list")
    }

    pub fn is_valid(&self, word: String) -> bool {
        let str = word.as_str();

        if word.len() == 5 {
            if WORDS.contains(&str) {
                true
            } else if self.known_words.len() > 0 {
                self.known_words.contains(&word)
            } else {
                true
            }
        } else {
            false
        }
    }
}
