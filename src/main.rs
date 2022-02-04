mod game;
mod words;

use game::{Game, GameResult};
use rand::seq::SliceRandom;
use std::fs::read_to_string;
use std::io::{self, Write};
use words::WORDS;

fn main() {
    let dict_file = read_to_string("/usr/share/dict/words").unwrap_or(String::new());
    let words: Vec<&str> = dict_file.split("\n").collect();

    let word = WORDS
        .choose(&mut rand::thread_rng())
        .expect("could not select word from word list");

    let mut game = Game::new(word);
    let mut res = GameResult::Incomplete;

    println!("{}", game);

    while res == GameResult::Incomplete {
        print!("Guess: ");

        io::stdout().flush().expect("could not write to stdout");

        let mut valid_guess = false;

        while !valid_guess {
            let mut guess = String::new();

            std::io::stdin()
                .read_line(&mut guess)
                .expect("Could not read value");

            let trimmed = guess.trim();
            let is_long_enough = trimmed.len() == 5;

            valid_guess = if is_long_enough {
                if WORDS.contains(&trimmed) {
                    true
                } else if words.len() > 0 {
                    words.contains(&trimmed)
                } else {
                    true
                }
            } else {
                false
            };

            if valid_guess {
                res = game.guess(guess.as_str());

                println!("{}", game);
            } else {
                print!("Invalid word try again: ");

                io::stdout().flush().expect("could not write to stdout");
            }
        }
    }

    if res == GameResult::Loose {
        println!("To bad! The word was: {:?}", &word);
    } else {
        println!("Well done!");
    }
}
