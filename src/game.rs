// External imports.
use piston_window::{Key, Context, G2d};
use piston_window::types::Color;
use rand::{thread_rng, Rng};
// Local imports.
use crate::snake::{Direction, Snake, Block};
use crate::draw::{draw_block, draw_rectangle};

// Constants.
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.00];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.00];
const BORDER_WIDTH: i32 = 1;
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.50];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;


pub struct Game {
    snake: Snake,
    food: Option<Block>,

    width: i32, 
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    /// Instantiate a new game.
    /// # Arguments
    /// * `width: i32` - The game window width in pixels.
    /// * `height: i32` - The game window height in pixels.
    /// # Returns
    /// * `Game` - The new Game instance.
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2, None, None),
            waiting_time: 0.0,
            food: Some(Block::new(6, 4)),
            width,
            height,
            game_over: false,
        }
    }

    /// React to a keypress.
    /// # Arguments
    /// * `piston_window::Key` - The key being pressed.
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return
        }

        // Associating all valid keys with the Some part of the Option and invalid ones with the None part.
        let direction = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction()),
        };
        
        // The snake cannot turn around.
        if direction.unwrap() == self.snake.head_direction().opposite() {
            return
        }
        self.update_snake(direction);
    }

    /// Move to the next position and ead food, stopping the game in case of a death.
    /// # Arguments
    /// * `direction: Option<Direction>` - The direction input.
    pub fn update_snake(&mut self, direction: Option<Direction>) {
        if self.check_snake_alive(direction) {
            self.snake.move_forward(direction);
            self.check_eaten();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    /// Draw all game elements: the snake, the borders and optional food and game over symbols.
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Drawing the snake and food.
        self.snake.draw(con, g);
        match self.food {
            Some(block) => draw_block(block, FOOD_COLOR, con, g),
            None => (),
        }

        // Drawing the top, bottom, left and right borders of the screen.
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, BORDER_WIDTH, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - BORDER_WIDTH, self.width, BORDER_WIDTH, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, BORDER_WIDTH, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - BORDER_WIDTH, 0, BORDER_WIDTH, self.height, con, g);

        // Drawing a game over text.
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    /// Move the game one tick.
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        
        // Restarting after some time.
        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }
        // Drawing food if not yet food.
        match self.food {
            Some(_) => (),
            None => self.add_food(),
        }
        // Moving if no input received.
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    /// Reset all the games attributes.
    pub fn restart(&mut self) {
        self.snake = Snake::new(2, 2, None, None);
        self.waiting_time = 0.0;
        self.food = Some(Block::new(6, 4));
        self.game_over = false;
    }

    /// Respawn food at a random location after a previous one has been eaten.
    pub fn add_food(&mut self) {
        // Spawn food at a random location.
        let mut rng = thread_rng();
        let mut food = Block::new(
            rng.gen_range(1..self.width - 1),
            rng.gen_range(1..self.height - 1));
        // Food cannot spawn on the snake.
        while self.snake.overlap_tail(food) {
            food = Block::new(
                rng.gen_range(1..self.width - 1),
                rng.gen_range(1..self.height - 1));
            }
        // Updating the food attribute, hence the mutable reference to self.
        self.food = Some(food);
    }


    /// Check if the snake has eaten food.
    pub fn check_eaten(&mut self) {
        // The head position coincides with the food.
        if self.snake.head_position() == self.food.unwrap() {
            self.food = None;
            self.snake.restore_tail();
        }
    }

    /// Check if the movement direction does not kill the snake.
    /// # Arguments
    /// * `direction: Option<Direction>` - The selected movement direction.
    /// # Returns
    /// * `bool` - Whether (true) or not (false) the snake survives the selected move.
    pub fn check_snake_alive(&self, direction: Option<Direction>) -> bool {
        let destination = self.snake.next_head(direction);
        !self.snake.overlap_tail(destination) && !destination.out_of_bounds([0, self.width - 1], [0, self.height - 1])
    }
}

// TODO:
// - Keypresses
// - Update Snake
// - Add food
// - Restart
// - Window
// - Clean Window