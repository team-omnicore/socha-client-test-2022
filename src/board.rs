use crate::bitboard::Bitmask;
use crate::piece::pieces::*;
use crate::piece::PieceType;
use crate::pos_from_coords;
use crate::r#move::Move;
use crate::team::Team;
use crate::xml_node::XmlNode;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Board {
    pub enemy_pieces: Bitmask,
    pub friendly_pieces: Bitmask,
    pub seesterne: Bitmask,
    pub muscheln: Bitmask,
    pub moewen: Bitmask,
    pub robben: Bitmask,
    pub double_stack: Bitmask,
}

impl Board {
    pub const fn new() -> Self {
        Board {
            enemy_pieces: Bitmask::new(),
            friendly_pieces: Bitmask::new(),
            seesterne: Bitmask::new(),
            muscheln: Bitmask::new(),
            moewen: Bitmask::new(),
            robben: Bitmask::new(),
            double_stack: Bitmask::new(),
        }
    }

    pub fn calculate_legal(&self) -> Vec<Move> {
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

    pub fn setup_random(&mut self) {
        self.friendly_pieces.bits = 0x00000000000000FF;
        self.enemy_pieces.bits = 0xFF00000000000000;
        self.robben.bits = 0x0900000000000012;
        self.seesterne.bits = 0xA0000000000000A0;
        self.muscheln.bits = 0x1200000000000009;
        self.moewen.bits = 0x4400000000000044;
    }

    pub fn apply(&mut self, r#move: &Move) -> u8 {
        //We know that the move is legal, now apply it to the board

        let origin = pos_from_coords!(r#move.origin.x as u8, r#move.origin.y as u8) as u8;

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

        let pos = pos_from_coords!(set_move.x as u8, set_move.y as u8);

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

    pub fn switch_sides(&mut self) {
        self.friendly_pieces.reverse();
        self.enemy_pieces.reverse();
        self.seesterne.reverse();
        self.moewen.reverse();
        self.muscheln.reverse();
        self.robben.reverse();
        self.double_stack.reverse();
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
                Some(piece) => match piece {
                    PieceType::ROBBE => "R",
                    PieceType::MUSCHEL => "M",
                    PieceType::SEESTERN => "S",
                    PieceType::MOEWE => "V",
                },
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

impl From<&XmlNode> for Board {
    fn from(node: &XmlNode) -> Self {
        let mut board = Board::new();
        let pieces = node.child("pieces").unwrap();

        for entry in &pieces.children {
            let coordinates = entry.child("coordinates").unwrap();
            let x: u8 = coordinates
                .attributes
                .get("x")
                .unwrap()
                .parse()
                .expect("Failed to parse coordinates while deserializing");
            let y: u8 = coordinates
                .attributes
                .get("y")
                .unwrap()
                .parse()
                .expect("Failed to parse coordinates while deserializing");

            let piece_node = entry.child("piece").unwrap();
            let piece_type =
                PieceType::piece_type_from_name(piece_node.attributes.get("type").unwrap())
                    .expect("Failed to associate Piece type while deserializing");

            let team = piece_node
                .attributes
                .get("team")
                .unwrap()
                .parse()
                .expect("Failed to associate Team while deserializing");

            let stacked = match piece_node.attributes.get("count").unwrap().parse::<u8>() {
                Ok(2) => true,
                _ => false,
            };

            let pos = crate::pos_from_server_coords!(x, y);

            /////////////////////////////////////////////////////////////

            //Davon ausgegangen, dass wir team 1 sind
            match team {
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
