use crate::gamestate::Gamestate;
use crate::r#move::Move;
use std::io::{BufWriter, Write};
use std::net::TcpStream;

#[derive(Copy, Clone, Debug)]
pub struct Game<'a> {
    pub gamestate: Gamestate,
    pub room_id: &'a Option<String>,
    stream: &'a TcpStream,
}

impl<'a> Game<'a> {

    pub fn new(stream: &'a TcpStream) -> Self {
        Self {
            gamestate: Gamestate::new(),
            room_id: &None,
            stream,
        }
    }

    /// Enter any open game
    pub fn join(&self) {
        BufWriter::new(self.stream).write("<protocol><join/>".as_bytes());
    }

    /// Enter an already open, but not yet started game
    pub fn join_room(&self, room_id: &String) {
        BufWriter::new(self.stream)
            .write(format!("<protocol><joinRoom roomId=\"{}\"/>", room_id).as_bytes());
    }

    /// Join a scheduled game
    pub fn join_prepared(&self, reservation: &String) {
        BufWriter::new(self.stream).write(
            format!(
                "<protocol><joinPrepared reservationCode=\"{}\"/>",
                reservation
            )
            .as_bytes(),
        );
    }

    /// Send move
    pub fn send_move(&self, r#move: &Move) {
        BufWriter::new(self.stream).write(format!("<room roomId=\"{}\"><data class=\"move\"><from x=\"{}\" y=\"{}\"/><to x=\"{}\" y=\"{}\"/></data></room>",
                                                   (&self.room_id).as_ref().unwrap(),
                                                   r#move.origin.x,
                                                   r#move.origin.y,
                                                   r#move.result.x,
                                                   r#move.result.y
        ).as_bytes());
    }
}
