use crate::piece::Piece;
use crate::r#move::Move;

pub struct Board {
    pub one_pieces: Vec<Piece>,
    pub two_pieces: Vec<Piece>
}

impl Board {
    pub fn new() -> Self {
        Self{
            one_pieces: Vec::new(),
            two_pieces: Vec::new(),
        }
    }

    pub fn two_possible_moves(&self) -> Vec<Move> {
        let mut possible_moves : Vec<Move> = Vec::new();
        for piece in &self.two_pieces {
            for m in piece.possible_moves() {
                let mut valid = true;
                for piece in &self.two_pieces {
                    if piece.position.x == m.to.x && piece.position.y == m.to.y {
                        valid = false;
                    }
                }
                if m.to.x < 0 || m.to.x > 7 ||  m.to.y < 0 || m.to.y > 7 {
                    valid = false;
                }
                if valid {
                    possible_moves.push(m);
                }
            }
        }
        return possible_moves;
    }
}