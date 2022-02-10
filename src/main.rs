mod dict;
mod game;

use dict::Dict;
use game::{Game, GameResult};
use std::io::{self, Stdout, Write};

fn main() {
    let dict = Dict::new();
    let word = dict.select_word();
    let mut stdout = io::stdout();

    let mut game = Game::new(word);
    let mut res = GameResult::Incomplete;

    println!("{}", game);

    while res == GameResult::Incomplete {
        prompt(&mut stdout, "Guess: ");

        let mut valid_guess = false;

        while !valid_guess {
            let mut guess = String::new();

            std::io::stdin()
                .read_line(&mut guess)
                .expect("Could not read value");

            valid_guess = dict.is_valid(guess.trim().to_string());

            if valid_guess {
                res = game.guess(guess.as_str());

                println!("{}", game);
            } else {
                prompt(&mut stdout, "Invalid word try again: ")
            }
        }
    }

    if res == GameResult::Loose {
        println!("To bad! The word was: {:?}", &word);
    } else {
        println!("Well done!");
    }
}

fn prompt(stdout: &mut Stdout, val: &str) {
    print!("{}", val);

    stdout.flush().expect("could not write to stdout");
}
