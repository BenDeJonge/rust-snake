// A simple Block struct, combining an x- and y-coordinate. Will not be exported so not pub.
// It is required to derive copy and clone allow movement of this type.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

impl Block {
    /// Instantiates a new Block.
    /// # Arguments
    /// * `x: i32` - The Blocks x-coordinate.
    /// * `y: i32` - The Blocks y-coordinate.
    /// # Returns
    /// * `Block` - The new Block instance.
    pub fn new(x: i32, y: i32) -> Block {
        Block { x, y }
    }

    /// Check whether this block falls within given bounds.
    /// # Arguments
    /// * `x: [i32; 2]` - The x-bounds as [lower, higher].
    /// * `y: [i32; 2]` - The y-bounds as [lower, higher].
    /// # Returns
    /// * `bool` - Whether (true) or not (false) the Block falls within the bounds.
    pub fn out_of_bounds(&self, x_bounds: [i32; 2], y_bounds: [i32; 2]) -> bool {
        self.x <= x_bounds[0]
            || self.x >= x_bounds[1] - 1
            || self.y <= y_bounds[0]
            || self.y >= y_bounds[1] - 1
    }
}
