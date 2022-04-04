#[derive(Debug, Copy, Clone)]
pub enum InterruptRequest {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad,
}

impl InterruptRequest {
    pub fn routine_location(&self) -> u16 {
        match self {
            InterruptRequest::VBlank => 0x40,
            InterruptRequest::LCDStat => 0x48,
            InterruptRequest::Timer => 0x50,
            InterruptRequest::Serial => 0x58,
            InterruptRequest::Joypad => 0x60,
        }
    }
}

pub struct Interrupts(u8);

impl From<u8> for Interrupts { fn from(from: u8) -> Self { Interrupts(from) } }

impl From<Interrupts> for u8 { fn from(from: Interrupts) -> Self { from.0 } }

impl Interrupts {
    pub fn get_request_by_priority(&self) -> Vec<InterruptRequest> {
        let mut res = vec![];
        let bits = self.0;
        if bits & 1 == 1 { res.push(InterruptRequest::VBlank) }
        if (bits >> 1) & 1 == 1 { res.push(InterruptRequest::LCDStat) }
        if (bits >> 2) & 1 == 1 { res.push(InterruptRequest::Timer) }
        if (bits >> 3) & 1 == 1 { res.push(InterruptRequest::Serial) }
        if (bits >> 4) & 1 == 1 { res.push(InterruptRequest::Joypad) }
        res
    }

    pub fn enable_request(&mut self, request: InterruptRequest) {
        match request {
            InterruptRequest::VBlank => self.0 |= 0b1,
            InterruptRequest::LCDStat => self.0 |= 0b10,
            InterruptRequest::Timer => self.0 |= 0b100,
            InterruptRequest::Serial => self.0 |= 0b1000,
            InterruptRequest::Joypad => self.0 |= 0b1_0000,
        }
    }

    pub fn is_enable(&self, request: InterruptRequest) -> bool {
        let bits = self.0;
        match request {
            InterruptRequest::VBlank => (bits >> 0) & 1 == 1,
            InterruptRequest::LCDStat => (bits >> 1) & 1 == 1,
            InterruptRequest::Timer => (bits >> 2) & 1 == 1,
            InterruptRequest::Serial => (bits >> 3) & 1 == 1,
            InterruptRequest::Joypad => (bits >> 4) & 1 == 1,
        }
    }

    pub fn disable_request(&mut self, request: InterruptRequest) {
        match request {
            InterruptRequest::VBlank => self.0 &= 0b1111_1110,
            InterruptRequest::LCDStat => self.0 &= 0b1111_1101,
            InterruptRequest::Timer => self.0 &= 0b1111_1011,
            InterruptRequest::Serial => self.0 &= 0b1111_0111,
            InterruptRequest::Joypad => self.0 &= 0b1110_1111,
        }
    }
}