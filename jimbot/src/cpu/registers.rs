use crate::cpu::fflag::FFlag;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum R16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum R8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    S,
    P,
    PCl,
    PCh,
}

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    s: u8,
    p: u8,
    pcl: u8,
    pch: u8,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            s: 0,
            p: 0,
            pcl: 0,
            pch: 0,
        }
    }
}

impl Registers {
    const Z_MASK: u8 = 0b1000_0000;
    const N_MASK: u8 = 0b0100_0000;
    const H_MASK: u8 = 0b0010_0000;
    const C_MASK: u8 = 0b0001_0000;

    pub fn get16i(&mut self, r16: R16, by: u16) -> u16 {
        let r = self.get16(r16);
        self.set16(r16, r + by);
        r
    }

    pub fn get16d(&mut self, r16: R16, by: u16) -> u16 {
        let r = self.get16(r16);
        self.set16(r16, r - by);
        r
    }

    pub fn getd16(&mut self, r16: R16, by: u16) -> u16 {
        let r = self.get16(r16);
        self.set16(r16, r - by);
        r - by
    }

    pub fn inc16(&mut self, r16: R16, by: u16) {
        self.set16(r16, self.get16(r16) + by);
    }

    pub fn dec16(&mut self, r16: R16, by: u16) {
        self.set16(r16, self.get16(r16) - by);
    }

    pub fn set16(&mut self, r16: R16, to: u16) {
        match r16 {
            R16::AF => {
                self.a = ((to >> 8) & 0xFF) as u8;
                self.f = (to & 0xF0) as u8;
            }
            R16::BC => {
                self.b = ((to >> 8) & 0xFF) as u8;
                self.c = (to & 0xFF) as u8;
            }
            R16::DE => {
                self.d = ((to >> 8) & 0xFF) as u8;
                self.e = (to & 0xFF) as u8;
            }
            R16::HL => {
                self.h = ((to >> 8) & 0xFF) as u8;
                self.l = (to & 0xFF) as u8;
            }
            R16::SP => {
                self.s = ((to >> 8) & 0xFF) as u8;
                self.p = (to & 0xFF) as u8;
            }
            R16::PC => {
                self.pch = ((to >> 8) & 0xFF) as u8;
                self.pcl = (to & 0xFF) as u8;
            }
        };
    }

    pub fn get16(&self, r16: R16) -> u16 {
        match r16 {
            R16::AF => u16::from_be_bytes([self.a, self.f]),
            R16::BC => u16::from_be_bytes([self.b, self.c]),
            R16::DE => u16::from_be_bytes([self.d, self.e]),
            R16::HL => u16::from_be_bytes([self.h, self.l]),
            R16::SP => u16::from_be_bytes([self.s, self.p]),
            R16::PC => u16::from_be_bytes([self.pch, self.pcl]),
        }
    }

    pub fn set8(&mut self, r8: R8, to: u8) {
        match r8 {
            R8::A => self.a = to,
            R8::B => self.b = to,
            R8::C => self.c = to,
            R8::D => self.d = to,
            R8::E => self.e = to,
            R8::F => self.f = to & 0xF0,
            R8::H => self.h = to,
            R8::L => self.l = to,
            R8::S => self.s = to,
            R8::P => self.p = to,
            R8::PCl => self.pcl = to,
            R8::PCh => self.pch = to,
        }
    }

    pub fn get8(&self, r8: R8) -> u8 {
        match r8 {
            R8::A => self.a,
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::F => self.f,
            R8::H => self.h,
            R8::L => self.l,
            R8::S => self.s,
            R8::P => self.p,
            R8::PCl => self.pcl,
            R8::PCh => self.pch,
        }
    }

    pub fn set_f(&mut self, flag: FFlag, set: bool) {
        let mask = Self::flag_mask(flag);
        if set {
            self.f |= mask;
        } else {
            self.f &= !mask;
        }
    }

    pub fn get_f(&self, flag: FFlag) -> bool {
        let mask = Self::flag_mask(flag);
        self.f & mask == mask
    }

    fn flag_mask(flag: FFlag) -> u8 {
        match flag {
            FFlag::Z => Self::Z_MASK,
            FFlag::N => Self::N_MASK,
            FFlag::H => Self::H_MASK,
            FFlag::C => Self::C_MASK,
        }
    }
}