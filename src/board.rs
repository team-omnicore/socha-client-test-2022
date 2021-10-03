use crate::bitboard::Bitboard;
use crate::coords;
use crate::game_move::Move;
use crate::piece::pieces::*;
use crate::piece::PieceType;
use crate::position;
use crate::team::Team;
use crate::vec2::{Vec2};
use crate::xml_node::XmlNode;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Board {
    pub enemy_pieces: Bitboard,
    pub friendly_pieces: Bitboard,
    pub seesterne: Bitboard,
    pub muscheln: Bitboard,
    pub moewen: Bitboard,
    pub robben: Bitboard,
    pub double_stack: Bitboard,
}

impl Board {
    pub const fn new() -> Self {
        Board {
            enemy_pieces: Bitboard::new(),
            friendly_pieces: Bitboard::new(),
            seesterne: Bitboard::new(),
            muscheln: Bitboard::new(),
            moewen: Bitboard::new(),
            robben: Bitboard::new(),
            double_stack: Bitboard::new(),
        }
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let moewen = self.moewen & self.friendly_pieces;
        let robben = self.robben & self.friendly_pieces;
        let muscheln = self.muscheln & self.friendly_pieces;
        let seesterne = self.seesterne & self.friendly_pieces;

        let mut out = MOEWE.calculate_moves(moewen, self);
        out.append(&mut ROBBE.calculate_moves(robben, self));
        out.append(&mut MUSCHEL.calculate_moves(muscheln, self));
        out.append(&mut SEESTERN.calculate_moves(seesterne, self));

        return out;
    }

    pub fn apply(&mut self, r#move: &Move) -> u8 {
        //We know that the move is legal, now apply it to the board

        let origin = position!(r#move.origin.x as u8, r#move.origin.y as u8) as u8;

        //Clear origin position of data
        match r#move.piece {
            PieceType::ROBBE => self.robben.clear(origin),
            PieceType::MUSCHEL => self.muscheln.clear(origin),
            PieceType::SEESTERN => self.seesterne.clear(origin),
            PieceType::MOEWE => self.moewen.clear(origin),
        }

        self.friendly_pieces.clear(origin);
        self.double_stack.clear(origin);

        ////////////////////////////////////////////////////////

        let set_move = r#move.origin + r#move.vector;

        let pos = position!(set_move.x as u8, set_move.y as u8);

        //Points to increase game by
        let mut points = 0u8;

        //If the piece is stacked double, increase the count and remove
        //it from the registers and remove any enemy that was there. Since
        //the piece gets removed from the board, we don't need to add it.
        if self.double_stack.get(pos) {
            points = 1;
            self.double_stack.clear(pos);
            self.enemy_pieces.clear(pos);
        } else {
            //If the piece is not stacked double, we check whether there
            //even is a piece at the position. If there is a piece at the
            //position we need to remove it,and replace it with our own
            if self.enemy_pieces.get(pos) {
                self.double_stack.set(pos);

                //Remove the enemy piece
                self.enemy_pieces.clear(pos);

                //Remove the enemy piece from the registers
                self.seesterne.clear(pos);
                self.muscheln.clear(pos);
                self.moewen.clear(pos);
                self.robben.clear(pos);
            }

            //Board at given position is free - we can place our piece
            self.friendly_pieces.set(pos);

            //Place the piece
            match r#move.piece {
                PieceType::ROBBE => self.robben.set(pos),
                PieceType::MUSCHEL => self.muscheln.set(pos),
                PieceType::SEESTERN => self.seesterne.set(pos),
                PieceType::MOEWE => self.moewen.set(pos),
            }
        }
        return points;
    }


    ///Apply anonymous move. Maybe create an own struct?
    pub fn apply_anonymous(&mut self, origin_pos: u8, result_pos: u8) -> u8 {
        let origin = coords!(origin_pos);
        let result = coords!(result_pos);

        let vector = result - origin;
        let r#move = Move::new(
            origin,
            vector,
            self.piece_at(origin_pos).expect("What the fuck"),
        );

        return self.apply(&r#move);
    }

    pub fn rotate90_clockwise(&mut self) {
        self.friendly_pieces = self.friendly_pieces.rotate90_clockwise();
        self.enemy_pieces = self.enemy_pieces.rotate90_clockwise();
        self.double_stack = self.double_stack.rotate90_clockwise();
        self.robben = self.robben.rotate90_clockwise();
        self.seesterne = self.seesterne.rotate90_clockwise();
        self.moewen = self.moewen.rotate90_clockwise();
        self.muscheln = self.muscheln.rotate90_clockwise();
    }

    pub fn rotate90_anti_clockwise(&mut self) {
        self.friendly_pieces = self.friendly_pieces.rotate90_anti_clockwise();
        self.enemy_pieces = self.enemy_pieces.rotate90_anti_clockwise();
        self.double_stack = self.double_stack.rotate90_anti_clockwise();
        self.robben = self.robben.rotate90_anti_clockwise();
        self.seesterne = self.seesterne.rotate90_anti_clockwise();
        self.moewen = self.moewen.rotate90_anti_clockwise();
        self.muscheln = self.muscheln.rotate90_anti_clockwise();
    }

    pub fn rotate180(&mut self) {
        self.friendly_pieces = self.friendly_pieces.rotate180();
        self.enemy_pieces = self.enemy_pieces.rotate180();
        self.double_stack = self.double_stack.rotate180();
        self.robben = self.robben.rotate180();
        self.seesterne = self.seesterne.rotate180();
        self.moewen = self.moewen.rotate180();
        self.muscheln = self.muscheln.rotate180();
    }

    pub fn flip_horizontal(&mut self) {
        self.friendly_pieces = self.friendly_pieces.flip_horizontal();
        self.enemy_pieces = self.enemy_pieces.flip_horizontal();
        self.double_stack = self.double_stack.flip_horizontal();
        self.robben = self.robben.flip_horizontal();
        self.seesterne = self.seesterne.flip_horizontal();
        self.moewen = self.moewen.flip_horizontal();
        self.muscheln = self.muscheln.flip_horizontal();
    }

    pub fn piece_at(&self, pos: u8) -> Option<PieceType> {
        if self.moewen.get(pos) {
            return Some(PieceType::MOEWE);
        } else if self.robben.get(pos) {
            return Some(PieceType::ROBBE);
        } else if self.seesterne.get(pos) {
            return Some(PieceType::SEESTERN);
        } else if self.muscheln.get(pos) {
            return Some(PieceType::MUSCHEL);
        }
        None
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out: String = String::from("╔══════════════════════════╗\n║  ");

        for i in (0..64).rev() {
            let plot = match self.piece_at(i) {
                None => "-",
                Some(piece) => {
                    match piece {
                        PieceType::ROBBE => "R",
                        PieceType::MUSCHEL => "H",
                        PieceType::SEESTERN => "S",
                        PieceType::MOEWE => "M",
                    }
                }
            };

            out.push_str(plot);
            if self.double_stack.get(i) {
                out.push_str("*")
            } else {
                out.push(' ');
            }
            out.push(' ');
            if i % 8 == 0 {
                if i == 0 {
                    out.push_str("║\n");
                } else {
                    out.push_str("║\n║  ")
                }
            }
        }
        out.push_str("╚══════════════════════════╝");

        write!(f, "{}", out)
    }
}

