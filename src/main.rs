mod dict;
mod game;

use dict::Dict;
use game::{Game, GameResult};
use std::io::{self, Write};

fn main() {
    let dict = Dict::new();
    let word = dict.select_word();

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

            valid_guess = dict.is_valid(guess.trim().to_string());

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

// Generate word list
// fn main() {
//     use std::fs::{read_to_string, write};

//     let word_file = read_to_string("words_all.txt").unwrap();
//     let words: String = word_file
//         .split("\n")
//         .filter_map(|f| {
//             let letters_only = f.chars().all(|x| x.is_alphabetic());
//             let val = f.to_lowercase().to_string();

//             if letters_only && val.len() == 5 {
//                 Some(val + "\n")
//             } else {
//                 None
//             }
//         })
//         .collect::<String>()
//         .trim()
//         .to_string();

//     write("words_5ltr.txt", words).unwrap();
// }
