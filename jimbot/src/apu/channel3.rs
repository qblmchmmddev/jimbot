use crate::apu::envelope::Envelope;
use crate::apu::frequency::Frequency;
use crate::apu::frequency_sweep::FrequencySweep;
use crate::apu::length::Length;
use crate::apu::wave_length::WaveLength;
use crate::cpu::registers::R8::P;

pub struct Channel3 {
    enable: bool,
    nr30: u8,
    nr31: u8,
    nr32: u8,
    nr33: u8,
    nr34: u8,
    wave_pattern_ram: [u8;0x10], // FF30-FF3F
    wave_pattern: [u8; 32],
    wave_index: usize,
    frequency: Frequency,
    length: Length,
}

impl Default for Channel3 {
    fn default() -> Self {
        Self {
            enable: false,
            nr30: 0,
            nr31: 0,
            nr32: 0,
            nr33: 0,
            nr34: 0,
            wave_pattern_ram: [0;0x10],
            wave_pattern: [0; 32],
            wave_index: 0,
            frequency: Frequency::default(),
            length: 0.into(),
        }
    }
}

impl Channel3 {
    pub fn restart(&mut self) {
        self.wave_index = 0;
        self.length.restart();
        self.frequency.restart();
    }

    pub fn cycle(&mut self) {
        if !self.enable { return; }
        if self.frequency.cycle() {
            self.wave_index = (self.wave_index + 1) % self.wave_pattern.len();
        }
    }

    pub fn clock(&mut self, step: u8) {
        if !self.enable { false; }
        match step {
            0 => if self.length.clock_length() {
                self.enable = false;
                self.nr30 = 0;
            },
            2 => if self.length.clock_length() {
                self.enable = false;
                self.nr30 = 0;
            }// clock length and sweep// clock length and sweep
            4 => if self.length.clock_length() {
                self.enable = false;
                self.nr30 = 0;
            },
            6 => if self.length.clock_length() {
                self.enable = false;
                self.nr30 = 0;
            }// clock length and sweep
            _ => { }
        }
    }

    pub fn get_data(&self) -> f32 {
        if self.enable {
            let amp = (self.wave_pattern[self.wave_index] >> self.output_shift()) as f32;
            amp
        } else {
            0.0
        }
    }

    fn output_shift(&self) -> u8 {
        match (self.nr32 >> 5) & 0b11 {
            0 => 4,
            1 => 0,
            2 => 1,
            3 => 2,
            _ => panic!("Unknown output shift pattern: {}", self.nr32)
        }
    }

    pub fn set(&mut self, address: usize, val: u8) {
        match address {
            0xFF1A => {
                self.nr30 = val;
                if (val >> 7) & 1 == 1 {
                    self.enable = true;
                } else {
                    self.enable = false;
                }
            }
            0xFF1B => {
                self.nr31 = val;
                self.length.set(val);
            }
            0xFF1C => {
                self.nr32 = val;
            }
            0xFF1D => {
                self.nr33 = val;
                self.frequency.set_nrx3(val);
            }
            0xFF1E => {
                self.nr34 = val;
                self.frequency.set_nrx4(val);
                if (val >> 6) & 1 == 1 { self.length.enable_length() }
                if (val >> 7) & 1 == 1 {
                    self.restart();
                }
            },
            0xFF30..=0xFF3f => {
                self.wave_pattern_ram[address - 0xFF30] = val;
                for i in 0..0x10 {
                    let w = self.wave_pattern_ram[i];
                    self.wave_pattern[i * 2] = w >> 4;
                    self.wave_pattern[i * 2 + 1] = w & 0xF;
                }
            },
            _ => panic!("SET APU CHANNEL 2: {:#06x}->{:#04x}", address, val)
        }
    }

    pub fn get(&self, address: usize) -> u8 {
        match address {
            0xFF1A => self.nr30,
            0xFF1B => self.nr31,
            0xFF1C => self.nr32,
            0xFF1D => self.nr33,
            0xFF1E => self.nr34,
            0xFF30..=0xFF3f => self.wave_pattern_ram[address - 0xFF30],
            _ => panic!("GET APU CHANNEL 3: {:#06x}", address)
        }
    }
}



