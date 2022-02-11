mod dict;
mod game;

use dict::Dict;
use game::{Game, GameResult};
use std::io::{self, Write};

fn main() {
    let dict = Dict::new();
    let mut game = Game::new(dict.select_word());

    println!("{}", game);

    let mut res = GameResult::Incomplete;

    while res == GameResult::Incomplete {
        let guess = get_user_guess(&dict, None);

        res = game.guess(guess.as_str());

        println!("{}", game);
    }

    if res == GameResult::Loose {
        println!("To bad! The word was: {:?}", String::from_iter(game.word));
    } else {
        println!("Well done!");
    }
}

fn get_user_guess(dict: &Dict, msg: Option<&str>) -> String {
    let guess = prompt(msg.unwrap_or("Guess: "));

    let valid_guess = dict.is_valid(&guess);

    if valid_guess {
        guess
    } else {
        get_user_guess(dict, Some("Invalid word try again: "))
    }
}

fn prompt(val: &str) -> String {
    print!("{}", val);

    io::stdout().flush().expect("could not write to stdout");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Could not read value");

    guess
}
