mod game;
mod words;

fn main() {
    let game = game::Game::new("hello");

    let res = game.guess("hello");

    println!("{:?}", res);
}
