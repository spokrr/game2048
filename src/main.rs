mod game;
use std::io;

use game::Game;

use crate::game::Direction::*;

fn main() {
    println!("╔{:═^28}╗", "");
    println!("║{:^28}║", " Welcome to 2048! ");
    println!("╚{:═^28}╝", "");

    let game: Game = Game::new_game();

    // main game loop
    while !game.is_over() {
        println!("{}", game.to_string());
        println!("Enter move (w/a/s/d) or q to quit:");
        let mut move_ = String::new();
        io::stdin().read_line(&mut move_).expect("Failed to read line");
        let move_= move_.trim().to_lowercase();
        let move_ = move_.as_str();

        if move_ == "q" {
            break;
        }

        match Game::parse_move(move_) {
            Ok(dir) => game.game_move(dir),
            Err(err) => println!("Invalid direction! err: {}", err),
        }    

    }
}