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

fn main() {
    let mut stream = TcpStream::connect("localhost:13050").expect("Couldn't connect to the server...");
    print!("Connected to stream: {:?}", stream);
    BufWriter::new(&stream).write("<protocol><join/>".as_bytes());

    let mut game_state : GameState = GameState::new();

    loop{
        let node : XmlNode = XmlNode::read_from(&stream);

        handle_xml_node(&node, &mut game_state, &stream);
        print_xml_node(node);
    }
}

pub fn handle_xml_node(node: &XmlNode, game_state: &mut GameState, stream: &TcpStream){
    if node.name == "room" {
        let room_id = node.attributes.get("roomId");
        game_state.room_id = room_id.unwrap().to_string();
    }
    if node.name == "state" {
        let turn = node.attributes.get("turn");
        game_state.turn = turn.unwrap().to_string().parse::<u16>().unwrap();
    }
    if node.name == "lastMove"{
            BufWriter::new(stream).write((r#"<room roomId=""#.to_owned() + &*game_state.room_id + r#""><data class="move"><from x="# + &*(7 - game_state.turn).to_string() + r#" y="5"/><to x="# + &*(7 - game_state.turn - 1).to_string() + r#" y="5"/></data></room>"#).as_bytes());
    }

    for child in &node.children {
        handle_xml_node(&child, game_state, stream);
    }
}

pub fn print_xml_node(node: XmlNode){
    println!("Name: {}", node.name);
    for (key, value) in node.attributes {
        println!("{} : {}", key, value);
    }
    for child in node.children {
        print_xml_node(child);
    }
}


