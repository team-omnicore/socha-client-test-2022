#![allow(dead_code)]
use crate::game::Join;
use env_logger::Builder;
use log::{LevelFilter};
use std::env;

mod bitboard;
mod board;
mod game;
mod game_move;
mod gamestate;
mod nibble;
mod piece;
mod team;
mod vec2;
mod xml_node;

fn main() {

    Builder::new()
        .parse_env(&env::var("MY_APP_LOG").unwrap_or_default())
        .filter_level(LevelFilter::Debug)
        .init();

    let mut game = Join::ANY
        .connect("localhost:13050")
        .expect("Connection failed");

    game.game_loop().unwrap_or(());
}