use std::collections::HashMap;

// Create a Direction enum, acting as a generic type holding all 4 possible directions.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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

    pub fn offsets() -> HashMap<Direction, [i32;2]> {
        HashMap::from([
                (Direction::Up, [0, -1]),
                (Direction::Down, [0, 1]),
                (Direction::Left, [-1, 0]),
                (Direction::Right, [1, 0]),
            ])
    }
}