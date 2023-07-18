// External imports.
use std::collections::VecDeque;
use piston_window::{Context, G2d};
use piston_window::types::Color;
// Importing local modules from the crate root.
use crate::draw::draw_block;
use crate::block::Block;
use crate::direction::Direction;

const SNAKE_HEAD_COLOR: Color = [0.00, 0.60, 0.00, 1.00];
const SNAKE_BODY_COLOR: Color = [0.00, 0.80, 0.00, 1.00];

const SNAKE_STARTING_LENGTH: i32 = 3;

pub struct Snake {
    /// The current and next direction in which the snake is travelling.
    current_direction: Direction,
    /// The (x,y) coordinate of the tail Block. 
    /// When eating food, the snake gets elongated by the tail block, resulting in a Block.
    /// During all other moves, the tail is not present, resulting in a None.
    tail: Option<Block>,
    /// The (x,y) coordinates of all body Blocks.
    body: VecDeque<Block>,
}


impl Snake {
    /// Instantiate a new Snake.
    /// # Arguments
    /// * `x: i32` - The x-coordinate of the head.
    /// * `y: i32` - The y-coordinate of the head.
    /// * `length: Option<i32>` - The initial length of the Snake, by default 3.
    /// * `direction: Option<Direction>` - The initial direction of the Snake, by default Direction::Right.
    /// # Returns
    /// * `Snake` - The new Snake instance.
    pub fn new(x: i32, y:i32, length: Option<i32>, direction: Option<Direction>) -> Snake {
        // Getting offsets for direction
        let (dx, dy) = match direction {
            Some(Direction::Up) => (0, -1),
            Some(Direction::Down) => (0, 1),
            Some(Direction::Left) => (-1, 0),
            _ => (1, 0),
        };
        
        // Creating a body.
        let mut body = VecDeque::new();
        let length = length.unwrap_or(SNAKE_STARTING_LENGTH);
        for _ in (0..length).rev() {
            body.push_back(Block {
                x: x + dx,
                y: y + dy,
            })
        }
        // Completing the Snake struct with a direction and absent tail.
        Snake {
            current_direction: direction.unwrap_or(Direction::Right),
            body,
            tail: None,
        }

    }


    /// Get the length of the Snake body VecDeque.
    pub fn len(&self) -> i32 {
        self.body.len() as i32
    }


    /// Draw all blocks in the Snakes body inside the context using the graphics engine.
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        let mut counter = 0;
        for block in &self.body {
            if counter > 0 {
                draw_block(*block, SNAKE_BODY_COLOR, con, g)
            } else {
                draw_block(*block, SNAKE_HEAD_COLOR, con, g)
            }
            counter += 1
        }
    }


    /// Find the head position of the snake.
    pub fn head_position(&self) -> Block {
        *self.body.front().unwrap()
    }


    /// Get the direction the head is moving in.
    pub fn head_direction(&self) -> Direction {
        self.current_direction
    }


    /// Move the Snake forward in the current direction.
    /// This method modifies the Snakes body, so requires a mutable reference to self.
    /// # Arguments
    /// * `direction: Option<Direction>` - Receives an optional Direction, depending on whether or not a key was pressed.
    pub fn move_forward(&mut self, direction: Option<Direction>) {
        // Unwrap the optional direction input.
        match direction {
            Some(dir) => self.current_direction = dir,
            None => (),
        }
        // Get the location of the new block based on the head position and the direction.
        // Note the required comma after each new match statement.
        let head = self.head_position();
        let new_block = match self.current_direction {
            Direction::Up => Block {
                x: head.x,
                y: head.y - 1,
            },
            Direction::Down => Block {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => Block {
                x: head.x - 1,
                y: head.y,
            },
            Direction::Right => Block {
                x: head.x + 1,
                y: head.y,
            },
        };
        // Push the new block into the body of the tail and remove the last block, mimicking movement.
        self.body.push_front(new_block);
        self.tail = Some(self.body.pop_back().unwrap());
    }


    /// Get the next head position based on the movement direction.
    /// # Arguments
    /// * `direction: Option<Direction>` - The movement direction, is None when no input is given.
    /// # Returns
    /// * `Block` - The next position of the Snakes head.
    pub fn next_head(&self, direction: Option<Direction>) -> Block {
        let head = self.head_position();
        // Keep heading in the current direction if no input is given.
        let moving_direction = match direction {
            Some(dir) => dir,
            None => self.current_direction,
        };
        // Update the coordinate of the head.
        match moving_direction {
            Direction::Up => Block{ x: head.x, y: head.y - 1 },
            Direction::Down => Block{ x: head.x, y:head.y + 1 },
            Direction::Left => Block{ x: head.x - 1, y:head.y },
            Direction::Right => Block { x: head.x + 1, y: head.y },
        }
    }


    /// Add the tail block when the snake has eaten food.
    pub fn restore_tail(&mut self) {
        self.body.push_back(self.tail.clone().unwrap())
    }


    /// Check if a block overlaps with the Snake body.
    /// # Arguments
    /// * `block: Block` - The block to check overlap for.
    /// # Returns
    /// * `bool` - Whether (true) or not (false) this block overlaps.
    pub fn overlap_tail(&self, block: Block) -> bool {
        // VecDeque does not support slicing of the back, which would be more convenient for .contains.
        let mut counter = 0;
        for body_part in &self.body {
            // Checking if the overlapping part could be the tail, which is ok as it will move anyway.
            counter += 1;
            if counter == self.body.len() {
                break;
            }
            // The overlapping bodypart is not the tail.
            else if *body_part == block {
                return true;
            }
        }
        false
    }

}