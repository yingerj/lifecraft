extern crate piston_window;
//extern crate rand;

use piston_window::*;
//use rand::os::OsRng;

const GRID_SIZE: usize = 10; //cells
const CELL_SIZE: usize = 10; //pixels

fn checkerboard(grid: &mut [[u8; GRID_SIZE]; GRID_SIZE]) {
    let mut fill: bool = true;
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            fill = !fill;
            if fill {
                grid[x][y] = fill as u8;
            }
        }
        if GRID_SIZE % 2 == 0 {
            fill = !fill;
        }
    }
}

fn main() {
    // Place to store latest mouse possition from mouse event
    let mut mouse_x: f64 = 0.0;
    let mut mouse_y: f64 = 0.0;
    // Set up our game board
    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    // Set up our window, duh
    let window: PistonWindow =
        WindowSettings::new("lifecraft", [(GRID_SIZE * CELL_SIZE) as u32; 2])
        .exit_on_esc(true).build().unwrap();

    // Our checkerboard initial values
    checkerboard(&mut grid);

    // For each window event e:
    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);
            for x in 0..GRID_SIZE {
                for y in 0..GRID_SIZE {
                    if grid[x][y] > 0 {
                        let x_position: f64 = (x * CELL_SIZE) as f64;
                        let y_position: f64 = (y * CELL_SIZE) as f64;
                        rectangle([1.0, 0.0, 0.0, 1.0], // red
                                  [x_position, y_position, CELL_SIZE as f64, CELL_SIZE as f64],
                                  c.transform, g);
                    }
                }
            }
        });

        if let Some(pos) = e.mouse_cursor_args() {
            mouse_x = pos[0];
            mouse_y = pos[1];
        }

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                println!("Pressed {:?}", button);
                println!("Mouse position x:{:?} y:{:?}", mouse_x, mouse_y);
            }
        }
    }
}
