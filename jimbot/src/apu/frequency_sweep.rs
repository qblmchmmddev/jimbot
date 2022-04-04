pub struct FrequencySweep {
    nr10: u8,
    shadow_frequency: u16,
    timer: u8,
    enabled: bool,
}

impl Default for FrequencySweep {
    fn default() -> Self {
        Self {
            nr10: 0,
            shadow_frequency: 0,
            timer: 0,
            enabled: false
        }
    }
}

impl FrequencySweep {

    pub fn set(&mut self, nr10: u8) {
        self.nr10 = nr10;
    }

    pub fn restart(&mut self, current_frequency: u16) {
        self.shadow_frequency = current_frequency;
        if self.sweep_time() > 0 {
            self.timer = self.sweep_time();
        } else {
            self.timer = 8;
        }
        self.enabled = self.sweep_time() != 0 || self.shift() != 0;
        if self.shift() != 0 {
            self.calculate_new_frequency();
        }
    }

    pub fn clock(&mut self) -> Option<u16> {
        if self.timer > 0 {
            self.timer -= 1;
            return None;
        }
        if self.sweep_time() > 0 {
            self.timer = self.sweep_time();
        } else {
            self.timer = 8;
        }

        if self.enabled && self.sweep_time() > 0 {
            let new_frequency = self.calculate_new_frequency();

            if new_frequency < 2048 && self.shift() > 0 {
                self.shadow_frequency = new_frequency;

                /* for overflow check */
                self.calculate_new_frequency();

                return Some(new_frequency);
            }
        }
        None
    }

    fn calculate_new_frequency(&mut self) -> u16 {
        let mut new_frequency =  self.shadow_frequency >> (self.shift() as u16);
        if self.is_decrease() {
            new_frequency = self.shadow_frequency - new_frequency;
        } else {
            new_frequency = self.shadow_frequency + new_frequency;
        }

        if new_frequency > 2047 {
            self.enabled = false;
        }
        new_frequency
    }

    fn sweep_time(&self) -> u8 {
        (self.nr10 >> 4) & 0b111
    }

    fn is_decrease(&self) -> bool {
        (self.nr10 >> 3) & 1 == 0
    }

    fn shift(&self) -> u8 {
        self.nr10 & 0b111
    }
}