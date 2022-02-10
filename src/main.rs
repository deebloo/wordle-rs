mod dict;
mod game;

use dict::Dict;
use game::{Game, GameResult};
use std::io::{self, Stdin, Stdout, Write};

fn main() {
    let dict = Dict::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut game = Game::new(dict.select_word());

    println!("{}", game);

    let mut res = GameResult::Incomplete;

    while res == GameResult::Incomplete {
        prompt(&mut stdout, "Guess: ");

        let guess = get_user_guess(&stdin, &mut stdout, &dict);

        res = game.guess(guess.as_str());

        println!("{}", game);
    }

    if res == GameResult::Loose {
        println!("To bad! The word was: {:?}", &game.word);
    } else {
        println!("Well done!");
    }
}

fn get_user_guess(stdin: &Stdin, stdout: &mut Stdout, dict: &Dict) -> String {
    let mut guess = String::new();

    stdin.read_line(&mut guess).expect("Could not read value");

    let valid_guess = dict.is_valid(&guess);

    if valid_guess {
        guess
    } else {
        prompt(stdout, "Invalid word try again: ");

        get_user_guess(stdin, stdout, dict)
    }
}

fn prompt(stdout: &mut Stdout, val: &str) {
    print!("{}", val);

    stdout.flush().expect("could not write to stdout");
}
