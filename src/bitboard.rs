use crate::vec2::Coords;
use core::fmt;
use std::fmt::Formatter;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
use thincollections::thin_v64::V64;

#[macro_export]
macro_rules! position {
    ($x:expr, $y:expr) => {
        (($y << 3) + (7 - $x)) as u8
    };
}
#[macro_export]
macro_rules! coords {
    ($pos:expr) => {
        Vec2 {
            x: (7 - ($pos & 7)) as i8,
            y: ($pos >> 3) as i8,
        }
    };
}
#[macro_export]
macro_rules! rotate_anti_90 {
    ($pos: expr) => {
        ((($pos >> 3) | ($pos << 3)) & 63) ^ 56
    };
}
#[macro_export]
macro_rules! rotate_clock_90 {
    ($pos: expr) => {
        ((($pos >> 3) | ($pos << 3)) & 63) ^ 7
    };
}
#[macro_export]
macro_rules! pos_from_server_coords {
    ($x:expr, $y:expr) => {
        (($x << 3) + (7 - $y)) as u8
    };
}
#[macro_export]
macro_rules! mask_from_coords {
    ($x:expr, $y:expr) => {
        Bitboard {
            bits: 1 << (($y << 3) + 7 - $x),
        }
    };
}

#[derive(Copy)]
pub struct Bitboard {
    pub bits: u64,
}

const K1: u64 = 0x00FF00FF00FF00FF;
const K2: u64 = 0x0000FFFF0000FFFF;

const D1: u64 = 0x5500550055005500;
const D2: u64 = 0x3333000033330000;
const D3: u64 = 0x0f0f0f0f00000000;

const H1: u64 = 0x5555555555555555;
const H2: u64 = 0x3333333333333333;
const H3: u64 = 0x0F0F0F0F0F0F0F0F;

impl Bitboard {
    /// Constructs a new Bitboard with all bits initialised to 0.
    ///
    /// returns: `Bitboard `
    ///
    pub const fn new() -> Self {
        Bitboard { bits: 0 }
    }

    /// Returns the state of the bit at the given position.
    ///
    /// This function will not panic, if an out of bounds value is passed as the position.
    ///
    /// # Arguments
    ///
    /// * `pos`: the position of the bit
    ///
    /// returns: true, if the bit is set to 1
    ///
    /// # Examples
    ///
    /// ```
    /// let mask = Bitboard::from_bits(1<<12);
    /// println!("{}", mask.get(12));
    ///
    /// //> true
    /// ```
    pub const fn get(&self, pos: u8) -> bool {
        (self.bits >> pos & 1u64) != 0
    }

    /// Returns the state of the bit at the given coordinates.
    ///
    /// This function will not panic, if ouf of bounds values are passed as coordinates.
    ///
    /// # Arguments
    ///
    /// * `x`: the x component of the coordinate
    /// * `y`: the y component of the coordinate
    ///
    /// returns: true, if the bit is set to 1
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mask = Bitboard::from_coords(4,5);
    /// println!("{}", mask.get_at_coords(4,5));
    ///
    /// //> true
    /// ```
    pub const fn get_at_coords(&self, x: u8, y: u8) -> bool {
        (self.bits >> (y << 3)) >> (7 - x) & 1u64 != 0
    }

    /// Sets the bit at at the given position to 1.
    ///
    /// This function will not panic, if an out of bounds value is passed as the position.
    ///
    /// # Arguments
    ///
    /// * `pos`: the position of the bit
    ///
    /// # Examples
    ///
    /// ```
    /// let mask = Bitboard::new();
    /// mask.set(0);
    ///
    /// //Mask now has a set bit at the 0th position
    /// ```
    pub fn set(&mut self, pos: u8) {
        self.bits |= 1 << pos
    }

    /// Sets the bit at the given coordinates. The coordinates are in cartesian form,
    /// with `(0,0)` being the bottom left coordinate, and `(7,7)` being the top right.<br>
    ///
    /// This function will not panic, if ouf of bounds values are passed as coordinates.
    ///
    /// # Arguments
    /// * `x`: the x component of the coordinate
    /// * `y`: the y component of the coordinate
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Bitboard::new();
    /// board.set_at_coords(4,2);
    /// ```
    pub fn set_at_coords(&mut self, x: u8, y: u8) {
        self.bits |= 1 << ((y << 3) + 7 - x)
    }

    /// Clears the bit at the given position, setting it to 0. <br>
    ///
    /// The function will not panic, if an out of bounds value is passed as the position.
    ///
    /// # Arguments
    ///
    /// * `pos`: the position at which to clear the bit
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Bitboard::new();
    /// board.clear(12); //Clears the bit at (2,1)
    /// ```
    pub fn clear(&mut self, pos: u8) {
        self.bits &= !(1 << pos)
    }

    /// Flips the bit at the given position, toggling its value.
    ///
    /// The function will not panic, if an out of bounds value is passed as the position.
    ///
    /// # Arguments
    ///
    /// * `pos`: the position at which to flip the bit
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Bitboard::new();
    /// board.flip(12); //Flips the bit at (2,1)
    /// ```
    pub fn flip_bit(&mut self, pos: u8) {
        self.bits ^= 1 << pos
    }

    /// Clears all the bits of this bitboard, setting them to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Bitboard::from_bits(0xA600D1DEA);
    /// board.clear_all();
    ///
    /// //Bitboard is now 0x0
    /// ```
    pub fn clear_all(&mut self) {
        self.bits = 0
    }

