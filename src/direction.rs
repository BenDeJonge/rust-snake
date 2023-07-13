// Create a Direction enum, acting as a generic type holding all 4 possible directions.
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right    
}

impl Direction {
    /// Returns the opposite direction from the current.
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up    => Direction::Down,
            Direction::Down  => Direction::Up,
            Direction::Left  => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}