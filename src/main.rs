#![allow(unused)]
#![allow(dead_code)]
use crate::game::GameError;
use crate::game_result::GameResult;
use env_logger::Builder;
use game::Join;
use log::LevelFilter;
use std::env;

mod bitboard;
mod board;
mod game;
mod game_move;
mod game_result;
mod gamestate;
mod nibble;
mod piece;
mod team;
mod vec2;
mod xml_node;

fn main() {
    Builder::new()
        .parse_env(&env::var("MY_APP_LOG").unwrap_or_default())
        .filter_level(LevelFilter::Info)
        .init();

    let mut game = Join::ANY
        .connect("localhost:13050")
        .expect("Connection failed");

    let result = game.game_loop();

    match result {
        Ok(res) => {
            log::info!("{:?}", res);
        }
        Err(err) => {
            log::error!("Network error! {:?}", err);
        }
    }
}
