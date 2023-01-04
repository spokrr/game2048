
#![allow(dead_code)] // temporary
#![allow(unused_variables)] // temporary

use core::fmt;

#[derive(Clone, Copy)]
pub struct Game {
    grid: [[u32; 4]; 4],
    score: u32,
    game_over: bool,
}

pub enum Direction {
    Up,
    Left,
    Down,
    Right
}

impl Game {
    pub fn new_game() -> Game {
        let mut game = Game { grid: [[0; 4]; 4], 
                                    score: 0,
                                    game_over: false };
        game.add_rand_tile();
        game.add_rand_tile();
        
        game
    }

    pub fn parse_move(move_: &str) -> Result<Direction, &str> {
        match move_.to_lowercase().as_str() {
            "w" => Ok(Direction::Up),
            "a" => Ok(Direction::Left),
            "s" => Ok(Direction::Down),
            "d" => Ok(Direction::Right),
            _ => Err("invalid direction"),
        }
    }

    pub fn game_move(&mut self, dir: Direction) {
        
        match dir {
            Direction::Up => { self.merge_up(); self.move_up(); self.merge_up(); },
            Direction::Left => { self.merge_left(); self.move_left(); self.merge_left(); },
            Direction::Down => { self.merge_down(); self.move_down(); self.merge_down(); },
            Direction::Right => { self.merge_right(); self.move_right(); self.merge_right(); },
        }
        println!("DEBUG: moved {}", dir);
        println!("DEBUG: adding random tile");
        self.add_rand_tile();
    }

    fn move_up(&mut self) {

    }

    fn merge_up(&mut self) {
        for i in 1..3 {
            for j in 0..3 {
                if self.grid[i][j] != 0 {
                    if self.grid[i][j] == self.grid[i-1][j] {
                        self.grid[i][j] = 0;
                        self.grid[i-1][j] *= 2;
                        println!("DEBUG: merging tile {0}, {1} upwards! the value of {2}, {3} is now {4}", 4 - j, 4 - i, (4 - j), (4 - i) + 1, self.grid[i-1][j]);
                    }
                }

            }
        }
    }

    fn move_left(&mut self) {

    }

    fn merge_left(&mut self) {

    }

    fn move_down(&mut self) {

    }

    fn merge_down(&mut self) {

    }

    fn move_right(&mut self) {

    }

    fn merge_right(&mut self) {

    }

    pub fn is_over(&self) -> bool {
        self.game_over
    }

    // returns the *width* of the element, NOT the element itself
    fn find_widest_element_width(&self) -> u32 {
        let mut biggest_so_far = self.grid[0][0];
        for i in 0..4 {
            for j in 0..4 {
                if self.grid[i][j] > biggest_so_far {
                    biggest_so_far = self.grid[i][j];
                }
            }
        }
        let width: u32 = f32::log10(biggest_so_far as f32) as u32 + 1;
        width
    }

    // adds a random 2 or 4 on the board, returns true if successful, false if failure
    fn add_rand_tile(&mut self) -> bool {
        let mut empty_tiles = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                if self.grid[i][j] == 0 {
                    empty_tiles.push((i,j));
                }
            }
        }

        if !empty_tiles.is_empty() {
            // index the empty tiles and pick a random one
            let index = rand::random::<usize>() % empty_tiles.len(); 
            let (x, y) = empty_tiles[index];
            // 10% chance to set to 4, 90% chance to set to 2
            self.grid[x][y] = if rand::random::<f32>() < 0.1 {
                4
            } else {
                2
            };
        } else {
            self.game_over = true;
            return false;
        }
        true
    }

}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut return_str = String::new();
        // we want to make the board length dynamically expand 
        // based on the widest element in the board
        let widest_width = self.find_widest_element_width();
        // each line with values needs 8 spaces and 5 pipes 
        // plus the width of the values
        let line_width: usize = ((widest_width * 4) + 13) as usize;
        for _i in 0..line_width { return_str += "-"; }
        for (i, arr) in self.grid.iter().enumerate() {
            return_str += "\n";
            for (j, element) in arr.iter().enumerate() {
                let width = widest_width as usize + 2;
                return_str += "|";
                return_str += format!("{:^width$}", element).as_str();
                // return_str += "|";
            }
            return_str += "|";
            return_str += "\n";
            for _i in 0..line_width { return_str += "-"; }
        }

        write!(f, "{}", return_str)
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Left => write!(f, "Left"),
            Direction::Down => write!(f, "Down"),
            Direction::Right => write!(f, "Right")
            // _ => write!(f, "Err")
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new_game()
    }
}