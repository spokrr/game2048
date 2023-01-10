use core::fmt;

#[derive(Clone, Copy)]
pub struct Game {
    grid: [[u32; 4]; 4],
    game_over: bool,
}


#[derive(PartialEq)]
pub struct Coords {
    i: usize,
    j: usize,
}

pub enum Direction {
    Up,
    Left,
    Down,
    Right
}

impl Game {
    // adds a random 2 or 4 on the board, returns the coords if successful, None if failure
    fn add_rand_tile(&mut self) -> Option<String> {
        let coords: String;
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
            coords = String::from(format!("{},{}", y + 1, 4 - x));
            return Some(coords)
        } else {
            self.game_over = true;
            return None;
        }
    }

    fn find_non_empty_tiles(&self,) -> Vec<Coords> {
        let mut non_empty_tiles = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                if self.grid[i][j] != 0 {
                    non_empty_tiles.push(Coords::from((i, j)));
                    // sorted from top to bottom and left to right
                }
            }
        }
        non_empty_tiles
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

    pub fn game_move(&mut self, dir: Direction) {
        // for performing the movements
        match dir {
            Direction::Up => { self.merge_up(); self.move_up(); self.merge_up(); },
            Direction::Left => { self.merge_left(); self.move_left(); self.merge_left(); },
            Direction::Down => { self.merge_down(); self.move_down(); self.merge_down(); },
            Direction::Right => { self.merge_right(); self.move_right(); self.merge_right(); },
        }
        println!("DEBUG: moved {}", dir);
        println!("DEBUG: adding random tile at {}", self.add_rand_tile().unwrap_or(String::from("ERR")));
    }

    pub fn get_score(&self) -> u32 {
        let non_empty_tiles = self.find_non_empty_tiles();
        let mut score = 0;

        for coords in non_empty_tiles.iter() {
            score += self.grid[coords.i][coords.j];
        }
        score
    }

    pub fn is_over(&self) -> bool {
        self.game_over
    }

    fn merge_down(&mut self) {
        let mut non_empty_tiles = self.find_non_empty_tiles();
        // reverse so we go bottom to top
        non_empty_tiles.reverse();

        for coords in non_empty_tiles.iter() {
            // make sure we don't array out of bounds
            if (coords.i < 3) && (self.grid[coords.i][coords.j] == self.grid[coords.i+1][coords.j]) {
                self.grid[coords.i][coords.j] = 0;
                self.grid[coords.i+1][coords.j] *= 2;
                println!("DEBUG: merging tile {0},{1} {5}wards! the value of {2},{3} is now {4}",
                                       coords.j+1,4-coords.i,         coords.j+1,4-coords.i-1, self.grid[coords.i+1][coords.j], Direction::Down);
            }
        }
    }

    fn merge_left(&mut self) {
        let non_empty_tiles = self.find_non_empty_tiles();

        for coords in non_empty_tiles.iter() {
            // make sure we don't array out of bounds
            if (coords.j > 0) && (self.grid[coords.i][coords.j] == self.grid[coords.i][coords.j-1]) {
                self.grid[coords.i][coords.j] = 0;
                self.grid[coords.i][coords.j-1] *= 2;
                println!("DEBUG: merging tile {0},{1} {5}wards! the value of {2},{3} is now {4}",
                                       coords.j+1,4-coords.i,         coords.j,4-coords.i, self.grid[coords.i][coords.j-1], Direction::Left);
            }
        }
    }

    fn merge_right(&mut self) {
        let mut non_empty_tiles = self.find_non_empty_tiles();
        // reverse the vec so we sort the right way
        non_empty_tiles.reverse();

        for coords in non_empty_tiles.iter() {
            // make sure we don't array out of bounds
            if (coords.j < 3) && (self.grid[coords.i][coords.j] == self.grid[coords.i][coords.j+1]) {
                self.grid[coords.i][coords.j] = 0;
                self.grid[coords.i][coords.j+1] *= 2;
                println!("DEBUG: merging tile {0},{1} {5}wards! the value of {2},{3} is now {4}",
                                       coords.j+1,4-coords.i,         coords.j+2,4-coords.i, self.grid[coords.i][coords.j+1], Direction::Left);
            }
        }
    }

    fn merge_up(&mut self) {
        let non_empty_tiles = self.find_non_empty_tiles();

        for coords in non_empty_tiles.iter() {
            // make sure we don't array out of bounds
            if (coords.i > 0) && (self.grid[coords.i][coords.j] == self.grid[coords.i-1][coords.j]) {
                self.grid[coords.i][coords.j] = 0;
                self.grid[coords.i-1][coords.j] *= 2;
                println!("DEBUG: merging tile {0},{1} {5}wards! the value of {2},{3} is now {4}",
                                       coords.j+1,4-coords.i,         coords.j+1,4-coords.i+1, self.grid[coords.i-1][coords.j], Direction::Up);
            }
        }

        // previous implementation, doesn't use find_non_empty_tiles()
        /* for i in 1..4 {
            for j in 0..4 {
                // if two tiles vertically are the same, merge em
                if (self.grid[i][j] != 0) && (self.grid[i][j] == self.grid[i-1][j]) {
                    self.grid[i][j] = 0;
                    self.grid[i-1][j] *= 2;
                    println!("DEBUG: merging tile {0},{1} {5}wards! the value of {2},{3} is now {4}",
                                                    j+1,4-i,                       j+1,4-i+1, self.grid[i-1][j], Direction::Up);
                }
            }
        } */
    }

    fn move_down(&mut self) {
        // first we find non-empty tiles
        let mut non_empty_tiles = self.find_non_empty_tiles();
        // reverse the vec because it's sorted top to bottom initially
        non_empty_tiles.reverse();

        // attempt to move each non-empty tile down, starting from the bottom and working upwards
        for coords in non_empty_tiles.iter_mut() {
            while coords.i < 3 && self.grid[coords.i+1][coords.j] == 0 {
                self.grid[coords.i+1][coords.j] = self.grid[coords.i][coords.j];
                self.grid[coords.i][coords.j] = 0;
                coords.i += 1;
            }
        }
    }

    fn move_left(&mut self) {
        // first we find non-empty tiles
        let mut non_empty_tiles = self.find_non_empty_tiles();

        // attempt to move each non-empty tile left, starting from the left and working right
        for coords in non_empty_tiles.iter_mut() {
            while coords.j > 0 && self.grid[coords.i][coords.j-1] == 0 {
                self.grid[coords.i][coords.j-1] = self.grid[coords.i][coords.j];
                self.grid[coords.i][coords.j] = 0;
                coords.j -= 1;
            }
        }
    }

    fn move_right(&mut self) {
        // first we find non-empty tiles
        let mut non_empty_tiles = self.find_non_empty_tiles();
        // reverse the vec because it's sorted left to right initially
        non_empty_tiles.reverse();

        // attempt to move each non-empty tile right, starting from the right and working left
        for coords in non_empty_tiles.iter_mut() {
            while coords.j < 3 && self.grid[coords.i][coords.j+1] == 0 {
                self.grid[coords.i][coords.j+1] = self.grid[coords.i][coords.j];
                self.grid[coords.i][coords.j] = 0;
                coords.j += 1;
            }
        }
    }

    fn move_up(&mut self) {
        // first we find non-empty tiles
        let mut non_empty_tiles = self.find_non_empty_tiles();

        // attempt to move each non-empty tile up, starting from the top and working downwards
        for coords in non_empty_tiles.iter_mut() {
            while coords.i > 0 && self.grid[coords.i-1][coords.j] == 0 {
                self.grid[coords.i-1][coords.j] = self.grid[coords.i][coords.j];
                self.grid[coords.i][coords.j] = 0;
                coords.i -= 1;
            }
        }
    }

    pub fn new_game() -> Game {
        // empty board, add two random tiles
        let mut game = Game { grid: [[0; 4]; 4], 
                                    game_over: false };
        game.add_rand_tile();
        game.add_rand_tile();
        
        game
    }

    pub fn parse_move(move_: &str) -> Result<Direction, &str> {
        // for parsing directions in the case of a TUI
        match move_.to_lowercase().as_str() {
            "w" => Ok(Direction::Up),
            "a" => Ok(Direction::Left),
            "s" => Ok(Direction::Down),
            "d" => Ok(Direction::Right),
            _ => Err("invalid direction"),
        }
    }

}

