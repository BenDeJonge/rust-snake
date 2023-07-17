// Loading in local modules. Also provides linting in those files.
mod draw;
mod snake;
mod game;
mod block;
mod direction;
mod food;


use piston_window::{WindowSettings, PistonWindow, Button, PressEvent, clear, UpdateEvent};
use piston_window::types::Color;

use draw::to_pixels;
use game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    // Creating a PistonWindow.
    let (width, height) = (20, 20);
    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [to_pixels(width) as u32, to_pixels(height) as u32]
    ).exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new(width, height);
    // Clearing the window.
    while let Some(event) = window.next() {
        // Catching game events corresponding to button presses. Handling in-game logic.
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        // Passing _ as OpenGL Device.
        window.draw_2d(&event, |con, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&con, g);
        });
        // Update event with anonymous function closure.
        event.update(|arg| {
            game.update(arg.dt)
        });
    }
}
