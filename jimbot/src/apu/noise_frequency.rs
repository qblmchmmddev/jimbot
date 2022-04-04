use std::process::id;

pub struct NoiseFrequency {
    nr43: u8,
    frequency_timer: u16,
}

impl Default for NoiseFrequency {
    fn default() -> Self {
        Self {
            nr43: 0,
            frequency_timer: 0,
        }
    }
}

impl NoiseFrequency {
    pub fn restart(&mut self) {
        self.frequency_timer = self.initial_frequency();
    }

    pub fn set_nr43(&mut self, nr43: u8) {
        self.nr43 = nr43;
    }

    pub fn initial_frequency(&self) -> u16 {
        (self.divisor() as u16) << (self.shift() as u16)
    }

    fn divisor(&self) -> u8 {
        let bits = self.nr43 & 0b111;
        match bits {
            0 => 8,
            1 => 16,
            2 => 32,
            3 => 48,
            4 => 64,
            5 => 80,
            6 => 96,
            7 => 112,
            _ => panic!("Unknown divisor: {}", bits)
        }
    }

    fn shift(&self) -> u8 {
        (self.nr43 >> 4) & 0b1111
    }

    pub fn is_width_mode(&self) -> bool {
        (self.nr43 >> 3) & 1 == 1
    }

    pub fn cycle(&mut self) -> bool {
        if self.initial_frequency() == 0 { return false }
        self.frequency_timer -= 1;
        if self.frequency_timer == 0 {
            self.restart();
            true
        } else {
            false
        }
    }
}