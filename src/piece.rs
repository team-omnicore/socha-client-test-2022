#![allow(dead_code)]

use crate::bitboard::Bitmask;
use crate::board::Board;
use crate::r#move::Move;
use crate::vec2::Vec2;
use std::fmt::{Display, Formatter};

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
        "Muschel",
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

    pub fn calculate_moves(&self, piece_positions: Bitmask, board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for position in piece_positions.get_set_bits() {
            let origin = Vec2::from_pos(position);

            for vector in self.vectors.iter() {
                let r#move = Move::new(origin, *vector, *self.typ);

                //Move not legal
                if r#move.out_of_bounds()
                    || board
                        .friendly_pieces
                        .get_at_coords(r#move.lands_at.x as u8, r#move.lands_at.y as u8)
                {
                    continue;
                }

                moves.push(r#move);
            }
        }
        mov
    }
}

impl PieceType {
    pub fn piece_type_from_name(name: &String) -> Option<PieceType> {
        return match name.as_str().unwrap() {
            "Moewe" => Some(PieceType::MOEWE),
            "Robbe" => Some(PieceType::ROBBE),
            "Muschel" => Some(PieceType::MUSCHEL),
            "Seestern" => Some(PieceType::SEESTERN),
            _ => None,
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
