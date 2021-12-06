#![allow(dead_code)]
#![allow(unused)]

use softwarechallenge_2022::bitboard::Bitboard;
use softwarechallenge_2022::gamestate::{Gamestate, overlaps};
use softwarechallenge_2022::moves::{robbe_gen_moves, seestern_gen_moves};

pub fn main() {

    //let mut points_added:u8 = 0;
//
    //let mut enemy =  Bitboard::from(0x041000000);
//
    //let mut double = Bitboard::from(0x001F03D0);
    //let new_robbe =  Bitboard::from(0x10000000);
    //let mut robben = Bitboard::from(0x00110000);
    //
    //let if_overlaps_enemy = overlaps(new_robbe, enemy);
    //
//
    //let if_overlaps_double = overlaps(new_robbe, double);
    //let clip_robbe_if_overlaps = !new_robbe | !if_overlaps_double;
//
//
    //points_added += 0x1u8 & if_overlaps_double.bits as u8;
//
    //println!("{}", points_added);
//
    //println!("{}", double);
    //println!("{}", new_robbe);
    //println!("{}", robben);
//
    //println!("clip_robbe_if_overlaps{}", clip_robbe_if_overlaps);
//
    //double &= clip_robbe_if_overlaps;
    //robben &= clip_robbe_if_overlaps;
//
    //println!("New double{}", double);
    //println!("New robben{}", robben);

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
