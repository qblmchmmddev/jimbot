use crate::apu::envelope::Envelope;
use crate::apu::frequency::Frequency;
use crate::apu::frequency_sweep::FrequencySweep;
use crate::apu::noise_frequency::NoiseFrequency;
use crate::apu::wave_length::WaveLength;
use crate::cpu::registers::R8::P;

pub struct Channel4 {
    enable: bool,
    lfsr: u16,
    nr41: u8,
    nr42: u8,
    nr43: u8,
    nr44: u8,
    frequency: NoiseFrequency,
    wave_length: WaveLength,
    envelope: Envelope,
}

impl Default for Channel4 {
    fn default() -> Self {
        Self {
            enable: false,
            lfsr: 0,
            nr41: 0,
            nr42: 0,
            nr43: 0,
            nr44: 0,
            frequency: NoiseFrequency::default(),
            wave_length: 0.into(),
            envelope: 0.into(),
        }
    }
}

impl Channel4 {
    pub fn restart(&mut self) {
        self.lfsr = 0b0111_1111_1111_1111;
        self.wave_length.restart();
        self.envelope.restart();
        self.frequency.restart();
    }

    pub fn cycle(&mut self) {
        if !self.enable { return; }
        if self.frequency.cycle() {
            let xor_result = (self.lfsr & 0b1) ^ ((self.lfsr >> 1) & 1);
            self.lfsr = (self.lfsr >> 1) | (xor_result << 14);
            if self.frequency.is_width_mode() {
                self.lfsr &= !(1 >> 6);
                self.lfsr |= xor_result << 6;
            }
        }
    }

    pub fn clock(&mut self, step: u8) {
        if !self.enable { return; }
        match step {
            0 => if self.wave_length.clock_length() {
                self.enable = false;
                self.nr44 = 0;
            },
            2 => if self.wave_length.clock_length() {
                self.enable = false;
                self.nr44 = 0;
            }// clock length and sweep// clock length and sweep
            4 => if self.wave_length.clock_length() {
                self.enable = false;
                self.nr44 = 0;
            },
            6 => if self.wave_length.clock_length() {
                self.enable = false;
                self.nr44 = 0;
            }// clock length and sweep
            7 => self.envelope.clock(),
            _ => {}
        }
    }

    pub fn get_data(&self) -> f32 {
        if self.enable {
            let amp = ((self.lfsr & 1) ^ 1) as f32;
            let vol = self.envelope.current_volume() as f32;
            (amp * vol)
        } else {
            0.0
        }
    }

    pub fn set(&mut self, address: usize, val: u8) {
        match address {
            0xFF20 => {
                self.nr41 = val;
                self.wave_length.set(val);
            }
            0xFF21 => {
                self.nr42 = val;
                self.envelope.set(val);
            }
            0xFF22 => {
                self.nr43 = val;
                self.frequency.set_nr43(val);
            }
            0xFF23 => {
                self.nr44 = val;
                if (val >> 7) & 1 == 1 {
                    self.enable = true;
                    if (val >> 6) & 1 == 1 { self.wave_length.enable_length() }
                    self.restart();
                }
            }
            _ => panic!("SET APU CHANNEL 4: {:#06x}->{:#04x}", address, val)
        }
    }

    pub fn get(&self, address: usize) -> u8 {
        match address {
            0xFF20 => self.nr41,
            0xFF21 => self.nr42,
            0xFF22 => self.nr43,
            0xFF23 => self.nr44,
            _ => panic!("GET APU CHANNEL 4: {:#06x}", address)
        }
    }
}



