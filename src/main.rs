mod game;
mod words;

use game::{Game, GameResult};
use rand::seq::SliceRandom;
use std::io::{self, Write};
use words::WORDS;

fn main() {
    let word = WORDS
        .choose(&mut rand::thread_rng())
        .expect("could not select word from word list");

    let mut game = Game::new(word);
    let mut res = GameResult::Incomplete;

    while res == GameResult::Incomplete {
        print!("Guess: ");

        io::stdout().flush().expect("could not write to stdout");

        let mut valid_guess = false;

        while !valid_guess {
            let mut guess = String::new();

            std::io::stdin()
                .read_line(&mut guess)
                .expect("Could not read value");

            if guess.trim().len() == 5 {
                valid_guess = true;

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
