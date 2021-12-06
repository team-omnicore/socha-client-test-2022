#![allow(dead_code)]
#![allow(unused)]

use softwarechallenge_2022::bitboard::Bitboard;
use softwarechallenge_2022::gamestate::{Gamestate, overlaps};
use softwarechallenge_2022::moves::{robbe_gen_moves, seestern_gen_moves};

pub fn main() {

    run();
}

fn run(){
    let mut g = Gamestate::new_default();

    println!("| Perft level | Move-count | time taken | Speed |\n| ----------- | ----------- | ----------- | ----------- |");
    for i in 7..14 {
        g.begin_perft(i);
    }
}


//print!("[");
//for i in 0..64 {
//    let moves = seestern_gen_moves(Bitboard::from(1<<i));
//    print!("0x{:X},", moves.bits)
//}
//print!("]");