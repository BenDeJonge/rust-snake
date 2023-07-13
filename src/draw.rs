// External imports.
use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

use crate::snake::Block;

// Setting up a constant for the block size in pixels.
const BLOCK_SIZE: f64 = 25.0;

/// Convert game coordinates to pixel values.
/// # Arguments
/// * `game_coord: f64` - The game coordinate to be converted to a pixel value.
/// # Returns
/// * `f64` - The game coordinate as a pixel value.
pub fn to_pixels(game_coord: i32) -> f64 {
    return (game_coord as f64) * BLOCK_SIZE;
}

/// Draw a single block in the context.
/// # Arguments
/// * `color: piston_window::types::Color` - The color of the rectangle.
/// * `x: i32` - The x coordinate in game coordinates.
/// * `y: i32` - The y coordinate in game coordinates.
/// * `con`: &piston_window::Context - A reference to the games context.
/// * `g`: &mut piston_window::G2d - A mutable reference to the graphics engine used for drawing.
pub fn draw_block(block: Block, color: Color, con: &Context, g: &mut G2d) {
    // Conversion to pixels.
    let gui_x = to_pixels(block.x);
    let gui_y = to_pixels(block.y);
    // Instantiating a rectangle in the context, as supported by the graphics engine.
    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    )
}

/// Draw a rectangle composed of blocks in the context.
/// # Arguments
/// * `color: piston_window::types::Color` - The color of the rectangle.
/// * `x: i32` - The x coordinate in game coordinates.
/// * `y: i32` - The y coordinate in game coordinates.
/// * `width: i32` - The width of the rectangle in blocks.
/// * `height: i32` - The height of the rectangle in blocks.
/// * `con: &piston_window::Context` - A reference to the games context.
/// * `g: &mut piston_window::G2d` - A mutable reference to the graphics engine used for drawing.
pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_pixels(x);
    let gui_y = to_pixels(y);
    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE*(width as f64), BLOCK_SIZE*(height as f64)],
        con.transform,
        g,
    )
}