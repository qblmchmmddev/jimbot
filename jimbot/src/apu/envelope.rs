pub struct Envelope {
    nrx2: u8,
    period_timer: u8,
    volume: u8,
}

impl From<u8> for Envelope { fn from(from: u8) -> Self { Envelope::new(from) } }

impl From<Envelope> for u8 { fn from(from: Envelope) -> Self { from.nrx2 } }

impl Envelope {
    fn new(nrx2: u8) -> Self {
        Self {
            nrx2,
            period_timer: 0,
            volume: 0,
        }
    }

    pub fn restart(&mut self) {
        self.volume = self.initial_volume();
        self.period_timer = self.sweep();
    }

    pub fn set(&mut self, nrx2: u8) {
        self.nrx2 = nrx2;
    }

    pub fn clock(&mut self) {
        if self.sweep() == 0 { return; }
        if self.period_timer > 0 {
            self.period_timer -= 1;
            if self.period_timer == 0 {
                self.period_timer = self.sweep();
            } else {
                return;
            }
        }

        if self.volume < 0xF && !self.is_decrease() {
            self.volume += 1
        } else if self.volume > 0 && self.is_decrease() {
            self.volume -= 1
        }
    }

    pub fn current_volume(&self) -> u8 {
        self.volume
    }

    pub fn initial_volume(&self) -> u8 {
        (self.nrx2 >> 4) & 0b1111
    }

    pub fn is_decrease(&self) -> bool {
        (self.nrx2 >> 3) & 1 == 0
    }

    pub fn sweep(&self) -> u8 {
        self.nrx2 & 0b111
    }
}