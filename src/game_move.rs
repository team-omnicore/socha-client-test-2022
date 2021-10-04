#![allow(dead_code)]

use crate::bitboard::*;
use crate::piece::PieceType;
use crate::team::Team;
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

    pub fn bits(&self) -> Bitboard {
        let x = self.origin.x + self.vector.x;
        let y = self.origin.y + self.vector.y;
        crate::mask_from_coords!(x, y)
    }

    ///Make this nicer when you have the brains to do it.
    pub fn translate(&self, team: &Team) -> Move {
        let mut result_board = Bitboard::from(self.result);
        result_board = result_board.flip_horizontal();

        let mut origin_board = Bitboard::from(self.origin);
        origin_board = origin_board.flip_horizontal();

        match team {
            Team::ONE => {
                result_board = result_board.rotate90_clockwise();
                origin_board = origin_board.rotate90_clockwise();
            }
            Team::TWO => {
                result_board = result_board.rotate90_anti_clockwise();
                origin_board = origin_board.rotate90_anti_clockwise();
            }
        }

        let origin_coords = crate::coords!(origin_board
            .get_set_bits()
            .get(0)
            .expect("Why the fk can't I plot my vector"));
        let result_coords = crate::coords!(result_board
            .get_set_bits()
            .get(0)
            .expect("Why the fk can't I plot my vector"));

        let vector = result_coords - origin_coords;

        Move::new(origin_coords, vector, self.piece)
    }

    pub fn out_of_bounds(&self) -> bool {
        return self.result.x > 7 || self.result.y > 7 || self.result.x < 0 || self.result.y < 0;
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} moves from {} to {}",
            self.piece, self.origin, self.result
        )
    }
}
