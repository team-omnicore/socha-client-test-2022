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
use crate::game::Game;
use crate::gamestate::Gamestate;
use crate::vec2::{Coords};
use crate::xml_node::XmlNode;
use std::net::TcpStream;

fn main() {
    let mask = Bitmask::from_bits(1 << 12);
    println!("{}", mask.get(12));
    let mut stream =
        TcpStream::connect("localhost:13050").expect("Couldn't connect to the server...");
    print!("Connected to stream: {:?}", stream);

    let mut games: Vec<Game> = Vec::new();

    loop {
        let node: XmlNode = XmlNode::read_from(&stream);
        handle_xml_node(node, &stream, &mut games);
    }
}

/// Handles the xml_node -
fn handle_xml_node<'a>(node: XmlNode, stream: &'a TcpStream, games: &mut Vec<Game<'a>>) {
    match node.name.as_str() {
        "room" => {
            let room_id = node.attributes.get("roomId");
            match room_id {
                Some(room_id) => {
                    println!("RoomID: {}", room_id);


                    //let game = games
                    //    .iter_mut()
                    //    .find(|game| game.room_id.as_ref().unwrap().eq(room_id));

                    for gayme in games {

                    }

                    let mut game = match game {
                        None => {Game::new(&stream)}
                        Some(game) => {*game}
                    };

                    let data = node.child("data").unwrap();
                    match data.attributes.get("class").unwrap().as_str() {
                        "memento" => {
                            let mut state = node.child("state").unwrap();

                            if game.gamestate.round == 0 {
                                game.gamestate = Gamestate::from(state);
                            } else {
                                //game.gamestate.update(state);

                                let last_move = state.child("lastMove").unwrap();
                                let from = last_move.child("from").unwrap();
                                let to = last_move.child("to").unwrap();

                                let from_x: i8 = from.attributes.get("x").unwrap().parse().unwrap();
                                let from_y: i8 = from.attributes.get("y").unwrap().parse().unwrap();
                                let origin = Coords::new(from_x, from_y);

                                let to_x: i8 = to.attributes.get("x").unwrap().parse().unwrap();
                                let to_y: i8 = to.attributes.get("y").unwrap().parse().unwrap();
                                let result = Coords::new(to_x, to_y);

                                let pos_origin = pos_from_coords!(origin.x, origin.y);
                                let pos_result = pos_from_coords!(result.x, result.y);

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
