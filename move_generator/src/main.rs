#![allow(dead_code, unused)]
#![feature(stmt_expr_attributes)]

use softwarechallenge_2022::bitboard::Bitboard;
use softwarechallenge_2022::board::Board;
use softwarechallenge_2022::gamestate::{Gamestate, overlaps};
use softwarechallenge_2022::square_of;

pub fn main() {

    //let mut old_robbe_1= Bitboard::from(0x00100000);
    //let mut new_robbe_1= Bitboard::from(0x00010000);
    //let mut double_1=    Bitboard::from(0x01101101);
//
    //let mut old_robbe_2= old_robbe_1.clone();
    //let mut new_robbe_2= new_robbe_1.clone();
    //let mut double_2= double_1.clone();
//
    //let new_pos = square_of(new_robbe_1.bits);
    //let old_pos = square_of(old_robbe_1.bits);
//
    //println!("{}", double_1);
//
    //if double_1.get(old_pos) {
    //    double_1 = double_1.clear(old_pos);
    //    double_1 = double_1.set(new_pos);
    //}
//
    //let overlaps = overlaps(double_2, old_robbe_2);
    //double_2 &= !old_robbe_2 | !overlaps;
    //double_2 |= new_robbe_2 & overlaps;
//
    ////if (double_2 & old_robbe_2).bits != 0 {
    ////    double_2 &= !old_robbe_2;
    ////    double_2 |= new_robbe_2;
    ////}
//
    //println!("{}", double_1);
    //println!("{}", double_2);
//
    //assert_eq!(double_1.bits, double_2.bits);




    run();
}

fn run() {
    let mut g = Gamestate::new_default();
    g.perft_up_to(14);
}
