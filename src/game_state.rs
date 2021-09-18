use crate::board::Board;

pub struct GameState {
    pub room_id: String,
    pub board: Board,
    pub turn: u16,
}

impl GameState{
    pub fn new() -> Self{
        Self {
            room_id: String::new(),
            board: Board::new(),
            turn: 0,
        }
    }
}