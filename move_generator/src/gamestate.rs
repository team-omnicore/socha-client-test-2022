use crate::bitboard::Bitboard;
use crate::board::Board;
use crate::moves::{moewe_gen_moves, muschel_gen_moves, robbe_gen_moves, robbe_lookup, seestern_gen_moves, seestern_lookup, moewe_lookup, muschel_lookup};
use crate::{bit_loop, pretty_print_speed, square_of};
use separator::Separatable;
use std::fmt;
use std::fmt::Formatter;
use std::time::SystemTime;

pub struct Gamestate {
    pub board: Board,
    pub turn: u8,
    pub maximizing_player: bool,
    pub score: Score, // [max_player | min_player]
}

#[derive(Copy, Clone)]
pub union Score {
    pub short: u16,
    pub bytes: [u8; 2],
}

impl Gamestate {
    pub fn new_default() -> Self {
        Gamestate {
            board: Board::new_default(),
            turn: 1,
            maximizing_player: true,
            score: Score { short: 0 },
        }
    }

    pub fn begin_perft(&self, depth: u8) {
        let start = SystemTime::now();
        let count = self.perft(depth);

        let stop = start.elapsed().unwrap();

        let speed = count as f64 / stop.as_secs_f64();

        println!(
            "Perft {:>2} | {:>18} | {:>10.2?} | {:>20}",
            depth,
            count.separated_string(),
            stop,
            pretty_print_speed(speed)
        );
    }

