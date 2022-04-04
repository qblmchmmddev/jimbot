use crate::apu::envelope::Envelope;
use crate::apu::frequency::Frequency;
use crate::apu::frequency_sweep::FrequencySweep;
use crate::apu::wave_length::WaveLength;
use crate::cpu::registers::R8::P;

pub struct Channel1 {
    enable: bool,
    nr10: u8,
    nr11: u8,
    nr12: u8,
    nr13: u8,
    nr14: u8,
    frequency_sweep: FrequencySweep,
    frequency: Frequency,
    wave_length: WaveLength,
    envelope: Envelope,
}

impl Default for Channel1 {
    fn default() -> Self {
        Self {
            enable: false,
            nr10: 0,
            nr11: 0,
            nr12: 0,
            nr13: 0,
            nr14: 0,
            frequency_sweep: Default::default(),
            frequency: Frequency::default(),
            wave_length: 0.into(),
            envelope: 0.into(),
        }
    }
}

impl Channel1 {
    pub fn restart(&mut self) {
        self.wave_length.restart();
        self.envelope.restart();
        self.frequency.restart();
        self.frequency_sweep.restart(self.frequency.initial_frequency());
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
                self.nr14 = 0;
            },
            2 => {
                if let Some(new_freq) = self.frequency_sweep.clock() {
                    self.frequency.set_new_frequency(new_freq);
                }
                if self.wave_length.clock_length() {
                    self.enable = false;
                    self.nr14 = 0;
                }
            }// clock length and sweep
            4 => if self.wave_length.clock_length() {
                self.enable = false;
                self.nr14 = 0;
            },
            6 => {
                if let Some(new_freq) = self.frequency_sweep.clock() {
                    self.frequency.set_new_frequency(new_freq);
                }
                if self.wave_length.clock_length() {
                    self.enable = false;
                    self.nr14 = 0;
                }
            }// clock length and sweep
            7 => self.envelope.clock(),
            _ => {}//panic!("Unknown step: {}", step)
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
            0xFF10 => {
                self.nr10 = val;
                self.frequency_sweep.set(val);
            }
            0xFF11 => {
                self.nr11 = val;
                self.wave_length.set(val);
            }
            0xFF12 => {
                self.nr12 = val;
                self.envelope.set(val);
            }
            0xFF13 => {
                self.nr13 = val;
                self.frequency.set_nrx3(val);
            }
            0xFF14 => {
                self.nr14 = val;
                if (val >> 7) & 1 == 1 {
                    self.enable = true;
                    self.frequency.set_nrx4(val);
                    if (val >> 6) & 1 == 1 { self.wave_length.enable_length() }
                    self.restart();
                }
            }
            _ => panic!("SET APU CHANNEL 1: {:#06x}->{:#04x}", address, val)
        }
    }

    pub fn get(&self, address: usize) -> u8 {
        match address {
            0xFF10 => self.nr10,
            0xFF11 => self.nr11,
            0xFF12 => self.nr12,
            0xFF13 => self.nr13,
            0xFF14 => self.nr14,
            _ => panic!("GET APU CHANNEL 1: {:#06x}", address)
        }
    }
}



