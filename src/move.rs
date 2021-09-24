use crate::vec2::Vec2;

pub struct Move {
    pub(crate) from: Vec2,
    pub(crate) to: Vec2,
}

impl Move {
    pub fn new(from : Vec2, to: Vec2) -> Self {
        Self{
            from,
            to,
        }
    }
}