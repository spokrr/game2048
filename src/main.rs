mod game;
use game::Game;
use std::io;

fn main() {
    let mut game: Game = Game::new_game();
    println!("running");
}
