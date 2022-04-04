#[derive(Debug, Copy, Clone)]
pub struct Flag(u8);

impl From<u8> for Flag { fn from(u8: u8) -> Self { Self(u8) } }

impl Into<u8> for Flag { fn into(self) -> u8 { self.0 } }

impl Flag {
    pub fn is_x_flipped(&self) -> bool { (self.0 >> 5) & 1 == 1 }
    pub fn is_y_flipped(&self) -> bool { (self.0 >> 6) & 1 == 1 }
    pub fn bg_prior(&self) -> bool { (self.0 >> 7) & 1 == 1 }
    pub fn palette_1(&self) -> bool { (self.0 >> 4) & 1 == 1 }
}

#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    y: u8,
    x: u8,
    tiledata_index: u8,
    flags: Flag,
}

impl From<[u8; 4]> for Sprite {
    fn from(from: [u8; 4]) -> Self {
        Self {
            y: from[0],
            x: from[1],
            tiledata_index: from[2],
            flags: from[3].into()
        }
    }
}

impl Sprite {
    pub fn new(y: u8, x: u8, tiledata_index: u8, flags: u8) -> Self {
        Self {
            y,
            x,
            tiledata_index,
            flags: flags.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        let flags: u8 = self.flags.into();
        flags + self.y + self.x + self.tiledata_index == 0
    }

    pub fn y(&self) -> u8 { self.y }
    pub fn x(&self) -> u8 { self.x }
    pub fn tiledata_index(&self) -> u8 { self.tiledata_index }
    pub fn flags(&self) -> &Flag { &self.flags }
}

impl Default for Sprite {
    fn default() -> Self {
        Self::new(0,0,0,0)
    }
}