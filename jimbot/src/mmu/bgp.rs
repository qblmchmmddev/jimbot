pub struct BGP(u8);

impl From<u8> for BGP { fn from(from: u8) -> Self { BGP(from) } }

impl From<BGP> for u8 { fn from(from: BGP) -> Self { from.0 } }

impl BGP {
    pub fn get_color(&self, index: u8) -> u8 {
        match index {
            0 => (self.0 >> 0) & 0b11,
            1 => (self.0 >> 2) & 0b11,
            2 => (self.0 >> 4) & 0b11,
            3 => (self.0 >> 6) & 0b11,
            _ => panic!("BGP index out of range: {}", index)
        }
    }
}

pub struct OBP(u8);

impl From<u8> for OBP { fn from(from: u8) -> Self { OBP(from) } }

impl From<OBP> for u8 { fn from(from: OBP) -> Self { from.0 } }

impl OBP {
    pub fn get_color(&self, index: u8) -> u8 {
        match index {
            0 => 0,
            1 => (self.0 >> 2) & 0b11,
            2 => (self.0 >> 4) & 0b11,
            3 => (self.0 >> 6) & 0b11,
            _ => panic!("BGP index out of range: {}", index)
        }
    }
}