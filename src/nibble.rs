#[derive(Debug, Copy, Clone)]
pub struct Nibble {
    data: u8,
}

impl Nibble {
    pub const fn new() -> Self {
        Nibble { data: 0 }
    }

    pub fn get_right(&self) -> u8 {
        self.data & 0x0F
    }

    pub fn get_left(&self) -> u8 {
        self.data >> 4
    }

    pub fn set_right(&mut self, value: u8) {
        self.data &= 0xF0;
        self.data |= value;
    }

    pub fn set_left(&mut self, value: u8) {
        self.data &= 0x0F;
        self.data |= value << 4;
    }
}
