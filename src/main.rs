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

    // Loading text assets.
    let assets = find_folder::Search::ParentsThenKids(3,3)
                 .for_folder("assets")
                 .unwrap();
    let ref font = assets.join("retro-gaming.ttf");
    let mut glyphs = window.load_font(font).unwrap();

    // Starting the main loop.
    let mut game = Game::new(width, height, None, None);
    while let Some(event) = window.next() {
        // Catching game events corresponding to button presses. Handling in-game logic.
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        // Passing _ as OpenGL Device.
        window.draw_2d(&event, |con, g, device| {
            // Clearing the window abd drawing a new one.
            clear(BACK_COLOR, g);
            game.draw(&mut glyphs, &con, g);
            // Clearing the glyphs buffer at the end of the frame drawing.
            glyphs.factory.encoder.flush(device);
        });
        // Update event with anonymous function closure.
        event.update(|arg| {
            game.update(arg.dt)
        });
    }
}