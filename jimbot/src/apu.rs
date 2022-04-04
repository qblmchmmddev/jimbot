use std::mem;
use crate::apu::channel1::Channel1;
use crate::apu::channel2::Channel2;
use crate::apu::channel3::Channel3;
use crate::apu::channel4::Channel4;

mod channel1;
mod channel2;
mod channel3;
mod frame_sequencer;
mod envelope;
mod wave_length;
mod frequency;
mod frequency_sweep;
mod length;
mod channel4;
mod noise_frequency;

pub struct APU {
    nr50: u8,
    nr51: u8,
    nr52: u8,
    channel1: Channel1,
    channel2: Channel2,
    channel3: Channel3,
    channel4: Channel4,
    amps: Vec<f32>,
    sample_counter: f32,
}

impl Default for APU {
    fn default() -> Self {
        Self {
            nr50: 0,
            nr51: 0,
            nr52: 0,
            channel1: Default::default(),
            channel2: Default::default(),
            channel3: Default::default(),
            channel4: Default::default(),
            amps: Vec::new(),
            sample_counter: 0.0,
        }
    }
}

impl APU {
    const SAMPLE_COUNTER_N_CYCLE: f32 = 69905.07 / 735.07 * 1.0;
    const PREDETERMINE_SQUARE_WAVES: [[u8; 8]; 4] = [
        [0, 0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 1, 1],
        [0, 0, 0, 0, 1, 1, 1, 1],
        [0, 1, 1, 1, 1, 1, 0, 0],
    ];

    pub fn cycle(&mut self) {
        if !self.is_sound_enable() { return; }
        self.channel1.cycle();
        self.channel2.cycle();
        self.channel3.cycle();
        self.channel4.cycle();

        self.sample_counter += 1.;
        if self.sample_counter >= Self::SAMPLE_COUNTER_N_CYCLE {
            let ch1 = self.channel1.get_data();
            let ch2 = self.channel2.get_data();
            let ch3 = self.channel3.get_data();
            let ch4 = self.channel4.get_data() / 2.;

            let sample = (ch1 + ch2 + ch3 + ch4) / 4.;
            self.amps.push(sample);

            self.sample_counter -= Self::SAMPLE_COUNTER_N_CYCLE;
        }
    }

    pub fn get_data(&mut self) -> Vec<f32> {
        // println!("{}", self.amps.len());
        mem::take(&mut self.amps)
    }

    fn is_sound_enable(&self) -> bool {
        (self.nr52 >> 7) & 1 == 1
    }

    pub fn clock(&mut self, step: u8) {
        self.channel1.clock(step);
        self.channel2.clock(step);
        self.channel3.clock(step);
        self.channel4.clock(step);
    }

    pub fn set(&mut self, address: usize, val: u8) {
        // println!("SET: {:#06x}->{:#04x}", address, val);
        match address {
            0xFF10..=0xFF14 => self.channel1.set(address, val),
            0xFF16..=0xFF19 => self.channel2.set(address, val),
            0xFF1A..=0xFF1E => self.channel3.set(address, val),
            0xFF30..=0xFF3f => self.channel3.set(address, val),
            0xFF20..=0xFF23 => self.channel4.set(address, val),
            0xFF24 => self.nr50 = val,
            0xFF25 => self.nr51 = val,
            0xFF26 => self.nr52 = val,
            _ => panic!("SET SOUND: {:#06x}->{:#04x}", address, val)
        }
    }

    pub fn get(&self, address: usize) -> u8 {
        let val = match address {
            0xFF10..=0xFF14 => self.channel1.get(address),
            0xFF16..=0xFF19 => self.channel2.get(address),
            0xFF1A..=0xFF1E => self.channel3.get(address),
            0xFF30..=0xFF3f => self.channel3.get(address),
            0xFF20..=0xFF23 => self.channel4.get(address),
            0xFF24 => self.nr50,
            0xFF25 => self.nr51,
            0xFF26 => self.nr52,
            _ => panic!("SET SOUND: {:#06x}", address)
        };
        // println!("GET: {:#06x}->{:#04x}", address, val);
        val
    }
    pub fn nr52(&self) -> u8 {
        self.nr52
    }
    pub fn nr51(&self) -> u8 {
        self.nr51
    }
    pub fn nr50(&self) -> u8 {
        self.nr50
    }
}