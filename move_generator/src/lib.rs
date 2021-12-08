#![allow(dead_code, unused)]
#![feature(stmt_expr_attributes)]

pub mod bitboard;
pub mod board;
pub mod gamestate;
pub mod moves;

pub fn bit_loop<F: FnMut(u64)>(mut x: u64, mut f: F) {
    while x != 0 {
        f(x);
        x &= x - 1;
    }
}

pub fn square_of(bitboard: u64) -> u8 {
    bitboard.trailing_zeros() as u8
}

pub fn pretty_print_speed(speed: f64) -> String {
    if speed < 1000f64 {
        format!("{:.1} Nodes/sec", speed)
    } else if speed < 1000000f64 {
        format!("{:.1} KNodes/sec", speed / 1000f64)
    } else if speed < 1000000000f64 {
        format!("{:.1} MNodes/sec", speed / 1000000f64)
    } else {
        format!("{:.2} GNodes/sec", speed / 1000000000f64)
    }
}
