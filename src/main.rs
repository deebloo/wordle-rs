mod dict;
mod game;
mod util;

use dict::Dict;
use game::{Game, GameResult};
use util::WordleIo;

fn main() {
    let mut io = WordleIo::new();
    let dict = Dict::new();
    let mut game = Game::new(dict.select_word());

    println!("{}", game);

    let mut res = GameResult::Incomplete;

    while res == GameResult::Incomplete {
        let guess = io.get_user_guess(&dict, None);

        res = game.guess(guess.as_str());

        println!("{}", game);
    }

    if res == GameResult::Loose {
        println!("To bad! The word was: {:?}", String::from_iter(game.word));
    } else {
        println!("Well done!");
    }
}
