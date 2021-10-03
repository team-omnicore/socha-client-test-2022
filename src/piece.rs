#![allow(dead_code)]

use crate::bitboard::Bitboard;
use crate::board::Board;
use crate::game_move::Move;
use crate::vec2::Vec2;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::string::ParseError;

pub mod pieces {
    use crate::piece::{Piece, PieceType};
    use crate::vec2::Vec2;

    pub const ROBBE: Piece<8> = Piece::new(
        "Robbe",
        &PieceType::ROBBE,
        [
            Vec2::new(2, 1),
            Vec2::new(1, 2),
            Vec2::new(-1, 2),
            Vec2::new(-2, 1),
            Vec2::new(-2, -1),
            Vec2::new(-1, -2),
            Vec2::new(1, -2),
            Vec2::new(2, -1),
        ],
    );

    pub const MUSCHEL: Piece<2> = Piece::new(
        "Herzmuschel",
        &PieceType::MUSCHEL,
        [Vec2::new(1, 1), Vec2::new(-1, 1)],
    );

    pub const SEESTERN: Piece<5> = Piece::new(
        "Seestern",
        &PieceType::SEESTERN,
        [
            Vec2::new(1, 1),
            Vec2::new(0, 1),
            Vec2::new(-1, 1),
            Vec2::new(1, -1),
            Vec2::new(-1, -1),
        ],
    );

    pub const MOEWE: Piece<4> = Piece::new(
        "Möwe",
        &PieceType::MOEWE,
        [
            Vec2::new(1, 0),
            Vec2::new(0, 1),
            Vec2::new(-1, 0),
            Vec2::new(0, -1),
        ],
    );
}

#[derive(Clone, Debug, Copy)]
pub enum PieceType {
    ROBBE,
    MUSCHEL,
    SEESTERN,
    MOEWE,
}

#[derive(Debug, Copy, Clone)]
pub struct Piece<const MOVE_COUNT: usize> {
    pub name: &'static str,
    pub typ: &'static PieceType,
    pub vectors: [Vec2; MOVE_COUNT],
}

impl<const MOVE_COUNT: usize> Piece<MOVE_COUNT> {
    pub const fn new(
        name: &'static str,
        typ: &'static PieceType,
        vectors: [Vec2; MOVE_COUNT],
    ) -> Self {
        Piece {
            name,
            typ: &typ,
            vectors,
        }
    }

    pub fn calculate_moves(&self, piece_positions: Bitboard, board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for position in piece_positions.get_set_bits() {
            let origin = Vec2::from_pos(position);

            for vector in self.vectors.iter() {
                let r#move = Move::new(origin, *vector, *self.typ);

                //Move not legal
                if r#move.out_of_bounds()
                    || board
                        .friendly_pieces
                        .get_at_coords(r#move.result.x as u8, r#move.result.y as u8)
                {
                    continue;
                }

                moves.push(r#move);
            }
        }
        moves
    }
}

impl PieceType {
    pub fn piece_type_from_name(name: &String) -> Option<PieceType> {
        return match name.as_str() {
            "Moewe" => Some(PieceType::MOEWE),
            "Robbe" => Some(PieceType::ROBBE),
            "Herzmuschel" => Some(PieceType::MUSCHEL),
            "Seestern" => Some(PieceType::SEESTERN),
            _ => None,
        };
    }
}

impl From<&String> for PieceType {
    fn from(str: &String) -> Self {
        return match str.as_str() {
            "Moewe" => PieceType::MOEWE,
            "Robbe" => PieceType::ROBBE,
            "Herzmuschel" => PieceType::MUSCHEL,
            "Seestern" => PieceType::SEESTERN,
            piece => {
                panic!("No piece of type: {}", piece)
            }
        };
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceType::ROBBE => {
                write!(f, "{}", pieces::ROBBE.name)
            }
            PieceType::MUSCHEL => {
                write!(f, "{}", pieces::MUSCHEL.name)
            }
            PieceType::SEESTERN => {
                write!(f, "{}", pieces::SEESTERN.name)
            }
            PieceType::MOEWE => {
                write!(f, "{}", pieces::MOEWE.name)
            }
        }
    }
}

impl FromStr for PieceType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Moewe" => Ok(PieceType::MOEWE),
            "Robbe" => Ok(PieceType::ROBBE),
            "Herzmuschel" => Ok(PieceType::MUSCHEL),
            "Seestern" => Ok(PieceType::SEESTERN),
            piece => {
                panic!("No piece of type: {}", piece)
            }
        }
    }
}
