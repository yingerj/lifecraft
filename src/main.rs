extern crate piston_window;
//extern crate rand;

use piston_window::*;
//use rand::os::OsRng;

//TODO: Implement random seeding of the board.

const BOARD_SIZE: usize = 10; //cells
const CELL_SIZE: usize = 40; //pixels

fn toggle_cell(board: &mut [[bool; BOARD_SIZE]; BOARD_SIZE], cell_pos: [usize; 2]) {
    let cell = &mut board[cell_pos[0]][cell_pos[1]];
    *cell = !*cell;
}

fn window_to_board_coordinates(screen_pos: [f64; 2]) -> [usize; 2] {
    let cell_size = CELL_SIZE as f64;
    let cell_float = [screen_pos[0]/cell_size, screen_pos[1]/cell_size];
    [cell_float[0] as usize, cell_float[1] as usize]
}

fn wrapping_idx(idx: usize, d_idx: isize) -> usize {
    let i_board_size = BOARD_SIZE as isize;
    let mut iidx = idx as isize;
    if (iidx + d_idx) == i_board_size {
        iidx = 0;
    }
    else if (iidx + d_idx) == -1 {
        iidx = i_board_size - 1;
    }
    else {
        iidx = iidx + d_idx;
    }
    iidx as usize
}

// Disclaimer: This algorithm is really bad, but basics first!
fn game_step(board: &mut [[bool; BOARD_SIZE]; BOARD_SIZE]) {
    let mut neighbor_cnt: [[u8; BOARD_SIZE]; BOARD_SIZE] = [[0; BOARD_SIZE]; BOARD_SIZE];
    // Check each cell for life an increment the neighbor_cnt of all of it's neighbors if found.
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            // If live cell found...
            if board[x][y] {
                // Increment neighbors' neighbor_cnt cells.
                for dx in -1..2 {
                    for dy in -1..2 {
                        //Don't Increment your own cell's neighbor_y, only those of neighbors.
                        if !((dx == 0) && (dy == 0)) {
                            let inc_x = wrapping_idx(x, dx);
                            let inc_y = wrapping_idx(y, dy);
                            println!("inc x:{:?}, y:{:?}", inc_x, inc_y);
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
            if board[x][y] && !(neighbor_cnt[x][y] == 2 || neighbor_cnt[x][y] == 3) {
                board[x][y] = false;
            }
            else if !board[x][y] && neighbor_cnt[x][y] == 3 {
                board[x][y] = true;
            }

        }
    }
}

fn main() {
    let mut mouse_pos: [f64; 2] = [0.0, 0.0];
    let mut board: [[bool; BOARD_SIZE]; BOARD_SIZE] = [[false; BOARD_SIZE]; BOARD_SIZE];
    let mut run: bool = false;

    let window: PistonWindow =
        WindowSettings::new("lifecraft", [(BOARD_SIZE * CELL_SIZE) as u32; 2])
        .exit_on_esc(true).build().unwrap();

    for e in window.ups(5).max_fps(5) {
        e.draw_2d(|c, g| {
            println!("Redraw");
            if run {
                game_step(&mut board);
            }
            clear([1.0; 4], g);
            for x in 0..BOARD_SIZE {
                for y in 0..BOARD_SIZE {
                    if board[x][y] {
                        let x_position: f64 = (x * CELL_SIZE) as f64;
                        let y_position: f64 = (y * CELL_SIZE) as f64;
                        rectangle([0.0, 0.0, 0.0, 1.0], // red
                                  [x_position, y_position, CELL_SIZE as f64, CELL_SIZE as f64],
                                  c.transform, g);
                    }
                }
            }
        });

        // Capture latest mouse cursor position.
        if let Some(pos) = e.mouse_cursor_args() {
            mouse_pos = pos;
        }

        if let Some(button) = e.press_args() {
            // Toggle cell state under cursor.
            if button == Button::Mouse(MouseButton::Left) {
                println!("Pressed {:?}", button);
                println!("Mouse position x:{:?} y:{:?}", mouse_pos[0], mouse_pos[1]);
                toggle_cell(&mut board, window_to_board_coordinates(mouse_pos));
            }
            // Play/Pause the game.
            if button == Button::Keyboard(Key::Space) {
                run = !run;
            }
        }
    }
}
