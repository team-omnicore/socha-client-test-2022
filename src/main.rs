mod bitboard;
mod board;
mod gamestate;
mod r#move;
mod nibble;
mod piece;
mod vec2;
mod xml_node;

use crate::bitboard::Bitmask;
use crate::board::Board;
use crate::gamestate::{Gamestate, COUNT, OTHER_COUNT};
use crate::piece::Piece;
use crate::xml_node::XmlNode;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event;
use quick_xml::{Error, Reader};
use std::borrow::Borrow;
use std::collections::VecDeque;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::ops::Add;
use std::str;
use std::time::{Duration, SystemTime};

fn main() {
    let mask = Bitmask::from_bits(1 << 12);
    println!("{}", mask.get(12));
}

/*fn main() {
    let mut stream =
        TcpStream::connect("localhost:13050").expect("Couldn't connect to the server...");
    print!("Connected to stream: {:?}", stream);
    BufWriter::new(&stream).write("<protocol><join/>".as_bytes());

    let mut game_state: GameState = GameState::new();

    loop {
        let node: XmlNode = XmlNode::read_from(&stream);
        //print_xml_node(&node);
        handle_xml_node(node, &mut game_state);
    }
}
*/

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

/*

fn handle_xml_node(node: XmlNode, game_state: &mut GameState) {
    match node.name.as_str() {
        "room" => {
            let room_id = node.attributes.get("roomId");
            match room_id {
                Some(room_id) => {
                    println!("RoomID: {}", room_id);
                    game_state.room_id = room_id.to_owned();
                }
                None => {}
            }
        }
        "data" => match node.attributes.get("class").unwrap().as_str() {
            "memento" => {
                let mut state = node.child("state");
                match state {
                    Some(state) => {
                        let turn = state.attributes.get("turn").unwrap();
                        game_state.turn = turn.to_owned().parse::<u16>().unwrap();

                        let start_team = state.child("startTeam").unwrap();

                        let board = state.child("board").unwrap();
                        let pieces = board.child("pieces").unwrap();

                        for entry in &pieces.children {
                            let coordinates = entry.child("coordinates").unwrap();
                            let x = coordinates.attributes.get("x").unwrap();
                            let y = coordinates.attributes.get("y").unwrap();

                            let piece_node = entry.child("piece").unwrap();
                            let piece_type = piece_node.attributes.get("type").unwrap();
                            let team = piece_node.attributes.get("team").unwrap();
                            let count = piece_node.attributes.get("count").unwrap();

                            let piece = Piece::new(
                                x.to_owned().parse::<u8>().unwrap(),
                                y.to_owned().parse::<u8>().unwrap(),
                                piece_type,
                            );

                            match team.as_str() {
                                "ONE" => game_state.board.one_pieces.push(piece),
                                "TWO" => game_state.board.two_pieces.push(piece),
                                _ => {}
                            }
                        }
                    }
                    None => {
                        panic!("No state!")
                    }
                }
            }
            "result" => {}
            _ => {}
        },
        _ => {}
    }
}

/**
pub fn handle_xml_node(node: XmlNode, game_state: &mut GameState, stream: &TcpStream){
    match node.name.as_str() {
        "room" => {
            let room_id = node.attributes.get("roomId");
            game_state.room_id = room_id.unwrap().to_string();
        }
        "data" => {
            let room_id = node.attributes.get("roomId");
            game_state.room_id = room_id.unwrap().to_string();
        }
        "state" => {
            let turn = node.attributes.get("turn");
            game_state.turn = turn.unwrap().to_string().parse::<u16>().unwrap();
        }
        "lastMove" => {
            BufWriter::new(stream).write((r#"<room roomId=""#.to_owned() + &*game_state.room_id + r#""><data class="move"><from x="# + &*(7 - game_state.turn).to_string() + r#" y="5"/><to x="# + &*(7 - game_state.turn - 1).to_string() + r#" y="5"/></data></room>"#).as_bytes());
        }

        _ => {}
    }

    for child in node.children {
        handle_xml_node(child, game_state, stream);
    }
}
 **/

fn print_xml_node(node: &XmlNode) {
    println!("Name: {}", node.name);
    for (key, value) in &node.attributes {
        println!("{} : {}", key, value);
    }
    for child in &node.children {
        print_xml_node(child);
    }
}
*/
