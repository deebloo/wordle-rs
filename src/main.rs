mod dict;
mod game;
mod words;

use dict::Dict;
use game::{Game, GameResult};
use std::fs::{read_to_string, write};
use std::io::{self, Write};

fn main() {
    let word_file = read_to_string("words.txt").unwrap();
    let words: String = word_file
        .split("\n")
        .filter_map(|f| {
            let val = f.to_string();

            if val.len() == 5 {
                Some(val + "\n")
            } else {
                None
            }
        })
        .collect();

    write("./5-letter-words.txt", words).unwrap();

    // let dict = Dict::new();
    // let word = dict.select_word();

    // let mut game = Game::new(word);
    // let mut res = GameResult::Incomplete;

    // println!("{}", game);

    // while res == GameResult::Incomplete {
    //     print!("Guess: ");

    //     io::stdout().flush().expect("could not write to stdout");

    //     let mut valid_guess = false;

    //     while !valid_guess {
    //         let mut guess = String::new();

    //         std::io::stdin()
    //             .read_line(&mut guess)
    //             .expect("Could not read value");

    //         valid_guess = dict.is_valid(guess.trim().to_string());

    //         if valid_guess {
    //             res = game.guess(guess.as_str());

    //             println!("{}", game);
    //         } else {
    //             print!("Invalid word try again: ");

    //             io::stdout().flush().expect("could not write to stdout");
    //         }
    //     }
    // }

    // if res == GameResult::Loose {
    //     println!("To bad! The word was: {:?}", &word);
    // } else {
    //     println!("Well done!");
    // }
}
