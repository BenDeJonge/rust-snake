#![windows_subsystem = "windows"]

// Loading in local modules. Also provides linting in those files.
mod block;
mod dateformat;
mod direction;
mod draw;
mod food;
mod game;
mod score;
mod snake;

use piston_window::types::Color;
use piston_window::{clear, Button, PistonWindow, PressEvent, UpdateEvent, WindowSettings};
use score::check_score;
use std::env;

use draw::to_pixels;
use game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const ASSETS_FOLDER: &str = "assets";
const ASSETS_FONT_NAME: &str = "joystix.monospace-regular.otf";
const ASSETS_SCORE_NAME: &str = "scores.json";

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // Creating a PistonWindow.
    let (width, height) = (20, 20);
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_pixels(width) as u32, to_pixels(height) as u32])
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Loading text assets.
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder(ASSETS_FOLDER)
        .unwrap();
    let font = &assets.join(ASSETS_FONT_NAME);
    let mut glyphs = window.load_font(font).unwrap();

    // Loading current high-scores
    let scores_file = &assets.join(ASSETS_SCORE_NAME);
    let mut scores = score::parse_scores(scores_file);
    // Starting the main loop.
    let mut game = Game::new(width, height, None, None);
    while let Some(event) = window.next() {
        // Checking if this score beats any other.
        if game.game_over() && !game.high_score {
            game.high_score = check_score(game.score(), &scores).is_some();
        }
        // Catching game events corresponding to button presses. Handling in-game logic.
        if let Some(Button::Keyboard(k)) = event.press_args() {
            game.key_pressed(k);
            game.ask_name(k, &mut scores, scores_file);
        };
        // Passing _ as OpenGL Device.
        window.draw_2d(&event, |con, g, device| {
            // Clearing the window abd drawing a new one.
            clear(BACK_COLOR, g);
            game.draw(
                //&scores,
                &mut glyphs,
                &con,
                g,
                &scores,
            );
            // Clearing the glyphs buffer at the end of the frame drawing.
            glyphs.factory.encoder.flush(device);
        });
        // Update event with anonymous function closure.
        event.update(|arg| game.update(arg.dt));
    }
}
