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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_words() {
        let dict = Dict::new();

        assert_eq!(dict.known_words.len() > 1, true);
        assert_eq!(dict.word_pool.len() > 1, true);
    }

    #[test]
    fn should_validate_word() {
        let dict = Dict::new();

        assert_eq!(dict.is_valid("adieu".to_string()), true);
        assert_eq!(dict.is_valid("spoil".to_string()), true);
    }

    #[test]
    fn should_validate_real_short_word() {
        let dict = Dict::new();

        assert_eq!(dict.is_valid("ask".to_string()), false);
        assert_eq!(dict.is_valid("pass".to_string()), false);
    }

    #[test]
    fn should_validate_unknown_word() {
        let dict = Dict::new();

        assert_eq!(dict.is_valid("asdfg".to_string()), false);
        assert_eq!(dict.is_valid("poruf".to_string()), false);
    }
}
