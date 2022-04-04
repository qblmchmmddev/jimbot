#[derive(Copy, Clone)]
pub enum TileMapArea {
    U9800,
    U9C00,
}

impl TileMapArea {
    pub fn address(&self, offset: u16) -> u16 {
        match self {
            TileMapArea::U9800 => 0x9800 + offset,
            TileMapArea::U9C00 => 0x9C00 + offset,
        }
    }
}

#[derive(Copy, Clone)]
pub enum TileDataArea {
    I8800,
    U8000,
}


impl TileDataArea {
    pub fn address(&self, offset: u16) -> u16 {
        match self {
            TileDataArea::I8800 => {
                let base = 0x9000i32;
                (base + (((offset as i8) as i32) * 16)) as u16
            }
            TileDataArea::U8000 => {
                let base = 0x8000u16;
                base + offset * 16
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct LCDC(u8);

impl From<u8> for LCDC { fn from(from: u8) -> Self { LCDC(from) } }

impl From<LCDC> for u8 { fn from(from: LCDC) -> Self { from.0 } }

impl LCDC {
    pub fn sprite_height(&self) -> u8 {
        if (self.0 >> 2) & 1 == 0 { 8 } else { 16 }
    }

    pub fn bg_tilemap_area(&self) -> TileMapArea {
        if (self.0 >> 3) & 1 == 0 {
            TileMapArea::U9800
        } else {
            TileMapArea::U9C00
        }
    }

    pub fn window_tilemap_area(&self) -> TileMapArea {
        if (self.0 >> 6) & 1 == 0 {
            TileMapArea::U9800
        } else {
            TileMapArea::U9C00
        }
    }

    pub fn bg_window_tiledata_area(&self) -> TileDataArea {
        if (self.0 >> 4) & 1 == 0 {
            TileDataArea::I8800
        } else {
            TileDataArea::U8000
        }
    }

    pub fn is_sprite_enable(&self) -> bool {
        (self.0 >> 1) & 1 == 1
    }

    pub fn is_display_enable(&self) -> bool {
        (self.0 >> 7) & 1 == 1
    }

    pub fn is_window_enable(&self) -> bool {
        self.is_bg_window_enable() && (self.0 >> 5) & 1 == 1
    }

    pub fn is_bg_window_enable(&self) -> bool {
        (self.0 >> 0) & 1 == 1
    }
}