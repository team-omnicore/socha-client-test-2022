#![allow(dead_code)]
#![allow(unused)]

use softwarechallenge_2022::bitboard::Bitboard;
use softwarechallenge_2022::gamestate::{overlaps, Gamestate};
use softwarechallenge_2022::moves::{
    moewe_gen_moves, moewe_lookup, muschel_gen_moves, muschel_lookup, robbe_gen_moves,
    robbe_lookup, seestern_gen_moves, seestern_lookup,
};

pub fn main() {
    run();
}

fn run() {
    let mut g = Gamestate::new_default();
    g.perft_up_to(14);
}
