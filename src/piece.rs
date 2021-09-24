use crate::vec2::Vec2;
use crate::r#move::Move;
use std::borrow::Borrow;

#[derive(Clone)]
pub struct Piece {
    pub position: Vec2,
    name: String,
    move_vec: Vec<Vec2>
}

impl Piece {
    pub fn new(position: Vec2, name: &str) -> Self{
        Self {
            position,
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

    pub fn possible_moves (&self) -> Vec<Move> {
        let mut possible_moves: Vec<Move> = Vec::new();
        for m in &self.move_vec {
            let possible_move_vec : Vec2 = self.position.add(m);
            possible_moves.push(Move::new(self.position.clone(), possible_move_vec));

        }
        return possible_moves;
    }
}