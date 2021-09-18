mod xml_node;
mod game_state;
mod board;
mod piece;
mod r#move;
mod vec2;

use std::net::TcpStream;
use std::io::{Write, BufWriter, BufReader, Read};
use quick_xml::{Reader, Error};
use quick_xml::events::Event;
use crate::xml_node::XmlNode;
use std::str;
use quick_xml::events::attributes::Attribute;
use std::borrow::Borrow;
use std::collections::VecDeque;
use crate::game_state::GameState;
use crate::piece::Piece;

fn main() {
    let mut stream = TcpStream::connect("localhost:13050").expect("Couldn't connect to the server...");
    print!("Connected to stream: {:?}", stream);
    BufWriter::new(&stream).write("<protocol><join/>".as_bytes());

    let mut game_state: GameState = GameState::new();

    loop {
        let node: XmlNode = XmlNode::read_from(&stream);
        //print_xml_node(&node);
        handle_xml_node(node, &mut game_state);
    }
}

pub fn handle_xml_node(node: XmlNode, game_state: &mut GameState) {
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
        "data" => {
            match node.attributes.get("class").unwrap().as_str() {
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

                                let piece = Piece::new(x.to_owned().parse::<u8>().unwrap(), y.to_owned().parse::<u8>().unwrap(), piece_type);

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
            }
        }
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

pub fn print_xml_node(node: &XmlNode) {
    println!("Name: {}", node.name);
    for (key, value) in &node.attributes {
        println!("{} : {}", key, value);
    }
    for child in &node.children {
        print_xml_node(child);
    }
}


