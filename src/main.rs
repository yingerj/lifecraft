extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::os::OsRng;

const grid_size: usize = 10;
const cell_size: usize = 10;


fn main() {
    // Set up our game board
    let mut grid: [[u8; grid_size]; grid_size] = [[0; grid_size]; grid_size];

    // Set up our window, duh
    let window: PistonWindow =
        WindowSettings::new("Hello Piston!", [(grid_size * cell_size) as u32; 2])
        .exit_on_esc(true).build().unwrap();

    // Our checkerboard initial values
    let mut fill: bool = true;
    for x in 0..grid_size {
        for y in 0..grid_size {
            fill = !fill;
            println!("fill: {}", fill);
            if fill {
                grid[x][y] = fill as u8;
            }
        }
        if grid_size % 2 == 0 {
            fill = !fill;
        }
    }

    // For each window event e:
    for e in window {
        e.draw_2d(|c, g| {
            println!("Redraw");
            clear([1.0; 4], g);
            for x in 0..grid_size {
                let x_min: f64 = (x * cell_size) as f64;
                let x_max: f64 = x_min + (cell_size as f64);
                println!("x min: {} max: {}", x_min, x_max);
                for y in 0..grid_size {
                    let y_min: f64 = (y * cell_size) as f64;
                    let y_max: f64 = y_min as f64 + (cell_size as f64);
                    println!("   y min: {} max: {}", y_min, y_max);
                    println!("fill: {}", fill);
                    if grid[x][y] > 0 {
                        rectangle([1.0, 0.0, 0.0, 1.0], // red
                                  [x_min, y_min, cell_size as f64, cell_size as f64],
                                  c.transform, g);
                    }
                }
                if grid_size % 2 == 0 {
                    fill = !fill;
                }
            }
        });
    }
}
