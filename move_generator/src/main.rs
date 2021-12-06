#![allow(dead_code)]
#![allow(unused)]

use softwarechallenge_2022::bitboard::Bitboard;
use softwarechallenge_2022::gamestate::{overlaps, Gamestate};
use softwarechallenge_2022::moves::{moewe_gen_moves, muschel_gen_moves, robbe_gen_moves, robbe_lookup, seestern_gen_moves, seestern_lookup, muschel_lookup, moewe_lookup};

pub fn main() {

    run();
}

fn run() {
    let mut g = Gamestate::new_default();

    println!("| Perft level | Move-count | time taken | Speed |\n| ----------- | ----------- | ----------- | ----------- |");
    for i in 0..14 {
        g.begin_perft(i);
    }
}
