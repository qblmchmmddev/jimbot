#[derive(Debug)]
pub enum Mode {
    HBlank,
    VBlank,
    OAMSearch,
    LCDTransfer,
}

#[derive(Copy, Clone)]
pub struct LCDSTAT(u8);

impl From<u8> for LCDSTAT { fn from(from: u8) -> Self { LCDSTAT(from) } }

impl From<LCDSTAT> for u8 { fn from(from: LCDSTAT) -> Self { from.0 } }

impl LCDSTAT {
    pub fn new(mode: Mode) -> Self {
        let mut stat = LCDSTAT(0);
        stat.set_mode(mode);
        stat
    }

    pub fn mode(&self) -> Mode {
        let bits = self.0 & 0b11;
        match bits {
            0b00 => Mode::HBlank,
            0b01 => Mode::VBlank,
            0b10 => Mode::OAMSearch,
            0b11 => Mode::LCDTransfer,
            _ => panic!("Unknown LCD Mode {:b}", bits),
        }
    }

    pub fn ly_eq_lyc(&self) -> bool { (self.0 >> 6) & 1 == 1 }
    pub fn oam_interrupt(&self) -> bool { (self.0 >> 5) & 1 == 1 }
    pub fn vblank_interrupt(&self) -> bool { (self.0 >> 4) & 1 == 1 }
    pub fn hblank_interrupt(&self) -> bool { (self.0 >> 3) & 1 == 1 }

    pub fn set_coincidence(&mut self, ly_eq_lyc: bool) {
        self.0 = if ly_eq_lyc { self.0 | 0b0000_0100 } else { self.0 & 0b1111_1011 };
    }

    pub fn coincidence(&self) -> bool {
        (self.0 >> 2) & 1 == 1
    }

    pub fn set_mode(&mut self, mode: Mode) {
        let bits: u8 = match mode {
            Mode::HBlank => 0b00,
            Mode::VBlank => 0b01,
            Mode::OAMSearch => 0b10,
            Mode::LCDTransfer => 0b11,
        };
        self.0 = (self.0 & 0b1111_1100) | bits;
    }
}