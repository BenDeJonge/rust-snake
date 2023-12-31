// External imports.
use piston_window::text;
use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d, Glyphs, Transformed};

// Local imports.
use crate::block::Block;
use crate::dateformat;
use crate::score;

// Setting up a constant for the block size in pixels.
pub const BLOCK_SIZE: f64 = 25.0;
pub const SNAKE_BLOCK_SIZE: f64 = 20.0;

/// Convert game coordinates to pixel values.
/// # Arguments
/// * `game_coord: f64` - The game coordinate to be converted to a pixel value.
/// # Returns
/// * `f64` - The game coordinate as a pixel value.
pub fn to_pixels(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

/// Draw a single block in the context.
/// # Arguments
/// * `color: piston_window::types::Color` - The color of the rectangle.
/// * `x: i32` - The x coordinate in game coordinates.
/// * `y: i32` - The y coordinate in game coordinates.
/// * `con`: &piston_window::Context - A reference to the games context.
/// * `g`: &mut piston_window::G2d - A mutable reference to the graphics engine used for drawing.
pub fn draw_block(
    block: Block,
    color: Color,
    offset: [f64; 2],
    size: [f64; 2],
    con: &Context,
    g: &mut G2d,
) {
    // Conversion to pixels.
    let gui_x = to_pixels(block.x) + offset[0];
    let gui_y = to_pixels(block.y) + offset[1];
    // Instantiating a rectangle in the context, as supported by the graphics engine.
    rectangle(color, [gui_x, gui_y, size[0], size[1]], con.transform, g)
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
pub fn draw_rectangle(
    color: Color,
    top_left: Block,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let gui_x = to_pixels(top_left.x);
    let gui_y = to_pixels(top_left.y);
    rectangle(
        color,
        [
            gui_x,
            gui_y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    )
}

/// Draw a string in the context.
/// # Arguments
/// * `text: &str` - The string to draw.
/// * `x: i32` - The x coordinate in game coordinates.
/// * `y: i32` - The y coordinate in game coordinates.
/// * `color: piston_window::Color` - The text color.
/// * `font_size: u32` - The text size.
/// * `glyphs: &mut piston_window::Glyphs` - The characterset to use.
/// * `con: &piston_window::Context` - A refrence to the games context.
/// * `g: &mut piston_window::G2d` - A mutable reference to the graphics engine used for drawing.
pub fn draw_text(
    text: &str,
    top_left: Block,
    color: Color,
    font_size: u32,
    glyphs: &mut Glyphs,
    con: &Context,
    g: &mut G2d,
) {
    for (i_line, line) in text.split('\n').enumerate() {
        let gui_x = to_pixels(top_left.x);
        let gui_y = to_pixels(top_left.y) + (font_size * (i_line + 1) as u32) as f64 * 1.1;
        text::Text::new_color(color, font_size)
            .draw(
                line,
                glyphs,
                &con.draw_state,
                con.transform.trans(gui_x, gui_y),
                g,
            )
            .unwrap();
    }
}

/// Display the current highscores.
/// # Arguments
/// * `scores: &[score::Score]` - A slice of the current highscore Vec.
/// * `top_left: Block` - The location of the top left corner of the text block.
/// * `color: piston_window::Color` - The text color.
/// * `font_size: u32` - The text size.
/// * `glyphs: &mut piston_window::Glyphs` - The characterset to use.
/// * `con: &piston_window::Context` - A refrence to the games context.
/// * `g: &mut piston_window::G2d` - A mutable reference to the graphics engine used for drawing.
pub fn show_scores(
    scores: &[score::Score],
    top_left: Block,
    color: Color,
    font_size: u32,
    glyphs: &mut Glyphs,
    con: &Context,
    g: &mut G2d,
) {
    let name_len = score::MAX_NAME_LENGTH;
    let mut text = String::new();
    for rank in 0..score::NUMBER_HIGH_SCORES {
        let score = scores.get(rank).unwrap();
        text.push_str(&format!(
            "{:2}. {:3} {:name_len$} {:19}\n",
            rank + 1,
            score.score(),
            score.player(),
            score.timestamp().format(dateformat::DISPLAY_FORMAT)
        ));
    }
    draw_text(&text, top_left, color, font_size, glyphs, con, g);
}

fn _get_offset_size(delta: i32) -> [f64; 2] {
    let shift = (BLOCK_SIZE - SNAKE_BLOCK_SIZE) / 2.0;
    match delta {
        0 => [shift, SNAKE_BLOCK_SIZE],
        1 => [-shift, BLOCK_SIZE],
        -1 => [shift, BLOCK_SIZE],
        _ => [0.0, BLOCK_SIZE],
    }
}

/// Get the offsets and sizes to connect two non-digestion blocks.
/// # Arguments
/// * `current: Block` - The current non-digestion block, closer to the tail.
/// * `previous: Block` - The previous non-digestion block, closer to the head.
/// # Returns
/// * `[f64; 2]` - The offset and size along x.
/// * `[f64; 2]` - The offset and size along y.
pub fn get_offset_size_regular(current: Block, previous: Block) -> ([f64; 2], [f64; 2]) {
    (
        _get_offset_size(current.x - previous.x),
        _get_offset_size(current.y - previous.y),
    )
}

/// Get the offsets and sizes to connect a non-digestion block to a digestion-block.
/// # Arguments
/// * `current: Block` - The current non-digestion block, closer to the tail.
/// * `previous: Block` - The previous non-digestion block, closer to the head.
/// * `next: Block` - The next digestion block, even closer to the tail.
/// # Returns
/// * `[f64; 2]` - The offset and size along x.
/// * `[f64; 2]` - The offset and size along y.
pub fn get_offset_size_digesting(
    current: Block,
    previous: Block,
    next: Block,
) -> ([f64; 2], [f64; 2]) {
    let (mut x_offset_size, mut y_offset_size) = get_offset_size_regular(current, previous);
    let shift = (BLOCK_SIZE - SNAKE_BLOCK_SIZE) / 2.0;

    match current.x - next.x {
        -1 => x_offset_size[1] += shift,
        1 => {
            x_offset_size[0] -= shift;
            x_offset_size[1] += shift;
        }
        _ => match current.y - next.y {
            1 => {
                y_offset_size[0] -= shift;
                y_offset_size[1] += shift;
            }
            -1 => y_offset_size[1] += shift,
            _ => (),
        },
    }

    (x_offset_size, y_offset_size)
}
