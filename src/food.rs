use crate::block::Block;
use crate::direction::Direction;
use crate::snake::Snake;

use rand::prelude::thread_rng;
use rand::prelude::SliceRandom;
use rand::Rng;

const FOOD_SPEED_INCREASE: i32 = 5;

/// Calculate the Euclidian distance between two Blocks.
/// # Arguments
/// * `block1: Block` - The first Block.
/// * `block2: Block` - The second Block.
/// # Returns
/// * `f64` - The Euclidian distance, in game coordinates.
pub fn get_distance(block1: Block, block2: Block) -> f64 {
    (((block1.x - block2.x).pow(2) + (block1.y - block2.y).pow(2)) as f64).sqrt()
}

/// Calculate the optimal offset to hide from the Snakes current head position.
/// # Arguments
/// * `block: Block` - The food Block that tries to escape.
/// * `snake: &Snake` - A reference to the Snake class from which the Block escapes.
/// * `x_bounds: [i32;2]` - The x-bounds of the level, in game coordinates.
/// * `y_bounds: [i32;2]` - The y-bounds of the level, in game coordinates.
/// # Returns
/// * `[i32;2]` - A random sample from the optimal escape offsets.
pub fn get_escape_offset(
    block: Block,
    snake: &Snake,
    x_bounds: [i32; 2],
    y_bounds: [i32; 2],
) -> [i32; 2] {
    let mut best_dist = get_distance(block, snake.head_position());
    let mut best_offsets: Vec<[i32; 2]> = vec![[0, 0]];

    for (_, offset) in Direction::offsets() {
        let destination = Block::new(block.x + offset[0], block.y + offset[1]);
        if destination.out_of_bounds(x_bounds, y_bounds) || snake.overlap_tail(destination) {
            continue;
        }
        let current_dist = get_distance(destination, snake.head_position());
        if current_dist > best_dist {
            best_dist = current_dist;
            best_offsets.clear();
            best_offsets.push(offset);
        } else if current_dist == best_dist {
            best_offsets.push(offset);
        }
    }

    // Choosing a random move out of all equivalent distances.
    let mut rng = thread_rng();
    best_offsets.choose(&mut rng).copied().unwrap()
}

/// Escape from the snake with some probability, dependent on the length of the snake.
/// # Arguments
/// * `block: Block` - The food Block that tries to escape.
/// * `snake: &Snake` - A reference to the Snake class from which the Block escapes.
/// * `x_bounds: [i32;2]` - The x-bounds of the level, in game coordinates.
/// * `y_bounds: [i32;2]` - The y-bounds of the level, in game coordinates.
/// # Returns
/// * `[i32;2]` - An optimal escape offset or `[0, 0]` if the food did not get lucky enough to move.
pub fn escape(block: Block, snake: &Snake, x_bounds: [i32; 2], y_bounds: [i32; 2]) -> [i32; 2] {
    let escape = get_escape_offset(block, snake, x_bounds, y_bounds);

    let area = (x_bounds[1] - x_bounds[0]) * (y_bounds[1] - y_bounds[0]);
    let weights = [(snake.len() * FOOD_SPEED_INCREASE).clamp(0, area), area];
    let escape_weight = thread_rng().gen_range(0..weights[1]);

    if escape_weight <= weights[0] {
        escape
    } else {
        [0, 0]
    }
}
