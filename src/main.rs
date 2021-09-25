mod bitboard;
mod board;
mod game;
mod gamestate;
mod r#move;
mod nibble;
mod piece;
mod team;
mod vec2;
mod xml_node;

use crate::bitboard::Bitmask;
use crate::board::Board;
use crate::gamestate::{Gamestate, COUNT, OTHER_COUNT};
use crate::piece::Piece;
use crate::vec2::Vec2;
use crate::xml_node::XmlNode;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::{Error, Reader, Writer};
use std::borrow::Borrow;
use std::collections::VecDeque;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::ops::Add;
use std::str;
use std::time::{Duration, SystemTime};
use crate::game::Game;

fn main() {
    let mask = Bitmask::from_bits(1 << 12);
    println!("{}", mask.get(12));
    let mut stream = TcpStream::connect("localhost:13050").expect("Couldn't connect to the server...");
    print!("Connected to stream: {:?}", stream);

    let mut games: Vec<Game> = Vec::new();

    loop {
        let node: XmlNode = XmlNode::read_from(&stream);
        handle_xml_node(node, &stream, &mut games);
    }
}

fn run_game() {
    let mut game = Gamestate::new();
    game.board.setup_random();
    for i in 0..8 {
        let recursion_depth = i;

        let start = SystemTime::now();
        println!(
            "{}",
            game.alpha_beta(recursion_depth, f32::NEG_INFINITY, f32::INFINITY, true)
        );
        let stop = SystemTime::elapsed(&start);
        //println!("Count = {}", unsafe{COUNT});
        //println!("Other Count {}", unsafe{OTHER_COUNT});
        //unsafe {println!("Moves/Round: {}", MOVES as f64/AVG_COUNTER as f64)}
        println!("Recursion depth: {}", recursion_depth);
        println!("Finished in: {:?}", stop.unwrap());

        unsafe {
            COUNT = 0;
            OTHER_COUNT = 0;
        }

        println!("\n====================================\n")
    }
}

fn test_speed() {
    let mut average = Duration::new(0, 0);

    for i in 1..10000 {
        let mut game = Gamestate::new();
        game.board.setup_random();
        let recursion_depth = 7;
        let start = SystemTime::now();
        game.simulate_dumb(recursion_depth);
        let stop = start.elapsed().unwrap();
        average = average.add(stop);
        println!("{:?}", average.div_f64(i as f64));
    }
}


/// Handles the xml_node -
fn handle_xml_node(node: XmlNode, stream: &TcpStream, games: &mut Vec<Game>) {
    match node.name.as_str() {
        "room" => {
            let room_id = node.attributes.get("roomId");
            match room_id {
                Some(room_id) => {
                    println!("RoomID: {}", room_id);

                    let mut game = match games.iter_mut().find(|game| game.room_id == room_id) {
                        Some(game) => { game }
                        None => {
                            Game::new(&stream)
                        }
                    };

                    let data = node.child("data").unwrap();
                    match data.attributes.get("class").unwrap().as_str() {
                        "memento" => {
                            let mut state = node.child("state").unwrap();

                            if game.gamestate.round == 0 {
                                game.gamestate = Gamestate::from_xml_node(state);
                            }else{
                                game.gamestate.update(state);

                                let last_move = state.child("lastMove").unwrap();
                                let from = last_move.child("from").unwrap();
                                let to = last_move.child("to").unwrap();

                                //TODO : update

                            }

                        }
                        &_ => {}
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}