impl Coords {
    fn from((i, j): (usize, usize)) -> Coords {
        Coords { i: i, j: j }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // from the unicode box drawing charset
        let top_left: &str =            "╔";
        let top_intersection: &str =    "╦";
        let top_right: &str =           "╗";
        let left_intersection: &str =   "╠";
        let right_intersection: &str =  "╣";
        let cross: &str =               "╬";
        let bottom_intersection: &str = "╩";
        let bottom_left: &str =         "╚";
        let bottom_right: &str =        "╝";
        let pipe: &str =                "║";
        let line: &str =                "═";

        let mut return_str = String::new();
        // we want to make the board length dynamically expand 
        // based on the widest element in the board
        let widest_width = self.find_widest_element_width();
        // each line with values needs 8 spaces and 5 pipes 
        // plus the width of the values
        let line_width: usize = (widest_width as usize * 4) + 13;
        let box_width: usize = widest_width as usize + 3;
        return_str += top_left;
        for i in 0..line_width - 2 {
            if (i + 1) % box_width == 0 {
                return_str += top_intersection;
            } else {
                return_str += line;
            }

        }
        return_str += top_right;
        
        let mut grid_minus_last_row = self.grid.to_vec();
        grid_minus_last_row.pop();
        // The middle two rows are simple
        for arr in grid_minus_last_row.iter() {
            return_str += "\n";
            for element in arr.iter() {
                let width = widest_width as usize + 2;
                return_str += pipe;
                return_str += format!("{:^width$}", element).as_str();
            }
            return_str += pipe;
            return_str += "\n";
            return_str += left_intersection;
            for i in 0..line_width - 2 {
                if (i + 1) % box_width == 0 {
                    return_str += cross;
                } else {
                    return_str += line;
                }
    
            }
            return_str += right_intersection;
        }
        // fourth row:
        return_str += "\n";
        
        for element in self.grid[3].iter() {
            let width = widest_width as usize + 2;
                return_str += pipe;
                return_str += format!("{:^width$}", element).as_str();
        }
        return_str += pipe;
        return_str += "\n";
        return_str += bottom_left;
        for i in 0..line_width - 2 {
            if (i + 1) % box_width == 0 {
                return_str += bottom_intersection;
            } else {
                return_str += line;
            }

        }
        return_str += bottom_right;

        /*
        TODO: fix below
        */
        write!(f, "{}", return_str) // inefficient, should write to f instead of return_str
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