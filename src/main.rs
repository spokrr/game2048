mod game;
use game::Game;
use std::io;

fn main() {
    let mut game: Game = Game::new_game();
    // let mut game2 = Game::default();
    println!("{}", game.to_string());
    // println!("{}", game2.to_string());
}
