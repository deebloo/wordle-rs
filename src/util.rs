use std::io::{stdin, stdout, Stdin, Stdout, Write};

pub struct WordleIo {
    stdout: Stdout,
    stdin: Stdin,
}

impl WordleIo {
    pub fn new() -> Self {
        Self {
            stdout: stdout(),
            stdin: stdin(),
        }
    }

    pub fn get_user_guess<F>(&mut self, msg: &str, is_valid: F) -> String
    where
        F: Fn(&String) -> bool,
    {
        let guess = self.prompt(msg);

        if is_valid(&guess) {
            guess
        } else {
            self.get_user_guess("Invalid word try again: ", is_valid)
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
