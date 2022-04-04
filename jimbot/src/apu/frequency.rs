use std::process::id;

pub struct Frequency {
    nrx3: u8,
    nrx4: u8,
    initial_frequency: u16,
    frequency_timer: u16,
}

impl Default for Frequency {
    fn default() -> Self {
        Self {
            nrx3: 0,
            nrx4: 0,
            initial_frequency: 0,
            frequency_timer: 0,
        }
    }
}

impl Frequency {
    pub fn restart(&mut self) {
        self.frequency_timer = (2048 - self.initial_frequency) * 4;
    }

    pub fn set_nrx3(&mut self, nrx3: u8) {
        self.nrx3 = nrx3;
        self.calculate_initial_frequency();
    }

    pub fn set_nrx4(&mut self, nrx4: u8) {
        self.nrx4 = nrx4;
        self.calculate_initial_frequency();
    }

    pub fn calculate_initial_frequency(&mut self) {
        self.initial_frequency = (((self.nrx4 & 0b111) as u16) << 8) | (self.nrx3 as u16);
    }

    pub fn cycle(&mut self) -> bool {
        if self.initial_frequency == 0 { return false; }
        self.frequency_timer -= 1;
        if self.frequency_timer == 0 {
            self.restart();
            true
        } else {
            false
        }
    }
    pub fn initial_frequency(&self) -> u16 {
        self.initial_frequency
    }

    pub fn set_new_frequency(&mut self, new_frequency: u16) {
        self.initial_frequency = new_frequency;
    }
}