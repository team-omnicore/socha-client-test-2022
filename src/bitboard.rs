use crate::r#move::Move;
use core::fmt;
use std::fmt::Formatter;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
use thincollections::thin_v64::V64;

#[macro_export]
macro_rules! pos_from_coords {
    ($x:expr, $y:expr) => {
        ($y << 3) + (7 - $x)
    };
}

#[macro_export]
macro_rules! pos_from_server_coords {
    ($x:expr, $y:expr) => {
        ((7 - $y) << 3) + (7 - $x)
    };
}

#[macro_export]
macro_rules! mask_from_coords {
    ($x:expr, $y:expr) => {
        Bitmask {
            bits: 1 << (($y << 3) + 7 - $x),
        }
    };
}

#[derive(Copy)]
pub struct Bitmask {
    pub bits: u64,
}

impl Bitmask {
    /// Constructs a new Bitmask with all bits initialised to 0.
    ///
    /// returns: `Bitmask `
    ///
    pub const fn new() -> Self {
        Bitmask { bits: 0 }
    }

    /// Constructs a new Bitmask with the bit at the given coordinate set to 1.
    ///
    /// # Arguments
    ///
    /// * `x`: the x component of the coordinate
    /// * `y`: the y component of the coordinate
    ///
    /// returns: `Bitmask `
    ///
    pub const fn from_coords(x: u8, y: u8) -> Self {
        Bitmask {
            bits: 1 << ((y * 8) + 7 - x),
        }
    }

    /// Constructs a new Bitmask from given bits.
    ///
    /// # Arguments
    ///
    /// * `bits`: the bits to create a Bitmask from
    ///
    /// returns: `Bitmask `
    ///
    pub const fn from_bits(bits: u64) -> Self {
        Bitmask { bits }
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
    /// let mask = Bitmask::from_bits(1<<12);
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
    /// let mut mask = Bitmask::from_coords(4,5);
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
    /// let mask = Bitmask::new();
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
    /// let mut board = Bitmask::new();
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
    /// let mut board = Bitmask::new();
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
    /// let mut board = Bitmask::new();
    /// board.flip(12); //Flips the bit at (2,1)
    /// ```
    pub fn flip(&mut self, pos: u8) {
        self.bits ^= 1 << pos
    }

    /// Clears all the bits of this bitmask, setting them to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Bitmask::from_bits(0xA600D1DEA);
    /// board.clear_all();
    ///
    /// //Bitmask is now 0x0
    /// ```
    pub fn clear_all(&mut self) {
        self.bits = 0
    }

    /// Sets all of the bits in this bitmask to 1.
    ///
    /// * `pos`: the position at which to flip the bit
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Bitmask::from_bits(0xA600D1DEA);
    /// board.set_all();
    ///
    /// //Bitmask is now 0xFFFFFFFFFFFFFFFF
    /// ```
    pub fn set_all(&mut self) {
        self.bits = u64::MAX
    }

    /// Efficiently rotates this bitmask 180°, by reversing it's bits.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Bitmask::from_bits(0xFFFF);
    /// println!("{:X}", board.bits.reverse_bits());
    ///
    /// //> FFFF000000000000
    /// ```
    pub fn reverse(&mut self) {
        self.bits = self.bits.reverse_bits();
    }

    /// Efficiently swaps this bitmask with another.
    ///
    /// # Arguments
    ///
    /// * `other`: the bitboard to switch with
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mask1 = Bitmask::from_bits(0x69);
    /// let mut mask2 = Bitmask::from_bits(0x42);
    ///
    /// mask1.swap_with(&mut mask2);
    ///
    /// //mask1 now has the value of mask2, and vice versa
    /// ```
    pub fn swap_with(&mut self, other: &mut Bitmask) {
        self.bits ^= other.bits;
        other.bits ^= self.bits;
        self.bits ^= other.bits;
    }

    /// Efficiently calculates the indexes of all bits that are set to
    /// 1 in this bitmask.
    ///
    /// # Arguments
    ///
    /// returns: thincollections::thin_v64::V64<u8>
    ///
    /// # Examples
    ///
    /// ```
    /// let mask = Bitmask::from_bits(0b10100001);
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
}

impl Clone for Bitmask {
    fn clone(&self) -> Self {
        Bitmask { bits: self.bits }
    }
}

impl BitAnd for Bitmask {
    type Output = Bitmask;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitmask {
            bits: self.bits & rhs.bits,
        }
    }
}

impl BitOr for Bitmask {
    type Output = Bitmask;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitmask {
            bits: self.bits | rhs.bits,
        }
    }
}

impl BitXor for Bitmask {
    type Output = Bitmask;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitmask {
            bits: self.bits ^ rhs.bits,
        }
    }
}

impl BitAndAssign for Bitmask {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits
    }
}

impl BitOrAssign for Bitmask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits
    }
}

impl BitXorAssign for Bitmask {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bits ^= rhs.bits
    }
}

impl Not for Bitmask {
    type Output = Bitmask;

    fn not(self) -> Self::Output {
        Bitmask { bits: !self.bits }
    }
}

impl fmt::Display for Bitmask {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut copy = Bitmask::from_bits(self.bits);
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

impl fmt::Debug for Bitmask {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:066b}", self.bits)
    }
}
