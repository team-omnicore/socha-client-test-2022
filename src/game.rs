use std::io::{BufReader, Result};
use std::io::{BufWriter, Write};
use std::net::TcpStream;

use crate::game_move::Move;
use crate::gamestate::Gamestate;
use crate::team::Team;
use crate::xml_node::XmlNode;
use log::debug;
use xml::EventReader;

#[derive(Debug)]
pub struct Game {
    pub gamestate: Gamestate,
    pub room_id: String,
    pub stream: TcpStream,
    pub team: Team,
}

impl Game {
    pub fn send_move(&self, r#move: &mut Move) {
        let final_move = r#move.translate(&self.team);
        log::info!("Sending move: {}", final_move);

        BufWriter::new(&self.stream).write(format!("<room roomId=\"{}\"><data class=\"move\"><from x=\"{}\" y=\"{}\"/><to x=\"{}\" y=\"{}\"/></data></room>",
                                                   &self.room_id,
                                                   final_move.origin.x,
                                                   final_move.origin.y,
                                                   final_move.result.x,
                                                   final_move.result.y
        ).as_bytes()).expect("Failed to write move");
    }

    fn on_move_request(&mut self) {
        log::info!("Received MoveRequest");
        let mut best = self.gamestate.best_move();
        self.send_move(&mut best);
    }

    fn on_receive_memento(&mut self, data_node: &XmlNode) {
        /*
        let state = data_node
            .child("state")
            .expect("Received data node without gamestate");

        //let turn:u8 = data_node.attributes.get("turn").unwrap().get(0).unwrap().parse().unwrap();

        {
            let last_move = state
                .child("lastMove")
                .expect("Failed to find lastMove in state");

            let from = last_move.child("from").unwrap();
            let to = last_move.child("to").unwrap();

            let from_x = from
                .attributes
                .get("x")
                .unwrap()
                .get(0)
                .unwrap()
                .parse::<i8>()
                .unwrap();
            let from_y = from
                .attributes
                .get("y")
                .unwrap()
                .get(0)
                .unwrap()
                .parse::<i8>()
                .unwrap();
            let to_x = to
                .attributes
                .get("x")
                .unwrap()
                .get(0)
                .unwrap()
                .parse::<i8>()
                .unwrap();
            let to_y = to
                .attributes
                .get("y")
                .unwrap()
                .get(0)
                .unwrap()
                .parse::<i8>()
                .unwrap();

            let from_pos = position!(from_x, from_y);
            let to_pos= position!(to_x, to_y);

            self.gamestate.board.apply_anonymous(from_pos, to_pos);
        }

        {
            for entry in state.child("ambers").unwrap().children.iter() {
                let team = entry.child("team").unwrap();
                let score = entry.child("int").unwrap();

                let team = Team::from(&team.data);
                let score = (&score.data).parse::<u8>().unwrap();

                match team {
                    Team::ONE => self.gamestate.points.set_left(score),
                    Team::TWO => self.gamestate.points.set_right(score),
                }
            }
        }

         */

        let gamestate_node = data_node.child("state").unwrap();

        let turn = gamestate_node
            .attributes
            .get("turn")
            .unwrap()
            .get(0)
            .unwrap()
            .parse::<u8>()
            .unwrap();

        let mut gamestate = Gamestate::from(gamestate_node);

        match self.team {
            Team::ONE => {
                gamestate.board.rotate90_anti_clockwise();
            }
            Team::TWO => {
                gamestate.board.rotate90_clockwise();
                gamestate
                    .board
                    .friendly_pieces
                    .swap_with(&mut gamestate.board.enemy_pieces);
            }
        }
        gamestate.board.flip_horizontal();

        self.gamestate = gamestate;

        println!(
            "\n[ReceivedMemento | Turn {}]\n{}",
            turn, self.gamestate.board
        );
    }

    pub fn game_loop(&mut self) -> std::result::Result<(), GameError> {
        let copy_of_stream = self.stream.try_clone().unwrap();
        let mut parser = EventReader::new(BufReader::new(&copy_of_stream));

        loop {
            let received = XmlNode::read_from(&mut parser);

            match received.name.as_str() {
                "protocol" => {
                    log::info!("Ending game");
                    return Ok(());
                }
                "data" => {
                    let class = received
                        .attributes
                        .get("class")
                        .expect("Received node without class")
                        .get(0)
                        .unwrap();

                    match class.as_str() {
                        "memento" => {
                            self.on_receive_memento(&received);
                        }
                        "moveRequest" => {
                            self.on_move_request();
                        }
                        "welcomeMessage" => {
                            panic!("Received multiple welcome messages!")
                        }
                        "result" => {
                            println!("{:?}", received);
                        }
                        class => {
                            panic!("Failed to match class: {}", class)
                        }
                    }
                }
                name => {
                    panic!("Failed to match node '{}': {:?}", name , received)
                }
            }
        }
    }
}

impl Clone for Game {
    fn clone(&self) -> Self {
        let gamestate = self.gamestate;
        let room_id = self.room_id.clone();
        let stream = self.stream.try_clone().expect("Failed to clone stream");
        let team = self.team;

        Self {
            gamestate,
            room_id,
            stream,
            team,
        }
    }
}

pub enum Join<'a> {
    ANY,
    ROOM(&'a str),
    PREPARED(&'a str),
}

impl<'a> Join<'a> {
    pub fn connect(&self, network_address: &str) -> Result<Game> {
        let stream = TcpStream::connect(network_address)?;

        debug!("Connected to server...");

        let mut writer = BufWriter::new(stream.try_clone().expect("Couldn't clone stream"));

        let _sent = match self {
            Join::ANY => writer.write("<protocol><join/>".as_bytes()),
            Join::ROOM(room_id) => {
                writer.write(format!("<protocol><joinRoom roomId=\"{}\"/>", room_id).as_bytes())
            }
            Join::PREPARED(reservation) => writer.write(
                format!(
                    "<protocol><joinPrepared reservationCode=\"{}\"/>",
                    reservation
                )
                .as_bytes(),
            ),
        }?;
        writer.flush()?;
        debug!("Sent join-request to server");

        let mut parser = EventReader::new(BufReader::new(&stream));

        let joined = XmlNode::read_from(&mut parser);
        let welcome = XmlNode::read_from(&mut parser);

        let room_id = joined.attributes.get("roomId").unwrap().get(0).unwrap();
        let my_team: Team = welcome
            .attributes
            .get("color")
            .expect("No attribute named \"color\"")
            .get(0)
            .unwrap()
            .into();

        let node = XmlNode::read_from(&mut parser);

        match node.attributes.get("class") {
            None => {}
            Some(class) => match class[0].as_str() {
                "memento" => {
                    let gamestate_node = node.child("state").unwrap();

                    let mut gamestate = Gamestate::from(gamestate_node);

                    match my_team {
                        Team::ONE => {
                            gamestate.board.rotate90_anti_clockwise();
                        }
                        Team::TWO => {
                            gamestate.board.rotate90_clockwise();
                            gamestate
                                .board
                                .friendly_pieces
                                .swap_with(&mut gamestate.board.enemy_pieces);
                        }
                    }

                    let game = Game {
                        gamestate,
                        room_id: room_id.clone(),
                        stream,
                        team: my_team,
                    };

                    log::info!("Joined {} as Team {:?}", game.room_id, game.team);
                    println!("\n[Start]\n{}", game.gamestate.board);

                    return Ok(game);
                }
                "moveRequest" => {
                    log::error!("Received a moveRequest before constructing a game");
                    panic!()
                }
                &_ => {}
            },
        }
        panic!()
    }
}

pub enum GameError {
}
