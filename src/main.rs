extern crate piston_window;
extern crate rand;
extern crate graphics;

use piston_window::*;
use rand::os::OsRng;

static grid_size: u32 = 10;
static cell_size: u32 = 10;

fn main() {
    // Set up our window, duh
    let window: PistonWindow =
        WindowSettings::new("Hello Piston!", [grid_size * cell_size; 2])
        .exit_on_esc(true).build().unwrap();
    // For each window event e:
    for e in window {
        // c is ?, g is ?
        let mut fill: bool = true;
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
                    fill = !fill;
                    println!("fill: {}", fill);
                    if fill {
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
