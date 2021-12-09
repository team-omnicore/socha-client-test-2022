use crate::bitboard::Bitboard;
use crate::board::Board;
use crate::moves::{
    moewe_lookup, muschel_lookup, robbe_gen_moves, robbe_lookup,
    seestern_gen_moves, seestern_lookup,
};
use crate::{pretty_print_speed, square_of};
use chrono::Local;
use separator::Separatable;

use std::io;
use std::io::Write;
use std::time::{Duration, SystemTime};

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

    pub fn new(board: Board) -> Self {
        Gamestate {
            board,
            turn: 1,
            maximizing_player: true,
            score: Score { short: 0 },
        }
    }

    pub fn perft_up_to(&self, depth: u8) {
        println!("|Depth|Move-count|Elapsed time|Speed|Multiplier|\n|---|---|---|---|---|");

        let mut last_time = 1f64;
        let mut last_count = 1f64;
        let mut time_multiplier = 1f64;
        let mut count_multiplier = 1f64;
        for i in 0..depth {
            let current_time = Local::now();
            let estimated_duration = Duration::from_secs_f64(time_multiplier * last_time);
            let estimated_finish = current_time
                .checked_add_signed(chrono::Duration::from_std(estimated_duration).unwrap())
                .unwrap();

            print!(
                "> Estimated time is {:.1?} - Working since {} - Finishes {}",
                estimated_duration,
                current_time.format("%Y-%m-%d %H:%M:%S"),
                estimated_finish.format("%Y-%m-%d %H:%M:%S")
            );
            io::stdout().flush();

            let start = SystemTime::now();
            let count = self.perft(i);
            let duration = start.elapsed().unwrap();

            count_multiplier = count as f64 / last_count;
            last_count = count as f64;

            time_multiplier = duration.as_secs_f64() / last_time;
            last_time = duration.as_secs_f64();

            let speed = count as f64 / duration.as_secs_f64();

            println!(
                "\rPerft {:>1} | {:>18} | {:>10.2?} | {:>16} | {:>3.1}x",
                i,
                count.separated_string(),
                duration,
                pretty_print_speed(speed),
                count_multiplier
            );
        }
    }

    pub fn perft(&self, depth: u8) -> u64 {

        unsafe {
            if self.score.bytes[0] == 2 {
                println!("{:?}", self.score.bytes);
            }
        }

        let seesterne = self.board.seesterne & self.board.friendly;
        let robben = self.board.robben & self.board.friendly;
        let muscheln = self.board.muscheln & self.board.friendly;
        let moewen = self.board.moewen & self.board.friendly;
        let enemy = self.board.enemy;
        let double = self.board.double;
        let unoccupied = !self.board.friendly;

        let mut counter = 0;

        /////////////////////////////////////////ROBBE////////////////////////////////////////
        //Employs lookup for move gen

        let mut robbe = robben.bits;
        while robbe != 0 {
            let old_pos = square_of(robbe);

            let legal = robbe_lookup(old_pos) & unoccupied;
            let mut mov = legal.bits;

            if depth > 0 {

                let old_robbe = Bitboard::from(1 << old_pos);
                let old_overlaps_double = overlaps(self.board.double, old_robbe);
                let double_clone = self.board.double &( !old_robbe | !old_overlaps_double );

                while mov != 0 {
                    if !self.is_game_over() {
                        let new_pos = square_of(mov);
                        let new_robbe = Bitboard::from(1 << new_pos);

                        let mut clone = self.clone();
                        clone.board.double = double_clone;

                        //Clear old FRIENDLY piece, set new piece
                        clone.board.robben &= !old_robbe;
                        clone.board.friendly &= !old_robbe;
                        clone.board.robben |= new_robbe;
                        clone.board.friendly |= new_robbe;

                        //Define points to be added to own score
                        let mut points_added = 0u8;

                        /// Move double from old piece to new piece
                        /// Branching:
                        /// ```rust
                        /// if double.get(old_pos) {
                        ///     double = double.clear(old_pos);
                        ///     double = double.set(new_pos);
                        /// }
                        /// ```
                        /// Non-branching:
                        /// ```rust
                        /// let overlaps = crate::softwarechallenge_2022::gamestate::overlaps(clone.board.double, old_robbe);
                        /// clone.board.double &= !old_robbe | !overlaps;          
                        /// clone.board.double |= new_robbe & overlaps;
                        /// ```
                        clone.board.double |= new_robbe & old_overlaps_double;

                        //If new piece overlaps with enemy's piece at that position
                        if (new_robbe & enemy).bits != 0 {

                            //Clear enemy's piece. Dont know which piece, so just clear all bitboards.
                            clone.board.enemy &= !new_robbe;
                            clone.board.robben &= !new_robbe;
                            clone.board.seesterne &= !new_robbe;
                            clone.board.moewen &= !new_robbe;
                            clone.board.muscheln &= !new_robbe;

                            /// We have already moved our double piece, if it was a double
                            /// piece. That means, if the new position is 1, there is a either a double
                            /// enemy piece, or we were a double friendly piece jumping on a single or double piece.
                            /// Each way, the field and double needs to be cleared, and we get a point.
                            clone.board.double ^= new_robbe;
                            
                            /// We have already moved our double piece, if it was a double
                            /// piece. That means, if the new position is 1, there is a either a double
                            /// enemy piece, or we were a double friendly piece jumping on a single or double piece.
                            /// Each way, the field and double needs to be cleared, and we get a point.
                            /// Branching:
                            /// ```rust
                            /// if double.get(new_pos) as u8 == 0 {
                            ///     clone.board.robben &= !new_robbe;
                            ///     clone.board.friendly &= !new_robbe;
                            ///     points_added += 1;
                            /// }
                            /// ```
                            /// Non-branching:
                            /// ```rust
                            /// let overlaps = crate::softwarechallenge_2022::gamestate::overlaps(double, new_robbe);
                            /// let clip_robbe = !new_robbe | overlaps;
                            /// clone.board.robben &= clip_robbe;
                            /// clone.board.friendly &= clip_robbe;
                            /// points_added += 1 & !overlaps.bits as u8;
                            /// ```
                            let new_overlaps_double = overlaps(double, new_robbe);
                            let clip_robbe = !new_robbe | new_overlaps_double;
                            clone.board.robben &= clip_robbe;
                            clone.board.friendly &= clip_robbe;
                            points_added += 1 & !new_overlaps_double.bits as u8;

                            //TODO error in algorithm - not accounting for moving double around, and not setting double correctly
                            /*let overlaps = overlaps(double, new_robbe);
                            let clip_robbe = !new_robbe | !overlaps;
                            clone.board.double &= clip_robbe;
                            clone.board.robben &= clip_robbe;
                            clone.board.friendly &= clip_robbe;
                            points_added += 1 & overlaps.bits as u8;

                             */
                        }

                        //points_added += (new_robbe.bits & 0xFF00000000000000 != 0) as u8; //but robbe is not leichtfigur
                        unsafe {
                            clone.score.bytes[clone.maximizing_player as usize] += points_added
                        };

                        clone.maximizing_player = !self.maximizing_player;
                        clone.board.friendly.swap_with(&mut clone.board.enemy); //Meh

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

            let legal = moewe_lookup(old_pos) & unoccupied;
            let mut mov = legal.bits;

            if depth > 0 {

                let old_moewe = Bitboard::from(1 << old_pos);

                let old_overlaps_double = overlaps(self.board.double, old_moewe);
                let double_clone = self.board.double & (!old_moewe | !old_overlaps_double);

                while mov != 0 {
                    if !self.is_game_over() {
                        let new_pos = square_of(mov);
                        let new_moewe = Bitboard::from(1 << new_pos);

                        let mut clone = self.clone();
                        clone.board.double = double_clone;

                        clone.board.moewen &= !old_moewe;
                        clone.board.friendly &= !old_moewe;
                        clone.board.moewen |= new_moewe;
                        clone.board.friendly |= new_moewe;

                        let mut points_added = 0u8;

                        ///look up!
                        clone.board.double |= new_moewe & old_overlaps_double;
                        
                        if (new_moewe & enemy).bits != 0 {
                            clone.board.enemy &= !new_moewe;
                            clone.board.robben &= !new_moewe;
                            clone.board.seesterne &= !new_moewe;
                            clone.board.muscheln &= !new_moewe;
                            clone.board.moewen &= !new_moewe;

                            clone.board.double ^= new_moewe;

                            let new_overlaps_double = overlaps(double, new_moewe);
                            let clip_moewe = !new_moewe | new_overlaps_double;
                            clone.board.moewen &= clip_moewe;
                            clone.board.friendly &= clip_moewe;
                            points_added += 1 & !new_overlaps_double.bits as u8;
                        }

                        points_added += self.lf_calculate_points(new_moewe);

                        unsafe {
                            clone.score.bytes[clone.maximizing_player as usize] += points_added
                        };

                        clone.maximizing_player = !self.maximizing_player;
                        clone.board.friendly.swap_with(&mut clone.board.enemy);

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

            let legal = seestern_lookup(old_pos, self.maximizing_player) & unoccupied;
            let mut mov = legal.bits;

            if depth > 0 {

                let old_seestern = Bitboard::from(1 << old_pos);
                let old_overlaps_double = overlaps(self.board.double, old_seestern);
                let double_clone = self.board.double & (!old_seestern | !old_overlaps_double);

                while mov != 0 {
                    if !self.is_game_over() {
                        let new_pos = square_of(mov);
                        let new_seestern = Bitboard::from(1 << new_pos);

                        let mut clone = self.clone();
                        clone.board.double = double_clone;

                        clone.board.seesterne &= !old_seestern;
                        clone.board.friendly &= !old_seestern;
                        clone.board.seesterne |= new_seestern;
                        clone.board.friendly |= new_seestern;

                        let mut points_added = 0u8;

                        clone.board.double |= new_seestern & old_overlaps_double;
                        
                        if (new_seestern & enemy).bits != 0 {
                            clone.board.enemy &= !new_seestern;
                            clone.board.robben &= !new_seestern;
                            clone.board.seesterne &= !new_seestern;
                            clone.board.moewen &= !new_seestern;
                            clone.board.muscheln &= !new_seestern;

                            clone.board.double ^= new_seestern;

                            let new_overlaps_double = overlaps(double, new_seestern);
                            let clip_seestern = !new_seestern | new_overlaps_double;
                            clone.board.seesterne &= clip_seestern;
                            clone.board.friendly &= clip_seestern;
                            points_added += 1 & !new_overlaps_double.bits as u8;
                        }

                        points_added += self.lf_calculate_points(new_seestern);

                        unsafe {
                            clone.score.bytes[clone.maximizing_player as usize] += points_added
                        };

                        clone.maximizing_player = !self.maximizing_player;
                        clone.board.friendly.swap_with(&mut clone.board.enemy);

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

            let legal = muschel_lookup(old_pos, self.maximizing_player) & unoccupied;
            let mut mov = legal.bits;

            if depth > 0 {

                let old_muschel = Bitboard::from(1 << old_pos);

                while mov != 0 {
                    if !self.is_game_over() {
                        let new_pos = square_of(mov);
                        let new_muschel = Bitboard::from(1 << new_pos);

                        let mut clone = self.clone();

                        clone.board.muscheln &= !old_muschel;
                        clone.board.friendly &= !old_muschel;
                        clone.board.muscheln |= new_muschel;
                        clone.board.friendly |= new_muschel;

                        let mut points_added = 0u8;

                        let old_overlaps_double = overlaps(clone.board.double, old_muschel);
                        clone.board.double &= !old_muschel | !old_overlaps_double;
                        clone.board.double |= new_muschel & old_overlaps_double;

                        if (new_muschel & enemy).bits != 0 {
                            clone.board.enemy &= !new_muschel;
                            clone.board.robben &= !new_muschel;
                            clone.board.seesterne &= !new_muschel;
                            clone.board.moewen &= !new_muschel;
                            clone.board.muscheln &= !new_muschel;

                            clone.board.double ^= new_muschel;

                            let new_overlaps_double = overlaps(double, new_muschel);
                            let clip_muschel = !new_muschel | new_overlaps_double;
                            clone.board.muscheln &= clip_muschel;
                            clone.board.friendly &= clip_muschel;
                            points_added += 1 & !new_overlaps_double.bits as u8;
                        }

                        points_added += self.lf_calculate_points(new_muschel);

                        unsafe {
                            clone.score.bytes[clone.maximizing_player as usize] += points_added
                        };

                        clone.maximizing_player = !self.maximizing_player;
                        clone.board.friendly.swap_with(&mut clone.board.enemy);

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
