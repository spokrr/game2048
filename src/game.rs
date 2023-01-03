
// #![allow(dead_code)] // temporary
// #![allow(unused_variables)] // temporary


pub struct Game {
    grid: [[u32; 4]; 4],
    score: u32,
    game_over: bool,
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

    pub fn print_board(&self) -> String {
        let mut return_str = String::new();
        // we want to make the board length dynamically expand 
        // based on the widest element in the board
        let widest_width = self.find_widest_element_width();
        // each line with values needs 8 spaces and 5 pipes 
        // plus the width of the values
        let line_width: usize = ((widest_width * 4) + 13u32) as usize;
        for i in 0..line_width { return_str += "-"; }

        return_str
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
        let width: u32 = f32::log10(biggest_so_far as f32) as u32;
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