    /// Sets all of the bits in this bitboard to 1.
    ///
    /// * `pos`: the position at which to flip the bit
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Bitboard::from_bits(0xA600D1DEA);
    /// board.set_all();
    ///
    /// //Bitboard is now 0xFFFFFFFFFFFFFFFF
    /// ```
    pub fn set_all(&mut self) {
        self.bits = u64::MAX
    }

    /// Efficiently rotates this bitboard 180°, by reversing it's bits.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Bitboard::from_bits(0xFFFF);
    /// println!("{:X}", board.bits.reverse_bits());
    ///
    /// //> FFFF000000000000
    /// ```
    pub fn reverse(&mut self) {
        self.bits = self.bits.reverse_bits();
    }

    /// Efficiently swaps this bitboard with another.
    ///
    /// # Arguments
    ///
    /// * `other`: the bitboard to switch with
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mask1 = Bitboard::from_bits(0x69);
    /// let mut mask2 = Bitboard::from_bits(0x42);
    ///
    /// mask1.swap_with(&mut mask2);
    ///
    /// //mask1 now has the value of mask2, and vice versa
    /// ```
    pub fn swap_with(&mut self, other: &mut Bitboard) {
        self.bits ^= other.bits;
        other.bits ^= self.bits;
        self.bits ^= other.bits;
    }

    /// Efficiently calculates the indexes of all bits that are set to
    /// 1 in this bitboard.
    ///
    /// # Arguments
    ///
    /// returns: thincollections::thin_v64::V64<u8>
    ///
    /// # Examples
    ///
    /// ```
    /// let mask = Bitboard::from_bits(0b10100001);
    /// println!("{:?}", mask.get_set_bits());
    ///
    /// //> [0, 5, 7]
    /// ```
    pub fn get_set_bits(&self) -> V64<u8> {
        let mut out = V64::new();

        let mut bits = self.bits;

        let mut ones = bits.count_ones();
        let mut counter: u8 = 0;

        while ones > 0 {
            if bits & 0x1 != 0 {
                out.push(counter);
                ones -= 1;
            }
            bits >>= 1;
            counter += 1;
        }
        out
    }

    pub fn rotate90_clockwise(&self) -> Self {
        self.flip_vertical().flip_diagonal_a1_h8()
    }

    pub fn rotate90_anti_clockwise(&self) -> Self {
        self.flip_diagonal_a1_h8().flip_vertical()
    }

    pub fn flip_vertical(&self) -> Self {
        let mut x = self.bits;
        x = ((x >> 8) & K1) | ((x & K1) << 8);
        x = ((x >> 16) & K2) | ((x & K2) << 16);
        x = (x >> 32) | (x << 32);
        Bitboard { bits: x }
    }

    pub fn flip_horizontal(&self) -> Self {
        let mut x = self.bits;
        x = ((x >> 1) & H1) + 2 * (x & H1);
        x = ((x >> 2) & H2) + 4 * (x & H2);
        x = ((x >> 4) & H3) + 16 * (x & H3);
        Bitboard { bits: x }
    }

    pub fn flip_diagonal_a1_h8(&self) -> Self {
        let mut x = self.bits;
        let mut _t: u64 = 0;
        _t = D3 & (x ^ (x << 28));
        x ^= _t ^ (_t >> 28);
        _t = D2 & (x ^ (x << 14));
        x ^= _t ^ (_t >> 14);
        _t = D1 & (x ^ (x << 7));
        x ^= _t ^ (_t >> 7);
        Bitboard { bits: x }
    }

    pub fn rotate180(&self) -> Self {
        let mut x = self.bits;
        x = ((x >> 1) & H1) | ((x & H1) << 1);
        x = ((x >> 2) & H2) | ((x & H2) << 2);
        x = ((x >> 4) & H3) | ((x & H3) << 4);
        x = ((x >> 8) & K1) | ((x & K1) << 8);
        x = ((x >> 16) & K2) | ((x & K2) << 16);
        x = (x >> 32) | (x << 32);
        Bitboard { bits: x }
    }
}

impl From<u64> for Bitboard {
    fn from(bits: u64) -> Self {
        Bitboard { bits }
    }
}

impl From<Coords> for Bitboard {
    fn from(coords: Coords) -> Self {
        Bitboard {
            bits: 1 << ((coords.y * 8) + 7 - coords.x),
        }
    }
}

impl Clone for Bitboard {
    fn clone(&self) -> Self {
        Bitboard { bits: self.bits }
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard {
            bits: self.bits & rhs.bits,
        }
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard {
            bits: self.bits | rhs.bits,
        }
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard {
            bits: self.bits ^ rhs.bits,
        }
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bits ^= rhs.bits
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard { bits: !self.bits }
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut copy = Bitboard::from(self.bits);
        copy.reverse();

        let mut out: String = String::from("╔══════════════════════════╗\n║  ");
        out.push(if copy.get(0) { 'x' } else { '-' });
        for i in 1..64u8 {
            if i & 7 == 0 {
                out.push_str("  ║\n║  ")
            } else {
                out.push_str("  ");
            }
            out.push(if copy.get(i) { 'x' } else { '-' });
        }
        out.push_str("  ║\n╚══════════════════════════╝");
        write!(f, "{}", out)
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:066b}", self.bits)
    }
}