///From board node
impl From<&XmlNode> for Board {
    fn from(node: &XmlNode) -> Self {
        let mut board = Board::new();
        let pieces = node.child("pieces").unwrap();

        for entry in &pieces.children {
            let coordinates = entry.child("coordinates").unwrap();
            let x = coordinates
                .attributes
                .get("x")
                .unwrap()
                .get(0)
                .unwrap()
                .parse::<u8>()
                .expect("Failed to parse coordinates while deserializing");

            let y = coordinates
                .attributes
                .get("y")
                .unwrap()
                .get(0)
                .unwrap()
                .parse::<u8>()
                .expect("Failed to parse coordinates while deserializing");

            let piece_node = entry.child("piece").unwrap();

            let piece_type = PieceType::from(
                piece_node
                    .attributes
                    .get("type")
                    .unwrap()
                    .get(0)
                    .expect("Failed to match piece"),
            );

            let piece_team = piece_node
                .attributes
                .get("team")
                .unwrap()
                .get(0)
                .unwrap()
                .parse::<Team>()
                .expect("Failed to associate Team while deserializing");

            let stacked = match piece_node
                .attributes
                .get("count")
                .unwrap()
                .get(0)
                .unwrap()
                .parse::<u8>()
            {
                Ok(2) => true,
                _ => false,
            };

            let pos = position!(x, y);

            //Presuming we are player 1
            //If the piece team is ONE, then the piece is friendly. Else its the enemies piece.
            match piece_team {
                Team::ONE => board.friendly_pieces.set(pos),
                Team::TWO => board.enemy_pieces.set(pos),
            }

            match piece_type {
                PieceType::ROBBE => board.robben.set(pos),
                PieceType::MUSCHEL => board.muscheln.set(pos),
                PieceType::SEESTERN => board.seesterne.set(pos),
                PieceType::MOEWE => board.moewen.set(pos),
            }

            if stacked {
                board.double_stack.set(pos);
            }
        }
        board
    }
}
