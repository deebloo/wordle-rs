use rand::seq::SliceRandom;
use std::fs::read_to_string;

pub struct Dict {
    pub known_words: Vec<String>,
    pub word_pool: Vec<String>,
}

impl Dict {
    pub fn new() -> Self {
        let known_file = read_to_string("words_5ltr.txt").expect("could not load words file");
        let pool_file = read_to_string("words_pool.txt").expect("could not load pool file");

        Self {
            known_words: known_file
                .split("\n")
                .map(|f| f.to_string())
                .collect::<Vec<String>>(),

            word_pool: pool_file
                .split("\n")
                .map(|f| f.to_string())
                .collect::<Vec<String>>(),
        }
    }

    pub fn select_word(&self) -> &str {
        self.word_pool
            .choose(&mut rand::thread_rng())
            .expect("could not select word from word list")
    }

    pub fn is_valid(&self, word: String) -> bool {
        if word.len() == 5 {
            if self.word_pool.contains(&word) {
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
