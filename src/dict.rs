use crate::util::load_words_from_file;

pub struct Dict {
    pub known_words: Vec<String>,
    pub word_pool: Vec<String>,
}

impl Dict {
    pub fn new() -> Self {
        Self {
            known_words: load_words_from_file(include_bytes!("../words/5ltr.txt")),
            word_pool: load_words_from_file(include_bytes!("../words/pool.txt")),
        }
    }

    pub fn select_word(&self) -> &str {
        "hello"
    }

    pub fn is_valid(&self, word: &String) -> bool {
        false
    }
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

        assert_eq!(dict.is_valid(&"adieu".to_string()), true);
        assert_eq!(dict.is_valid(&"spoil".to_string()), true);
    }

    #[test]
    fn should_validate_trimmed_word() {
        let dict = Dict::new();

        assert_eq!(dict.is_valid(&"adieu ".to_string()), true);
        assert_eq!(dict.is_valid(&" spoil".to_string()), true);
    }

    #[test]
    fn should_validate_real_short_word() {
        let dict = Dict::new();

        assert_eq!(dict.is_valid(&"ask".to_string()), false);
        assert_eq!(dict.is_valid(&"pass".to_string()), false);
    }

    #[test]
    fn should_validate_unknown_word() {
        let dict = Dict::new();

        assert_eq!(dict.is_valid(&"asdfg".to_string()), false);
        assert_eq!(dict.is_valid(&"poruf".to_string()), false);
    }
}
