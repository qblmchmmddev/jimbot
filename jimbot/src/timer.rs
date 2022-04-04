use crate::apu::APU;
use crate::mmu::tac::TAC;

pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    apu_clock_step: u8,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            apu_clock_step: 0,
        }
    }
}

impl Timer {
    /// returns true if TIMA exceeds $FF (set timer interrupt flag)
    pub fn cycle(&mut self, apu: &mut APU) -> bool {
        let prev_div = self.div;
        self.div = self.div.wrapping_add(1);

        let tac: TAC = self.tac.into();
        let mut tima_overflow = false;
        if tac.is_timer_enable() {
            let tima_clock_select = tac.clock_select();
            let prev_div_tima_bit = (prev_div >> tima_clock_select) & 1;
            let curr_div_tima_bit = (self.div >> tima_clock_select) & 1;
            if prev_div_tima_bit == 1 && curr_div_tima_bit == 0 {
                let (tima, overflow) = self.tima.overflowing_add(1);
                self.tima = if overflow {
                    tima_overflow = true;
                    self.tma
                } else { tima };
            }
        }

        let prev_div_sound_clock_bit = (prev_div >> 13) & 1;
        let curr_div_sound_clock_bit = (self.div >> 13) & 1;
        if prev_div_sound_clock_bit == 1 && curr_div_sound_clock_bit == 0 {
            apu.clock(self.apu_clock_step);
            self.apu_clock_step = (self.apu_clock_step + 1) % 8;
        }
        tima_overflow
    }

    pub fn set(&mut self, address: usize, val: u8) {
        match address {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = val,
            0xFF06 => self.tma = val,
            0xFF07 => self.tac = val,
            _ => panic!("SET TIMER: {:#06x}->{:#04x}", address, val)
        }
    }

    pub fn get(&self, address: usize) -> u8 {
        match address {
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!("GET TIMER: {:#06x}", address)
        }
    }
}
