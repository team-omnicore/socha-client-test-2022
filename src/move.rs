#![allow(dead_code)]

use crate::bitboard::*;
use crate::piece::PieceType;
use crate::vec2::*;
use std::fmt::{Display, Formatter};

/// WARNING! Potential bugs with lands_at, because it is not synchronised with Move.
/// As long as move stays immutable everything is fine.
#[derive(Clone, Debug, Copy)]
pub struct Move {
    pub origin: Vec2,
    pub vector: Vec2,
    pub piece: PieceType,
    pub result: Vec2,
}

impl Move {
    pub fn new(origin: Vec2, vector: Vec2, piece: PieceType) -> Self {
        Move {
            origin,
            vector,
            piece,
            result: origin + vector,
        }
    }

    pub fn bits(&self) -> Bitmask {
        let x = self.origin.x + self.vector.x;
        let y = self.origin.y + self.vector.y;
        crate::mask_from_coords!(x, y)
    }

    pub fn out_of_bounds(&self) -> bool {
        return self.result.x > 7 || self.result.y > 7 || self.result.x < 0 || self.result.y < 0;
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} moves to {}", self.piece, self.origin + self.vector)
    }
}
