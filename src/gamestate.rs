#![macro_use]

use crate::board::Board;
use crate::nibble::Nibble;
use crate::piece::PieceType;
use crate::pos_from_coords;
use crate::r#move::Move;
use rand::Rng;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::thread::sleep;

#[derive(Debug, Copy, Clone)]
pub struct Gamestate {
    pub points: Nibble, //Team 0 is left, Team 1 is right
    pub board: Board,
    pub round: u8,
}

pub static mut COUNT: u64 = 0;
pub static mut OTHER_COUNT: u64 = 0;

impl Gamestate {
    pub const fn new() -> Self {
        Gamestate {
            points: Nibble::new(),
            board: Board::new(),
            round: 0,
        }
    }

    pub fn simulate_dumb(&mut self, turns: u8) {
        if turns == 0 || self.points.get_left() == 2 || self.points.get_right() == 2 {
            //println!("{}", self);
            return;
        }

        let legal = self.board.calculate_legal();

        unsafe {
            COUNT += legal.len() as u64;
        }

        for my_move in legal {
            //println!("{}", my_move);
            let mut clone = Self::clone(self);
            clone.apply(&my_move);
            clone.board.switch_sides();
            clone.simulate_dumb(turns - 1);
        }
    }

    pub fn alpha_beta(
        &mut self,
        depth: u8,
        mut alpha: f32,
        mut beta: f32,
        maximizing_player: bool,
    ) -> f32 {
        if depth == 0 || self.is_win() {
            return self.eval();
        }

        let legal_moves = self.board.calculate_legal();

        return if maximizing_player {
            let mut max_eval = f32::NEG_INFINITY;
            for game_move in legal_moves {
                let mut clone = Self::clone(self);
                clone.apply(&game_move);
                clone.board.switch_sides();
                let eval = clone.alpha_beta(depth - 1, alpha, beta, false);
                max_eval = f32::max(max_eval, eval);
                alpha = f32::max(alpha, eval);
                if beta <= alpha {
                    break;
                }
            }
            max_eval
        } else {
            let mut min_eval = f32::INFINITY;
            for game_move in legal_moves {
                let mut clone = Self::clone(self);
                clone.apply(&game_move);
                clone.board.switch_sides();
                let eval = clone.alpha_beta(depth - 1, alpha, beta, true);
                min_eval = f32::min(min_eval, eval);
                beta = f32::min(alpha, eval);
                if beta <= alpha {
                    break;
                }
            }
            min_eval
        };
    }

    /// Eval function
    pub fn eval(&self) -> f32 {
        rand::thread_rng().gen()
    }

    /// Win condition
    fn is_win(&self) -> bool {
        self.points.get_left() == 2 || self.points.get_right() == 2
    }

    pub fn apply(&mut self, r#move: &Move) {
        //We know that the move is legal, now apply it to the board

        let origin = pos_from_coords!(r#move.origin.x as u8, r#move.origin.y as u8) as u8;

        //Clear origin position of data
        match r#move.piece {
            PieceType::ROBBE => self.board.robben.clear(origin),
            PieceType::MUSCHEL => self.board.muscheln.clear(origin),
            PieceType::SEESTERN => self.board.seesterne.clear(origin),
            PieceType::MOEWE => self.board.moewen.clear(origin),
        }

        self.board.friendly_pieces.clear(origin);
        self.board.double_stack.clear(origin);

        ////////////////////////////////////////////////////////

        let set_move = r#move.origin + r#move.vector;

        let pos = pos_from_coords!(set_move.x as u8, set_move.y as u8) as u8;

        //If the piece is stacked double, increase the count and remove
        //it from the registers and remove any enemy that was there. Since
        //the piece gets removed from the board, we don't need to add it.
        if self.board.double_stack.get(pos) {
            self.increase_ambers(self.round % 2);
            self.board.double_stack.clear(pos);
            self.board.enemy_pieces.clear(pos);
        } else {
            //If the piece is not stacked double, we check whether there
            //even is a piece at the position. If there is a piece at the
            //position we need to remove it,and replace it with our own
            if self.board.enemy_pieces.get(pos) {
                self.board.double_stack.set(pos);

                //Remove the enemy piece
                self.board.enemy_pieces.clear(pos);

                //Remove the enemy piece from the registers
                self.board.seesterne.clear(pos);
                self.board.muscheln.clear(pos);
                self.board.moewen.clear(pos);
                self.board.robben.clear(pos);
            }

            //Board at given position is free - we can place our piece
            self.board.friendly_pieces.set(pos);

            //Place the piece
            match r#move.piece {
                PieceType::ROBBE => self.board.robben.set(pos),
                PieceType::MUSCHEL => self.board.muscheln.set(pos),
                PieceType::SEESTERN => self.board.seesterne.set(pos),
                PieceType::MOEWE => self.board.moewen.set(pos),
            }
        }
    }

    fn increase_ambers(&mut self, team: u8) {
        match team {
            0 => self.points.set_left(self.points.get_left() + 1),
            1 => self.points.set_right(self.points.get_right() + 1),
            _ => {}
        }
    }
}

impl Display for Gamestate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}
