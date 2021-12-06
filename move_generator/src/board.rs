use crate::bitboard::Bitboard;
use crate::moves::{moewe_gen_moves, muschel_gen_moves, robbe_gen_moves, seestern_gen_moves};
use crate::{bit_loop, square_of};
use std::fmt;
use std::fmt::Formatter;

pub struct Board {
    pub enemy: Bitboard,
    pub friendly: Bitboard,
    pub seesterne: Bitboard,
    pub muscheln: Bitboard,
    pub moewen: Bitboard,
    pub robben: Bitboard,
    pub double: Bitboard,
}

impl Board {
    pub const fn new() -> Self {
        Board {
            enemy: Bitboard::new(),
            friendly: Bitboard::new(),
            seesterne: Bitboard::new(),
            muscheln: Bitboard::new(),
            moewen: Bitboard::new(),
            robben: Bitboard::new(),
            double: Bitboard::new(),
        }
    }

    pub fn new_default() -> Self {
        let enemy = Bitboard::from(0xFF00000000000000u64);
        let friendly = Bitboard::from(0xFFu64);

        let double = Bitboard::new();

        let muscheln = Bitboard::from(0b00110000) | Bitboard::from(0b00110000).rotate180();
        let seesterne = Bitboard::from(0b00001100) | Bitboard::from(0b00001100).rotate180();
        let robben = Bitboard::from(0b10000010) | Bitboard::from(0b10000010).rotate180();
        let moewen = Bitboard::from(0b01000001) | Bitboard::from(0b01000001).rotate180();

        Board {
            enemy,
            friendly,
            seesterne,
            muscheln,
            moewen,
            robben,
            double,
        }
    }

    pub fn rotate180(&mut self) {
        self.friendly = self.friendly.rotate180();
        self.enemy = self.enemy.rotate180();
        self.double = self.double.rotate180();
        self.robben = self.robben.rotate180();
        self.seesterne = self.seesterne.rotate180();
        self.moewen = self.moewen.rotate180();
        self.muscheln = self.muscheln.rotate180();
    }

    pub fn piece_at(&self, pos: u8) -> char {
        let mut x = if self.robben.get(pos) {
            'r'
        } else if self.muscheln.get(pos) {
            'h'
        } else if self.moewen.get(pos) {
            'm'
        } else if self.seesterne.get(pos) {
            's'
        } else {
            '-'
        };
        if self.double.get(pos) {
            x = x.to_ascii_uppercase();
        }
        return x;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out: String = String::from("╔══════════════════════════╗\n");
        let mut index_min = 56 + 8;
        let mut index_max = 64 + 8;
        for i in 0..8 {
            index_max -= 8;
            index_min -= 8;
            out.push_str("║  ");
            for j in index_min..index_max {
                out.push(self.piece_at(j));
                out.push_str("  ");
            }
            out.push_str("║\n");
        }
        out.push_str("╚══════════════════════════╝");
        write!(f, "{}", out)
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Board {
            enemy: self.enemy,
            friendly: self.friendly,
            seesterne: self.seesterne,
            muscheln: self.muscheln,
            moewen: self.moewen,
            robben: self.robben,
            double: self.double,
        }
    }
}

/*
impl fmt::Display for Gamestate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out: String = String::from("╔══════════════════════════╗\n");
        let mut index_min = 56+8;
        let mut index_max = 64+8;
        for i in 0..8 {
            index_max -= 8;
            index_min -= 8;
            out.push_str("║  ");
            for j in index_min..index_max{
                out.push(self.piece_at(j));
                out.push_str("  ");
            }
            out.push_str("║\n");
        }
        out.push_str("╚══════════════════════════╝");
        write!(f, "{}", out)
    }
}
 */