    fn perft(&self, depth: u8) -> u64 {
        let seesterne = self.board.seesterne & self.board.friendly;
        let robben = self.board.robben & self.board.friendly;
        let muscheln = self.board.muscheln & self.board.friendly;
        let moewen = self.board.moewen & self.board.friendly;
        let enemy = self.board.enemy;
        let double = self.board.double;
        let friendly = self.board.friendly;
        let unoccupied = !self.board.friendly;

        let mut counter = 0;

        /////////////////////////////////////////ROBBE////////////////////////////////////////
        //Employs lookup for move gen

        let mut robbe = robben.bits;
        while robbe != 0 {
            let old_pos = square_of(robbe);
            let old_robbe = Bitboard::from(1 << old_pos);

            let legal = robbe_lookup(old_pos) & unoccupied;
            let mut mov = legal.bits;

            if depth > 0 {
                while mov != 0 {
                    if !self.is_game_over() {
                        let new_pos = square_of(mov);
                        let new_robbe = Bitboard::from(1 << new_pos);

                        let mut clone = self.clone();

                        //Legal moves do not overlap with double or friendly pieces

                        clone.board.robben &= !old_robbe;
                        clone.board.friendly &= !old_robbe;
                        clone.board.robben |= new_robbe;
                        clone.board.friendly |= new_robbe;

                        let mut points_added = 0u8;
                        if (new_robbe & enemy).bits != 0 {
                            clone.board.enemy &= !new_robbe;
                            clone.board.robben &= !new_robbe;
                            clone.board.seesterne &= !new_robbe;
                            clone.board.muscheln &= !new_robbe;
                            clone.board.moewen &= !new_robbe;

                            let overlaps = Bitboard::from(
                                ((double.bits & new_robbe.bits) != 0) as u64 * u64::MAX,
                            );
                            let clip_robbe = !new_robbe | !overlaps;
                            clone.board.double &= clip_robbe;
                            clone.board.robben &= clip_robbe;
                            clone.board.friendly &= clip_robbe;
                            points_added += 1 & overlaps.bits as u8;
                        }

                        //points_added += (new_robbe.bits & 0xFF00000000000000 != 0) as u8; //but robbe is not leichtfigur
                        unsafe {
                            clone.score.bytes[clone.maximizing_player as usize] += points_added
                        };

                        clone.maximizing_player = !self.maximizing_player;
                        clone.board.friendly.swap_with(&mut clone.board.enemy); //Meh
                                                                                //clone.board.rotate180(); //Dont need it, because robbe moves symmetrically

                        counter += clone.perft(depth - 1);
                    }
                    mov &= mov - 1;
                }
            } else {
                counter += legal.bits.count_ones() as u64;
            }
            robbe &= robbe - 1;
        }

        /////////////////////////////////////////MOEWE///////////////////////////////////////
        //Employs lookup for move gen

        let mut moewe = moewen.bits;
        while moewe != 0 {
            let old_pos = square_of(moewe);
            let old_moewe = Bitboard::from(1 << old_pos);

            let legal = moewe_lookup(old_pos) & unoccupied;
            let mut mov = legal.bits;

            if depth > 0 {
                while mov != 0 {
                    if !self.is_game_over() {
                        let new_pos = square_of(mov);
                        let new_moewe = Bitboard::from(1 << new_pos);

                        let mut clone = self.clone();

                        //Legal moves do not overlap with double or friendly pieces

                        clone.board.moewen &= !old_moewe;
                        clone.board.friendly &= !old_moewe;
                        clone.board.moewen |= new_moewe;
                        clone.board.friendly |= new_moewe;

                        let mut points_added = 0u8;
                        if (new_moewe & enemy).bits != 0 {
                            clone.board.enemy &= !new_moewe;
                            clone.board.robben &= !new_moewe;
                            clone.board.seesterne &= !new_moewe;
                            clone.board.muscheln &= !new_moewe;
                            clone.board.moewen &= !new_moewe;

                            let overlaps = Bitboard::from(
                                ((double.bits & new_moewe.bits) != 0) as u64 * u64::MAX,
                            );
                            let clip_moewe = !new_moewe | !overlaps;
                            clone.board.double &= clip_moewe;
                            clone.board.moewen &= clip_moewe;
                            clone.board.friendly &= clip_moewe;
                            points_added += 1 & overlaps.bits as u8;
                        }

                        points_added += self.lf_calculate_points(new_moewe);

                        unsafe {
                            clone.score.bytes[clone.maximizing_player as usize] += points_added
                        };

                        clone.maximizing_player = !self.maximizing_player;
                        clone.board.friendly.swap_with(&mut clone.board.enemy);
                        //clone.board.rotate180(); //Dont need it, because moewe moves symmetrically

                        counter += clone.perft(depth - 1);
                    }
                    mov &= mov - 1;
                }
            } else {
                counter += legal.bits.count_ones() as u64;
            }
            moewe &= moewe - 1;
        }

        /////////////////////////////////////////SEESTERN///////////////////////////////////////
        //Employs lookup for move gen

        let mut seestern = seesterne.bits;
        while seestern != 0 {
            let old_pos = square_of(seestern);
            let old_seestern = Bitboard::from(1 << old_pos);

            let legal = seestern_lookup(old_pos, self.maximizing_player) & unoccupied;
            let mut mov = legal.bits;

            if depth > 0 {
                while mov != 0 {
                    if !self.is_game_over() {
                        let new_pos = square_of(mov);
                        let new_seestern = Bitboard::from(1 << new_pos);

                        let mut clone = self.clone();

                        //Legal moves do not overlap with double or friendly pieces

                        clone.board.seesterne &= !old_seestern;
                        clone.board.friendly &= !old_seestern;
                        clone.board.seesterne |= new_seestern;
                        clone.board.friendly |= new_seestern;

                        let mut points_added = 0u8;
                        if (new_seestern & enemy).bits != 0 {
                            clone.board.enemy &= !new_seestern;
                            clone.board.robben &= !new_seestern;
                            clone.board.seesterne &= !new_seestern;
                            clone.board.muscheln &= !new_seestern;
                            clone.board.seesterne &= !new_seestern;

                            let overlaps = Bitboard::from(
                                ((double.bits & new_seestern.bits) != 0) as u64 * u64::MAX,
                            );
                            let clip_seestern = !new_seestern | !overlaps;
                            clone.board.double &= clip_seestern;
                            clone.board.seesterne &= clip_seestern;
                            clone.board.friendly &= clip_seestern;
                            points_added += 1 & overlaps.bits as u8;
                        }

                        points_added += self.lf_calculate_points(new_seestern);

                        unsafe {
                            clone.score.bytes[clone.maximizing_player as usize] += points_added
                        };

                        clone.maximizing_player = !self.maximizing_player;
                        clone.board.friendly.swap_with(&mut clone.board.enemy);
                        //clone.board.rotate180(); //Dont need it, because seestern moves symmetrically

                        counter += clone.perft(depth - 1);
                    }
                    mov &= mov - 1;
                }
            } else {
                counter += legal.bits.count_ones() as u64;
            }
            seestern &= seestern - 1;
        }

        /////////////////////////////////////////MUSCHEL///////////////////////////////////////
        //Employs lookup for move gen

        let mut muschel = muscheln.bits;
        while muschel != 0 {
            let old_pos = square_of(muschel);
            let old_muschel = Bitboard::from(1 << old_pos);

            let legal = muschel_lookup(old_pos, self.maximizing_player) & unoccupied;
            let mut mov = legal.bits;

            if depth > 0 {
                while mov != 0 {
                    if !self.is_game_over() {
                        let new_pos = square_of(mov);
                        let new_muschel = Bitboard::from(1 << new_pos);

                        let mut clone = self.clone();

                        //Legal moves do not overlap with double or friendly pieces

                        clone.board.muscheln &= !old_muschel;
                        clone.board.friendly &= !old_muschel;
                        clone.board.muscheln |= new_muschel;
                        clone.board.friendly |= new_muschel;

                        let mut points_added = 0u8;
                        if (new_muschel & enemy).bits != 0 {
                            clone.board.enemy &= !new_muschel;
                            clone.board.robben &= !new_muschel;
                            clone.board.muscheln &= !new_muschel;
                            clone.board.muscheln &= !new_muschel;
                            clone.board.muscheln &= !new_muschel;

                            let overlaps = Bitboard::from(
                                ((double.bits & new_muschel.bits) != 0) as u64 * u64::MAX,
                            );
                            let clip_muschel = !new_muschel | !overlaps;
                            clone.board.double &= clip_muschel;
                            clone.board.muscheln &= clip_muschel;
                            clone.board.friendly &= clip_muschel;
                            points_added += 1 & overlaps.bits as u8;
                        }

                        points_added += self.lf_calculate_points(new_muschel);

                        unsafe {
                            clone.score.bytes[clone.maximizing_player as usize] += points_added
                        };

                        clone.maximizing_player = !self.maximizing_player;
                        clone.board.friendly.swap_with(&mut clone.board.enemy);
                        //clone.board.rotate180(); //Dont need it, because muschel moves symmetrically

                        counter += clone.perft(depth - 1);
                    }
                    mov &= mov - 1;
                }
            } else {
                counter += legal.bits.count_ones() as u64;
            }
            muschel &= muschel - 1;
        }

        return counter;
    }

    const fn lf_calculate_points(&self, bitboard: Bitboard) -> u8 {
        ((bitboard.bits & 0xFF00000000000000 & ((self.maximizing_player as u64) * u64::MAX)
            | bitboard.bits & 0xFF & ((!self.maximizing_player as u64) * u64::MAX))
            != 0) as u8
    }

    const fn is_game_over(&self) -> bool {
        unsafe { self.score.bytes[0] >= 2 || self.score.bytes[1] >= 2 }
    }
}

pub const fn overlaps(rhs: Bitboard, lhs: Bitboard) -> Bitboard {
    Bitboard::from(((rhs.bits & lhs.bits) != 0) as u64 * u64::MAX)
}

impl Clone for Gamestate {
    fn clone(&self) -> Self {
        Gamestate {
            board: self.board.clone(),
            turn: self.turn,
            maximizing_player: self.maximizing_player,
            score: self.score.clone(),
        }
    }
}
