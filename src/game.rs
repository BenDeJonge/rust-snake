// External imports.
use piston_window::types::Color;
use piston_window::{Context, G2d, Glyphs, Key};
use rand::{thread_rng, Rng};
use std::path::PathBuf;

// Local imports.
use crate::block::Block;
use crate::direction::Direction;
use crate::draw::{draw_block, draw_rectangle, draw_text, show_scores, BLOCK_SIZE};
use crate::food;
use crate::score::{create_empty_name, write_score, Score, MAX_NAME_LENGTH};
use crate::snake::Snake;

// Constants.
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.00];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.00];
const BORDER_WIDTH: i32 = 1;
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.50];
const GAMEOVER_TEXT_COLOR: Color = [1.0, 1.0, 1.0, 0.9];
const SCORE_BORDER_WIDTH: i32 = 1;
const SCORE_FONT_SIZE: u32 = 20;
const MOVING_PERIOD: f64 = 0.5;
const FOOD_SPEED_INCREASE: i32 = 5;
const SPEED_FACTOR: f64 = 0.8;
const FOODS_PER_SPEED_INCREASE: i32 = 5;

struct Borders {
    top_border: Block,
    bottom_border: Block,
    left_border: Block,
    right_border: Block,
    score_border: Block,
    high_score_border: Block,
    score_name_border: Block,
}

pub struct Game {
    snake: Snake,
    food: Option<Block>,
    direction_queue: Vec<Option<Direction>>,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,

    score: i32,
    pub high_score: bool,
    pub score_written: bool,
    score_name: String,

    borders: Borders,
}

impl Game {
    /// Instantiate a new game.
    /// # Arguments
    /// * `width: i32` - The game window width in pixels.
    /// * `height: i32` - The game window height in pixels.
    /// # Returns
    /// * `Game` - The new Game instance.
    pub fn new(
        width: i32,
        height: i32,
        starting_length: Option<i32>,
        starting_direction: Option<Direction>,
    ) -> Game {
        Game {
            snake: Snake::new(2, 2, starting_length, starting_direction),
            waiting_time: 0.0,
            food: Some(Block::new(6, 4)),
            width,
            height: height - SCORE_BORDER_WIDTH,
            game_over: false,
            direction_queue: Vec::new(),
            score: 0,
            high_score: false,
            score_written: false,
            score_name: create_empty_name(),
            borders: Borders {
                top_border: Block::new(0, 0),
                bottom_border: Block::new(0, height - BORDER_WIDTH - SCORE_BORDER_WIDTH),
                left_border: Block::new(0, 0),
                right_border: Block::new(width - BORDER_WIDTH, 0),
                score_border: Block::new(0, height - BORDER_WIDTH),
                high_score_border: Block::new(BORDER_WIDTH, height / 2 + 1),
                score_name_border: Block::new(BORDER_WIDTH, height / 2 - 1),
            },
        }
    }

