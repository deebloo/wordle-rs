use rand::seq::SliceRandom;
use std::fs::read_to_string;

pub struct Dict {
    pub known_words: Vec<String>,
    pub word_pool: Vec<String>,
}

impl Dict {
    pub fn new() -> Self {
        Self {
            known_words: words_from_file("words_5ltr.txt"),
            word_pool: words_from_file("words_pool.txt"),
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

fn words_from_file(path: &str) -> Vec<String> {
    let file = read_to_string(path).expect(format!("could not load {}", path).as_str());

    file.split("\n")
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
}
