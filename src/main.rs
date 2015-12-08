extern crate piston_window;
mod game;

use piston_window::*;
use game::{BOARD_SIZE, CELL_SIZE, GameOfLife};

fn main() {

    let window: PistonWindow =
        WindowSettings::new("lifecraft", [(BOARD_SIZE * CELL_SIZE) as u32; 2])
        .exit_on_esc(true).build().unwrap();

    let mut game = GameOfLife::new();

    let updates = 15;
    for e in window.ups(updates).max_fps(updates) {
        e.draw_2d(|c, g| {
            println!("Redraw");
            if game.run {
                game.game_step();
            }
            clear([1.0; 4], g);
            for x in 0..BOARD_SIZE {
                for y in 0..BOARD_SIZE {
                    if game.board[x][y] {
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
            game.mouse_pos = pos;
        }
        // Handle buttons
        if let Some(button) = e.press_args() {
            // Toggle cell state under cursor.
            if button == Button::Mouse(MouseButton::Left) {
                game.toggle_cell();
            }
            // Play/Pause the game.
            if button == Button::Keyboard(Key::Space) {
                println!("Pressed Space");
                game.run = !game.run;
            }
        }
    }
}