    /// React to a keypress.
    /// # Arguments
    /// * `piston_window::Key` - The key being pressed.
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            match key {
                Key::Space => self.restart(),
                _ => return,
            }
        };

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
            return;
        }
        self.direction_queue.push(direction);
    }

    /// Interact with the name entry field.
    /// * `key: piston_window::Key` - The key being pressed. Allows letter, backspace and enter.
    /// * `scores: &mut Vec<Score>` - The vector of Score structs to push the new score to.
    /// * `scores_file: &PathBuf` - The location of the score file to write the new scores to.
    pub fn ask_name(&mut self, key: Key, scores: &mut Vec<Score>, scores_file: &PathBuf) {
        if self.game_over && self.high_score && !self.score_written {
            if let Some(letter) = match key {
                // Valid letter.
                Key::A => Some('A'),
                Key::B => Some('B'),
                Key::C => Some('C'),
                Key::D => Some('D'),
                Key::E => Some('E'),
                Key::F => Some('F'),
                Key::G => Some('G'),
                Key::H => Some('H'),
                Key::I => Some('I'),
                Key::J => Some('J'),
                Key::K => Some('K'),
                Key::L => Some('L'),
                Key::M => Some('M'),
                Key::N => Some('N'),
                Key::O => Some('O'),
                Key::P => Some('P'),
                Key::Q => Some('Q'),
                Key::R => Some('R'),
                Key::S => Some('S'),
                Key::T => Some('T'),
                Key::U => Some('U'),
                Key::V => Some('V'),
                Key::W => Some('W'),
                Key::X => Some('X'),
                Key::Y => Some('Y'),
                Key::Z => Some('Z'),
                // Removing a letter from the name.
                Key::Backspace => {
                    self.score_name.pop();
                    None
                }
                // Accepting the name.
                Key::Return => {
                    write_score(scores, &self.score_name, self, scores_file);
                    self.score_written = true;
                    None
                }
                // Invalid key.
                _ => None,
            } {
                // Adding a letter if there is still room.
                if self.score_name.chars().count() < MAX_NAME_LENGTH {
                    self.score_name.push(letter)
                }
            }
        }
    }

    /// Move to the next position and ead food, stopping the game in case of a death.
    pub fn update_snake(&mut self) {
        let direction = match self.direction_queue.last() {
            Some(dir) => *dir,
            None => Some(self.snake.head_direction()),
        };
        if self.check_snake_alive(direction) {
            self.snake.move_forward(direction);
            self.check_eaten();
        } else {
            self.game_over = true;
        }
        // Resetting.
        self.waiting_time = 0.0;
        self.direction_queue.clear();
    }

    /// Move the food if not eaten yet and the game is not over.
    pub fn update_food(&mut self) {
        let speed = if self.game_over {
            0
        } else {
            FOOD_SPEED_INCREASE
        };
        if let Some(food) = self.food {
            let offset = food::escape(food, &self.snake, [0, self.width], [0, self.height], speed);
            self.food = Some(Block::new(food.x + offset[0], food.y + offset[1]))
        }
    }

    fn _draw_background(&self, con: &Context, g: &mut G2d) {
        // Drawing the top, bottom, left and right borders of the screen.

        draw_rectangle(
            BORDER_COLOR,
            self.borders.top_border,
            self.width,
            BORDER_WIDTH,
            con,
            g,
        );
        draw_rectangle(
            BORDER_COLOR,
            self.borders.bottom_border,
            self.width,
            BORDER_WIDTH,
            con,
            g,
        );
        draw_rectangle(
            BORDER_COLOR,
            self.borders.left_border,
            BORDER_WIDTH,
            self.height,
            con,
            g,
        );
        draw_rectangle(
            BORDER_COLOR,
            self.borders.right_border,
            BORDER_WIDTH,
            self.height,
            con,
            g,
        );

        // Drawing the score border.
        draw_rectangle(
            BORDER_COLOR,
            self.borders.score_border,
            self.width,
            SCORE_BORDER_WIDTH,
            con,
            g,
        );
    }

    fn _draw_score_text(&self, glyphs: &mut Glyphs, con: &Context, g: &mut G2d) {
        draw_text(
            &format!("SCORE: {}", self.score.to_string().as_str()),
            Block::new(SCORE_BORDER_WIDTH, self.height + SCORE_BORDER_WIDTH / 2),
            FOOD_COLOR,
            SCORE_FONT_SIZE,
            glyphs,
            con,
            g,
        );
    }

    fn _draw_speed_text(&self, glyphs: &mut Glyphs, con: &Context, g: &mut G2d) {
        draw_text(
            &format!(
                "SPEED: {}",
                (1 + self.score / FOODS_PER_SPEED_INCREASE)
                    .to_string()
                    .as_str()
            ),
            Block::new(
                self.width - 7 * SCORE_BORDER_WIDTH,
                self.height + SCORE_BORDER_WIDTH / 2,
            ),
            FOOD_COLOR,
            SCORE_FONT_SIZE,
            glyphs,
            con,
            g,
        );
    }
    fn _draw_game_over_screen(&self, glyphs: &mut Glyphs, con: &Context, g: &mut G2d) {
        draw_rectangle(
            GAMEOVER_COLOR,
            Block::new(SCORE_BORDER_WIDTH, BORDER_WIDTH),
            self.width - 2 * BORDER_WIDTH,
            self.height - BORDER_WIDTH - SCORE_BORDER_WIDTH,
            con,
            g,
        );
        let highscore = match self.high_score {
            true => " - HIGHSCORE",
            false => "",
        };
        draw_text(
            &format!("GAME OVER\n{}{}\n<SPACE> TO PLAY", self.score, highscore),
            Block::new(BORDER_WIDTH, BORDER_WIDTH),
            GAMEOVER_TEXT_COLOR,
            32,
            glyphs,
            con,
            g,
        );
    }

    fn _draw_scoreboard(&self, scores: &[Score], glyphs: &mut Glyphs, con: &Context, g: &mut G2d) {
        show_scores(
            scores,
            self.borders.high_score_border,
            GAMEOVER_TEXT_COLOR,
            15,
            glyphs,
            con,
            g,
        )
    }

    fn _draw_name_querry(&self, glyphs: &mut Glyphs, con: &Context, g: &mut G2d) {
        draw_text(
            &format!("Name: {}", &self.score_name),
            self.borders.score_name_border,
            GAMEOVER_TEXT_COLOR,
            SCORE_FONT_SIZE,
            glyphs,
            con,
            g,
        )
    }

    /// Draw all game elements: the snake, the borders, food, game over symbols and the score.
    /// # Arguments
    /// * `glyphs: &mut piston_window::Glyphs` - The characters to use for drawing.
    /// * `con: &piston_window::Context` - The context in which to draw.
    /// * `g: &mut G2d` - The 2d graphics driver to use.
    pub fn draw(
        &mut self,
        // key: Option<Key>,
        // scores: &HashMap<i32, Score>,
        glyphs: &mut Glyphs,
        con: &Context,
        g: &mut G2d,
        scores: &[Score],
    ) {
        // Drawing the snake and food.
        self.snake.draw(con, g);
        if let Some(food) = self.food {
            draw_block(
                food,
                FOOD_COLOR,
                [0.0, 0.0],
                [BLOCK_SIZE, BLOCK_SIZE],
                con,
                g,
            );
        };

        self._draw_background(con, g);
        self._draw_score_text(glyphs, con, g);
        self._draw_speed_text(glyphs, con, g);

        // Drawing a game over screen.
        if self.game_over {
            self._draw_game_over_screen(glyphs, con, g);
            self._draw_scoreboard(scores, glyphs, con, g)
        }

        if self.high_score {
            self._draw_name_querry(glyphs, con, g);
        }
    }

    /// Move the game one tick, checking for game over, food presence and drawing the snake.
    /// # Arguments
    /// * `delta_time: f64` - The timestep of the tick in seconds.
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        // Drawing food if not yet food.
        match self.food {
            Some(_) => (),
            None => self.add_food(),
        }
        // Moving after the moving period has passed.
        if self.waiting_time
            > MOVING_PERIOD * SPEED_FACTOR.powi(self.score / FOODS_PER_SPEED_INCREASE)
        {
            self.update_food();
            self.update_snake();
        }
    }

    /// Reset all the games attributes.
    pub fn restart(&mut self) {
        self.snake = Snake::new(2, 2, None, None);
        self.direction_queue = Vec::new();
        self.waiting_time = 0.0;
        self.food = Some(Block::new(6, 4));
        self.game_over = false;
        self.score = 0;
        self.high_score = false;
        self.score_written = false;
        self.score_name = create_empty_name();
    }

    /// Respawn food at a random location after a previous one has been eaten.
    pub fn add_food(&mut self) {
        // Spawn food at a random location.
        let mut rng = thread_rng();
        let mut food = Block::new(
            rng.gen_range(1..self.width - 1),
            rng.gen_range(1..self.height - 1),
        );
        // Food cannot spawn on the snake.
        while self.snake.overlap_tail(food) {
            food = Block::new(
                rng.gen_range(1..self.width - 1),
                rng.gen_range(1..self.height - 1),
            );
        }
        // Updating the food attribute, hence the mutable reference to self.
        self.food = Some(food);
    }

    /// Check if the snake has eaten food.
    pub fn check_eaten(&mut self) {
        // The head position coincides with the food.
        if self.snake.head_position() == self.food.unwrap() {
            self.snake
                .digesting
                .insert(self.food.unwrap(), self.snake.len());
            self.food = None;
            self.snake.restore_tail();
            self.score += 1;
        }
    }

    /// Check if the movement direction does not kill the snake.
    /// # Arguments
    /// * `direction: Option<Direction>` - The selected movement direction.
    /// # Returns
    /// * `bool` - Whether (true) or not (false) the snake survives the selected move.
    pub fn check_snake_alive(&self, direction: Option<Direction>) -> bool {
        let destination = self.snake.next_head(direction);
        !self.snake.overlap_tail(destination)
            && !destination.out_of_bounds([0, self.width], [0, self.height])
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}
