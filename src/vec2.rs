#[derive(Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

impl Vec2 {
    /// Creates a new vector.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Add this vec2 to another
    pub fn add(&self, vec2: &Vec2) -> Vec2 {
        return Vec2::new(vec2.x + self.x, vec2.y + self.y);
    }

    /// Creates a new vector with both components initialized to the given value.
    pub fn both(value: i32) -> Self {
        Self::new(value, value)
    }

    /// Finds the minimum with another point.
    pub fn min(self, other: Vec2) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// Finds the maximum with another point.
    pub fn max(self, other: Vec2) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }

}


