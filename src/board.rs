use crate::piece::Piece;

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
}