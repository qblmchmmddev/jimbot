pub struct TAC(u8);

impl From<u8> for TAC { fn from(from: u8) -> Self { TAC(from) } }

impl From<TAC> for u8 { fn from(from: TAC) -> Self { from.0 } }

impl TAC {
    pub fn is_timer_enable(&self) -> bool {
        (self.0 >> 2) & 1 == 1
    }

    pub fn clock_select(&self) -> u8 {
        let bits = self.0 & 0b11;
        match bits {
            0b00 => 9,
            0b01 => 3,
            0b10 => 5,
            0b11 => 7,
            _ => panic!("Unknown clock select {:b}", bits)
        }
    }
}