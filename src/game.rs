use crate::gamestate::Gamestate;
use std::net::TcpStream;
use std::io::{BufWriter, Write};
use crate::r#move::Move;

pub struct Game<'a> {
    pub gamestate: Gamestate,
    pub room_id: Option<String>,
    stream: &'a TcpStream,
}

impl Game {
    pub fn new(stream: &TcpStream) -> Self {
        Self {
            gamestate: Gamestate::new(),
            room_id: None,
            stream,
        }
    }

    /// Enter any open game
    pub fn join(&self) {
        BufWriter::new(&self.stream).write("<protocol><join/>".as_bytes());
    }

    /// Enter an already open, but not yet started game
    pub fn join_room(&self, room_id: &String) {
        BufWriter::new(&self.stream).write(format!("<protocol><joinRoom roomId=\"{}\"/>", room_id).as_bytes());
    }

    /// Join a scheduled game
    pub fn join_prepared(&self, reservation: &String){
        BufWriter::new(&self.stream).write(format!("<protocol><joinPrepared reservationCode=\"{}\"/>", reservation).as_bytes());
    }

    /// Send move
    pub fn send_move(&self, r#move: &Move) {
        BufWriter::new(&self.stream).write(format!("<room roomId=\"{}\"><data class=\"move\"><from x=\"{}\" y=\"{}\"/><to x=\"{}\" y=\"{}\"/></data></room>", &self.room_id, r#move.from.x, r#move.from.y, r#move.to.x, r#move.to.y).as_bytes());
    }
}
