use piston_window::*;

pub const BOARD_SIZE: usize = 20; //cells
pub const CELL_SIZE: usize = 30; //pixels

fn wrapping_idx(idx: usize, d_idx: isize) -> usize {
    let i_board_size = BOARD_SIZE as isize;
    let mut signed_idx = idx as isize;
    if (signed_idx + d_idx) == i_board_size {
        signed_idx = 0;
    }
    else if (signed_idx + d_idx) == -1 {
        signed_idx = i_board_size - 1;
    }
    else {
        signed_idx = signed_idx + d_idx;
    }
    signed_idx as usize
}

pub struct GameOfLife {
    pub mouse_pos: [f64; 2],
    pub board: [[bool; BOARD_SIZE]; BOARD_SIZE],
    pub run: bool,
}

//TODO: Implement random seeding of the board.
impl GameOfLife {
    pub fn new()-> GameOfLife {
        GameOfLife {
            mouse_pos: [0.0, 0.0],
            board: [[false; BOARD_SIZE]; BOARD_SIZE],
            run: false,
        }
    }

//TODO: Bring event handling in here from main.
/*
    pub fn render(&mut self) {
        |c, g| {
            println!("Redraw");
            clear([1.0; 4], g);
            for x in 0..BOARD_SIZE {
                for y in 0..BOARD_SIZE {
                    if self.board[x][y] {
                        let x_position: f64 = (x * CELL_SIZE) as f64;
                        let y_position: f64 = (y * CELL_SIZE) as f64;
                        rectangle([0.0, 0.0, 0.0, 1.0], // red
                                  [x_position, y_position, CELL_SIZE as f64, CELL_SIZE as f64],
                                  c.transform, g);
                    }
                }
            }
        }
    }
*/
    pub fn toggle_cell(&mut self) {
        let cell_size = CELL_SIZE as f64;
        let x = (self.mouse_pos[0]/cell_size) as usize;
        let y = (self.mouse_pos[1]/cell_size) as usize;
        let cell = &mut self.board[x][y];
        *cell = !*cell;
    }

    // Disclaimer: This algorithm is really bad, but basics first!
    pub fn game_step(&mut self) {
        let mut neighbor_cnt: [[u8; BOARD_SIZE]; BOARD_SIZE] = [[0; BOARD_SIZE]; BOARD_SIZE];
        // Check each cell for life an increment the neighbor_cnt of all of it's neighbors if found.
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                // If live cell found...
                if self.board[x][y] {
                    // Increment neighbors' neighbor_cnt cells.
                    for dx in -1..2 {
                        for dy in -1..2 {
                            //Don't Increment your own cell's neighbor_y, only those of neighbors.
                            if !((dx == 0) && (dy == 0)) {
                                let inc_x = wrapping_idx(x, dx);
                                let inc_y = wrapping_idx(y, dy);
                                neighbor_cnt[inc_x][inc_y] += 1;
                            }
                        }
                    }
                }
            }
        }
        // Update board
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                // if cell allready alive
                if self.board[x][y] && !(neighbor_cnt[x][y] == 2 || neighbor_cnt[x][y] == 3) {
                    self.board[x][y] = false;
                }
                else if !self.board[x][y] && neighbor_cnt[x][y] == 3 {
                    self.board[x][y] = true;
                }

            }
        }
    }
}
