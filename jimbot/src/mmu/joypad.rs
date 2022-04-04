pub enum Key {
    Start,
    Select,
    B,
    A,
    Down,
    Up,
    Left,
    Right,
}

pub enum Mode {
    Direction,
    Action,
    None,
}

pub struct JoyPad {
    mode: Mode,
    start: bool,
    select: bool,
    b: bool,
    a: bool,
    down: bool,
    up: bool,
    left: bool,
    right: bool,
}

impl Default for JoyPad {
    fn default() -> Self {
        Self {
            mode: Mode::None,
            start: false,
            select: false,
            b: false,
            a: false,
            down: false,
            up: false,
            left: false,
            right: false
        }
    }
}

impl JoyPad {
    pub fn write(&mut self, u8: u8) {
        self.mode = if (u8 >> 5) & 1 == 0 {
            Mode::Action
        } else if (u8 >> 4) & 0 == 0 {
            Mode::Direction
        } else {
            Mode::None
        };
    }

    pub fn press(&mut self, key: Key) {
        match key {
            Key::Start => { self.start = true }
            Key::Select => { self.select = true }
            Key::B => { self.b = true }
            Key::A => { self.a = true }
            Key::Down => { self.down = true }
            Key::Up => { self.up = true }
            Key::Left => { self.left = true }
            Key::Right => { self.right = true }
        }
    }

    pub fn release(&mut self, key: Key) {
        match key {
            Key::Start => { self.start = false }
            Key::Select => { self.select = false }
            Key::B => { self.b = false }
            Key::A => { self.a = false }
            Key::Down => { self.down = false }
            Key::Up => { self.up = false }
            Key::Left => { self.left = false }
            Key::Right => { self.right = false }
        }
    }

    pub fn bytes(&self) -> u8 {
        match self.mode {
            Mode::None => { return 0xFF },
            Mode::Direction => {
                let mut byte = 0b1101_1111;

                if self.down { byte &= 0b1111_0111 }
                if self.up { byte &= 0b1111_1011 }
                if self.left { byte &= 0b1111_1101 }
                if self.right { byte &= 0b1111_1110 }
                byte
            },
            Mode::Action => {
                let mut byte = 0b1110_1111;

                if self.start { byte &= 0b1111_0111 }
                if self.select { byte &= 0b1111_1011 }
                if self.b { byte &= 0b1111_1101 }
                if self.a { byte &= 0b1111_1110 }
                byte
            }
        }
    }


}