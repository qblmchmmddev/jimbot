#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PixelType {
    Background(u8),
    Window(u8),
    Sprite {
        pixels: u8,
        palette_1: bool,
    },
}

impl PixelType {
    pub fn from_bg_tile_data(lo: u8, hi: u8) -> [Self; 8] {
        [
            PixelType::Background(((lo >> 7) & 1) | (((hi >> 7) & 1) << 1)),
            PixelType::Background(((lo >> 6) & 1) | (((hi >> 6) & 1) << 1)),
            PixelType::Background(((lo >> 5) & 1) | (((hi >> 5) & 1) << 1)),
            PixelType::Background(((lo >> 4) & 1) | (((hi >> 4) & 1) << 1)),
            PixelType::Background(((lo >> 3) & 1) | (((hi >> 3) & 1) << 1)),
            PixelType::Background(((lo >> 2) & 1) | (((hi >> 2) & 1) << 1)),
            PixelType::Background(((lo >> 1) & 1) | (((hi >> 1) & 1) << 1)),
            PixelType::Background(((lo >> 0) & 1) | (((hi >> 0) & 1) << 1)),
        ]
    }

    pub fn from_window_tile_data(lo: u8, hi: u8) -> [Self; 8] {
        [
            PixelType::Window(((lo >> 7) & 1) | (((hi >> 7) & 1) << 1)),
            PixelType::Window(((lo >> 6) & 1) | (((hi >> 6) & 1) << 1)),
            PixelType::Window(((lo >> 5) & 1) | (((hi >> 5) & 1) << 1)),
            PixelType::Window(((lo >> 4) & 1) | (((hi >> 4) & 1) << 1)),
            PixelType::Window(((lo >> 3) & 1) | (((hi >> 3) & 1) << 1)),
            PixelType::Window(((lo >> 2) & 1) | (((hi >> 2) & 1) << 1)),
            PixelType::Window(((lo >> 1) & 1) | (((hi >> 1) & 1) << 1)),
            PixelType::Window(((lo >> 0) & 1) | (((hi >> 0) & 1) << 1)),
        ]
    }

    pub fn from_sprite_tile_data(lo: u8, hi: u8, palette_1: bool) -> [Self; 8] {
        [
            PixelType::Sprite { pixels: ((lo >> 7) & 1) | (((hi >> 7) & 1) << 1), palette_1},
            PixelType::Sprite { pixels: ((lo >> 6) & 1) | (((hi >> 6) & 1) << 1), palette_1},
            PixelType::Sprite { pixels: ((lo >> 5) & 1) | (((hi >> 5) & 1) << 1), palette_1},
            PixelType::Sprite { pixels: ((lo >> 4) & 1) | (((hi >> 4) & 1) << 1), palette_1},
            PixelType::Sprite { pixels: ((lo >> 3) & 1) | (((hi >> 3) & 1) << 1), palette_1},
            PixelType::Sprite { pixels: ((lo >> 2) & 1) | (((hi >> 2) & 1) << 1), palette_1},
            PixelType::Sprite { pixels: ((lo >> 1) & 1) | (((hi >> 1) & 1) << 1), palette_1},
            PixelType::Sprite { pixels: ((lo >> 0) & 1) | (((hi >> 0) & 1) << 1), palette_1},
        ]
    }
}