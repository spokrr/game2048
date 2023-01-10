mod game;
use std::io;

use game::Game;

// use crate::game::Direction::*;

fn main() {
    println!("╔{:═^28}╗", "");
    println!("║{:^28}║", "Welcome to 2048!");
    println!("╚{:═^28}╝", "");

    let mut game: Game = Game::new_game();

    // main game loop
    while !game.is_over() {
        println!("{}", game.to_string());
        println!("Enter move (w/a/s/d) or q to quit:");
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Failed to read line");
        let inp= inp.trim().to_lowercase();
        let inp = inp.as_str();

        if inp == "q" {
            break;
        }

        match Game::parse_move(inp) {
            Ok(dir) => game.game_move(dir),
            Err(_) => println!("Invalid direction!"),
        }    

    }

    println!("Game over!");
    println!("Score: {}", game.get_score());
}