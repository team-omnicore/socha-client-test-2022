mod xml_node;
mod game_state;
mod board;
mod piece;
mod r#move;
mod vec2;

use std::net::TcpStream;
use std::io::{Write, BufWriter, BufReader, Read, Cursor};
use quick_xml::{Reader, Error, Writer};
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesDecl};
use crate::xml_node::XmlNode;
use std::str;
use quick_xml::events::attributes::Attribute;
use std::borrow::Borrow;
use std::collections::VecDeque;
use crate::game_state::GameState;
use crate::piece::Piece;
use crate::vec2::Vec2;
use crate::r#move::Move;
use rand::Rng;


fn main() {



    let mut stream = TcpStream::connect("localhost:13050").expect("Couldn't connect to the server...");
    print!("Connected to stream: {:?}", stream);
    BufWriter::new(&stream).write("<protocol><join/>".as_bytes());

    let mut game_state: GameState = GameState::new();
    loop {
        let node: XmlNode = XmlNode::read_from(&stream);
        //print_xml_node(&node);
        handle_xml_node(node, &mut game_state, &stream);
    }
}

pub fn make_move(stream: &TcpStream, m : &Move, room_id : &String) {

    let mut writer2 = Writer::new(Cursor::new(Vec::new()));

    let mut room_start = BytesStart::owned("room", "room".len());
    room_start.push_attribute(("roomId", room_id.to_string().as_str()));
    writer2.write_event(Event::Start(room_start));

    let mut data_start = BytesStart::owned("data", "data".len());
    data_start.push_attribute(("class", "move"));
    writer2.write_event(Event::Start(data_start));

    let mut from = BytesStart::owned("from", "from".len());
    from.push_attribute(("x", m.from.x.to_string().as_str()));
    from.push_attribute(("y", m.from.y.to_string().as_str()));
    writer2.write_event(Event::Empty(from));
    let mut to = BytesStart::owned("to", "to".len());
    to.push_attribute(("x", m.to.x.to_string().as_str()));
    to.push_attribute(("y", m.to.y.to_string().as_str()));
    writer2.write_event(Event::Empty(to));

    println!("From Move X: {:?}", m.from.x);
    println!("From Move Y: {:?}", m.from.y);
    println!("To Move X: {:?}", m.to.x);
    println!("To Move Y: {:?}", m.to.y);

    let mut data_end = BytesEnd::owned(Vec::from("data"));
    writer2.write_event(Event::End(data_end));

    let mut room_end = BytesEnd::owned(Vec::from("room"));
    writer2.write_event(Event::End(room_end));

    let result = writer2.into_inner().into_inner();
    println!("String: {}", str::from_utf8(&*result).unwrap());
    BufWriter::new(stream).write(str::from_utf8(&*result).unwrap().as_bytes());
}

pub fn handle_xml_node(node: XmlNode, game_state: &mut GameState, stream: &TcpStream) {
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

                            if game_state.turn == 0 {
                                let start_team = state.child("startTeam").unwrap();
                                println!("Start Team: {}", start_team.text);

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

                                    let piece = Piece::new(Vec2::new(x.to_owned().parse().unwrap(), y.to_owned().parse().unwrap()), piece_type);

                                    match team.as_str() {
                                        "ONE" => game_state.board.one_pieces.push(piece),
                                        "TWO" => game_state.board.two_pieces.push(piece),
                                        _ => {}
                                    }
                                }
                            }else{
                                let last_move = state.child("lastMove").unwrap();
                                let from = last_move.child("from").unwrap();
                                let to = last_move.child("to").unwrap();

                                let m = Move::new(Vec2::new(from.attributes.get("x").unwrap().to_owned().parse::<i32>().unwrap(), from.attributes.get("y").unwrap().to_owned().parse::<i32>().unwrap()), Vec2::new(to.attributes.get("x").unwrap().to_owned().parse::<i32>().unwrap(), to.attributes.get("y").unwrap().to_owned().parse::<i32>().unwrap()));
                                for mut piece in &mut game_state.board.one_pieces {
                                    if piece.position.x == m.from.x && piece.position.y == m.from.y {
                                        piece.position = Vec2::new(m.to.x, m.to.y);
                                    }
                                }
                                let index = game_state.board.two_pieces.iter().position(|piece| piece.position.x == m.from.x && piece.position.y == m.from.y);
                                if index.is_some() {
                                    game_state.board.two_pieces.remove(index.unwrap());
                                }

                            }

                            if game_state.turn % 2 == 1 {
                                let possible_m = &game_state.board.two_possible_moves();
                                let m = &possible_m[rand::thread_rng().gen_range(0..possible_m.len()-1)];
                                for mut piece in &mut game_state.board.two_pieces {
                                    if piece.position.x == m.from.x && piece.position.y == m.from.y {
                                        piece.position = Vec2::new(m.to.x, m.to.y);
                                    }
                                }
                                make_move(stream, &m, &game_state.room_id);
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

pub fn print_xml_node(node: &XmlNode) {
    println!("Name: {}", node.name);
    for (key, value) in &node.attributes {
        println!("{} : {}", key, value);
    }
    for child in &node.children {
        print_xml_node(child);
    }
}


