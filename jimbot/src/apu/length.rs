use crate::apu::APU;

pub struct Length {
    nr31: u8,
    is_length_enable: bool,
    length_timer: u8,
}

impl From<u8> for Length { fn from(from: u8) -> Self { Length::new(from) } }

impl From<Length> for u8 { fn from(from: Length) -> Self { from.nr31 } }

impl Length {
    pub fn new(nr31: u8) -> Self {
        Self {
            length_timer: 0,
            nr31,
            is_length_enable: false,
        }
    }

    pub fn restart(&mut self) {
        self.length_timer = 255 - self.length();
    }

    pub fn clock_length(&mut self) -> bool {
        if !self.is_length_enable { return false; }
        self.length_timer -= 1;
        if self.length_timer == 0 {
            self.is_length_enable = false;
            return true;
        }
        false
    }

    pub fn enable_length(&mut self) {
        self.is_length_enable = true;
    }

    pub fn set(&mut self, nr31: u8) {
        self.nr31 = nr31;
    }

    pub fn length(&self) -> u8 { self.nr31 }
}