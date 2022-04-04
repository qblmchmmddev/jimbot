use crate::apu::APU;

pub struct WaveLength {
    nrx1: u8,
    current_wave_duty_position: u8,
    is_length_enable: bool,
    length_timer: u8,
}

impl From<u8> for WaveLength { fn from(from: u8) -> Self { WaveLength::new(from) } }

impl From<WaveLength> for u8 { fn from(from: WaveLength) -> Self { from.nrx1 } }

impl WaveLength {
    pub fn new(nrx1: u8) -> Self {
        Self {
            length_timer: 0,
            nrx1,
            current_wave_duty_position: 0,
            is_length_enable: false,
        }
    }

    pub fn restart(&mut self) {
        self.current_wave_duty_position = 0;
        self.length_timer = 64 - self.length();
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

    pub fn get_amp(&self) -> u8 {
        self.wave()[self.current_wave_duty_position as usize]
    }

    pub fn enable_length(&mut self) {
        self.is_length_enable = true;
    }

    pub fn set(&mut self, nrx1: u8) {
        self.nrx1 = nrx1;
    }

    pub fn next(&mut self) {
        self.current_wave_duty_position = (self.current_wave_duty_position + 1) % 8;
    }

    pub fn wave_pattern_duty(&self) -> u8 {
        (self.nrx1 >> 6) & 0b11
    }

    pub fn wave(&self) -> [u8; 8] {
        APU::PREDETERMINE_SQUARE_WAVES[self.wave_pattern_duty() as usize]
    }

    pub fn length(&self) -> u8 {
        self.nrx1 & 0b1_1111
    }
}