use std::io::{Stdin, Stdout, Write};

use crate::Dict;

pub struct WordleIo {
    stdout: Stdout,
    stdin: Stdin,
}

impl WordleIo {
    pub fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
            stdin: std::io::stdin(),
        }
    }

    pub fn get_user_guess(&mut self, dict: &Dict, msg: Option<&str>) -> String {
        let guess = self.prompt(msg.unwrap_or("Guess: "));

        let valid_guess = dict.is_valid(&guess);

        if valid_guess {
            guess
        } else {
            self.get_user_guess(dict, Some("Invalid word try again: "))
        }
    }

    fn prompt(&mut self, val: &str) -> String {
        print!("{}", val);

        self.stdout.flush().expect("could not write to stdout");

        let mut guess = String::new();

        self.stdin
            .read_line(&mut guess)
            .expect("Could not read value");

        guess
    }
}

pub fn load_words_from_file(bytes: &[u8]) -> Vec<String> {
    let file = String::from_utf8_lossy(&bytes);

    file.split("\n")
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
}
