use crate::apu::envelope::Envelope;
use crate::apu::frequency::Frequency;
use crate::apu::frequency_sweep::FrequencySweep;
use crate::apu::wave_length::WaveLength;
use crate::cpu::registers::R8::P;

pub struct Channel2 {
    enable: bool,
    nr21: u8,
    nr22: u8,
    nr23: u8,
    nr24: u8,
    frequency: Frequency,
    wave_length: WaveLength,
    envelope: Envelope,
}

impl Default for Channel2 {
    fn default() -> Self {
        Self {
            enable: false,
            nr21: 0,
            nr22: 0,
            nr23: 0,
            nr24: 0,
            frequency: Frequency::default(),
            wave_length: 0.into(),
            envelope: 0.into(),
        }
    }
}

impl Channel2 {
    pub fn restart(&mut self) {
        self.wave_length.restart();
        self.envelope.restart();
        self.frequency.restart();
    }

    pub fn cycle(&mut self) {
        if !self.enable { return; }
        if self.frequency.cycle() {
            self.wave_length.next();
        }
    }

    pub fn clock(&mut self, step: u8) {
        if !self.enable { false; }
        match step {
            0 => if self.wave_length.clock_length() {
                self.enable = false;
                self.nr24 = 0;
            },
            2 => if self.wave_length.clock_length() {
                self.enable = false;
                self.nr24 = 0;
            }// clock length and sweep// clock length and sweep
            4 => if self.wave_length.clock_length() {
                self.enable = false;
                self.nr24 = 0;
            },
            6 => if self.wave_length.clock_length() {
                self.enable = false;
                self.nr24 = 0;
            }// clock length and sweep
            7 => self.envelope.clock(),
            _ => { }
        }
    }

    pub fn get_data(&self) -> f32 {
        if self.enable {
            let amp = self.wave_length.get_amp() as f32;
            let vol = self.envelope.current_volume() as f32;
            amp * vol
        } else {
            0.0
        }
    }

    pub fn set(&mut self, address: usize, val: u8) {
        match address {
            0xFF16 => {
                self.nr21 = val;
                self.wave_length.set(val);
            }
            0xFF17 => {
                self.nr22 = val;
                self.envelope.set(val);
            }
            0xFF18 => {
                self.nr23 = val;
                self.frequency.set_nrx3(val);
            }
            0xFF19 => {
                self.nr24 = val;
                if (val >> 7) & 1 == 1 {
                    self.enable = true;
                    self.frequency.set_nrx4(val);
                    if (val >> 6) & 1 == 1 { self.wave_length.enable_length() }
                    self.restart();
                }
            }
            _ => panic!("SET APU CHANNEL 2: {:#06x}->{:#04x}", address, val)
        }
    }

    pub fn get(&self, address: usize) -> u8 {
        match address {
            0xFF16 => self.nr21,
            0xFF17 => self.nr22,
            0xFF18 => self.nr23,
            0xFF19 => self.nr24,
            _ => panic!("GET APU CHANNEL 2: {:#06x}", address)
        }
    }
}



