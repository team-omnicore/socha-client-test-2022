#![macro_use]

use crate::board::Board;
use crate::nibble::Nibble;
use crate::piece::PieceType;
use crate::pos_from_coords;
use crate::r#move::Move;
use crate::xml_node::XmlNode;
use rand::Rng;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::thread::sleep;

#[derive(Debug, Copy, Clone)]
pub struct Gamestate {
    pub points: Nibble, //Team 0 is left, Team 1 is right
    pub board: Board,
    pub round: u8,
}

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
            return;
        }

        let legal = self.board.calculate_legal();

        for my_move in legal {
            let mut clone = Self::clone(self);
            clone.board.apply(&my_move);
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
                clone.board.apply(&game_move);
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
                clone.board.apply(&game_move);
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
}

impl Display for Gamestate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}

impl From<&XmlNode> for Gamestate {
    fn from(node: &XmlNode) -> Self {
        let mut gamestate = Gamestate::new();
        let start_team = &node.child("startTeam").unwrap().text;
        let turn: u8 = node.attributes.get("turn").unwrap().parse().unwrap(); //Nach höchstens 30 Zügen ist das spiel um, also brauchen wir nicht mehr speichern
        gamestate.round = turn;

        let board_node = node.child("board").unwrap();
        gamestate.board = Board::from(board_node);

        gamestate
    }
}
