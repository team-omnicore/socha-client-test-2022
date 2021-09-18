use crate::vec2::Vec2;

pub struct Piece {
    x: u8,
    y: u8,
    name: String,
    move_vec: Vec<Vec2>
}

impl Piece {
    pub fn new(x : u8, y : u8, name: &str) -> Self{
        Self {
            x,
            y,
            name: name.to_string(),
            move_vec: Self::move_vec_from_name(name),
        }
    }

    pub fn move_vec_from_name(name: &str) -> Vec<Vec2>{
        return match name {
            "Herzmuschel" => vec![Vec2::new(1, 1), Vec2::new(1, -1)],
            "Moewe" => vec![Vec2::new(-1, 0), Vec2::new(1, 0), Vec2::new(0, -1), Vec2::new(0, 1)],
            "Seestern" => vec![Vec2::new(-1, -1), Vec2::new(-1, 1), Vec2::new(1, -1), Vec2::new(1, 1), Vec2::new(0, 1)],
            "Robbe" => vec![Vec2::new(-2, -1), Vec2::new(-2, 1), Vec2::new(-1, -2), Vec2::new(-1, 2), Vec2::new(1, -2), Vec2::new(1, 2), Vec2::new(2, -1), Vec2::new(2, 1)],
            _ => vec![]
        }
    }
}