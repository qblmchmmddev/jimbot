use std::process::id;
use crate::mmu::MMU;
use crate::mmu::sprite::Sprite;
use crate::ppu::pixel_fifo::PixelFifo;
use crate::ppu::pixel_type::PixelType;

#[derive(Debug)]
pub enum Step {
    Idle,
    FetchTileDataLow { sprite: Sprite },
    FetchTileDataHi { sprite: Sprite, tile_data_row_address_low: u16, tile_data_row_low: u8 },
    PushToFifo { sprite: Sprite, tile_data_row_low: u8, tile_data_row_hi: u8 },
}

pub struct SpritePixelFetcher {
    cycle_available: u8,
    current_step: Step,
}

impl Default for SpritePixelFetcher {
    fn default() -> Self {
        Self {
            cycle_available: 0,
            current_step: Step::Idle,
        }
    }
}

impl SpritePixelFetcher {
    pub fn fetch(&mut self, sprite: Sprite, mmu: &MMU, pixel_fifo: &mut PixelFifo) {
        self.current_step = Step::FetchTileDataLow { sprite };
        self.step(mmu, pixel_fifo);
    }

    pub fn need_step(&self) -> bool {
        match self.current_step {
            Step::Idle => false,
            _ => true,
        }
    }

    pub fn step(&mut self, mmu: &MMU, pixel_fifo: &mut PixelFifo) {
        self.cycle_available += 1;
        match self.current_step {
            Step::Idle => panic!("Is idling no need to step"),
            Step::FetchTileDataLow { sprite } => self.fetch_tile_data_low(sprite, mmu),
            Step::FetchTileDataHi { sprite, tile_data_row_address_low, tile_data_row_low } => self.fetch_tile_data_hi(sprite, tile_data_row_address_low, tile_data_row_low, mmu),
            Step::PushToFifo { sprite, tile_data_row_low, tile_data_row_hi } => self.push_to_fifo(sprite, tile_data_row_low, tile_data_row_hi, pixel_fifo, mmu),
        }
    }

    fn fetch_tile_data_low(&mut self, sprite: Sprite, mmu: &MMU) {
        if self.cycle_available < 2 { return; }
        self.cycle_available -= 2;
        let ly = mmu.ly();
        let sprite_height = mmu.lcdc().sprite_height();
        let tile_row_offset = 2 * if sprite.flags().is_y_flipped() { (sprite_height - 1) - ((ly - sprite.y()) % sprite_height) } else { (ly.wrapping_sub(sprite.y())) % sprite_height };
        let tile_data_address = if mmu.lcdc().sprite_height() == 8 {
            0x8000 + sprite.tiledata_index() as u16 * 16 // 16 bytes per tile
        } else {
            0x8000 + (sprite.tiledata_index() & !1) as u16 * 16
        };
        let tile_data_row_address_low = tile_data_address + tile_row_offset as u16;
        let mut tile_data_row_low = mmu.get(tile_data_row_address_low);
        if sprite.x() < 8 {
            if sprite.flags().is_x_flipped() {
                tile_data_row_low = tile_data_row_low >> (8 - sprite.x());
            } else {
                tile_data_row_low = tile_data_row_low << (8 - sprite.x());
            }
        }
        self.current_step = Step::FetchTileDataHi { sprite, tile_data_row_address_low, tile_data_row_low }
    }

    fn fetch_tile_data_hi(&mut self, sprite: Sprite, tile_data_row_address_low: u16, tile_data_row_low: u8, mmu: &MMU) {
        if self.cycle_available < 2 { return; }
        self.cycle_available -= 2;
        let mut tile_data_row_hi = mmu.get(tile_data_row_address_low + 1);
        if sprite.x() < 8 {
            if sprite.flags().is_x_flipped() {
                tile_data_row_hi = tile_data_row_hi >> (8 - sprite.x());
            } else {
                tile_data_row_hi = tile_data_row_hi << (8 - sprite.x());
            }
        }
        self.current_step = Step::PushToFifo {
            sprite,
            tile_data_row_low,
            tile_data_row_hi,
        }
    }

    fn push_to_fifo(&mut self, sprite: Sprite, tile_data_row_low: u8, tile_data_row_hi: u8, pixel_fifo: &mut PixelFifo, mmu: &MMU) {
        if self.cycle_available < 2 { return; }
        self.cycle_available -= 2;
        let mut sprite_tile_pixels = if mmu.lcdc().is_sprite_enable() {
            PixelType::from_sprite_tile_data(tile_data_row_low, tile_data_row_hi, sprite.flags().palette_1())
        } else {
            [PixelType::Sprite { pixels: 0, palette_1: sprite.flags().palette_1() }; 8]
        };
        if sprite.flags().is_x_flipped() { sprite_tile_pixels.reverse() };
        let mut fifo_front_8 = pixel_fifo.pop_front_8();
        for i in 0..8 {
            let fifo_px = fifo_front_8[i];
            let sprite_px = sprite_tile_pixels[i];
            match (fifo_px, sprite_px) {
                (PixelType::Background(bg_pixels), PixelType::Sprite { pixels: sprite_pixels, palette_1: _ }) => {
                    if sprite.flags().bg_prior() {
                        if bg_pixels == 0 && sprite_pixels > 0 { fifo_front_8[i] = sprite_px; }
                    } else {
                        if sprite_pixels > 0 { fifo_front_8[i] = sprite_px; }
                    }
                }
                (PixelType::Window(window_pixels), PixelType::Sprite { pixels: sprite_pixels, palette_1: _ }) => {
                    if sprite.flags().bg_prior() {
                        if window_pixels == 0 && sprite_pixels > 0 { fifo_front_8[i] = sprite_px; }
                    } else {
                        if sprite_pixels > 0 { fifo_front_8[i] = sprite_px; }
                    }
                    // if !sprite.flags().bg_prior() {
                    //     if window_pixels == 0 && sprite_pixels > 0 { fifo_front_8[i] = sprite_px }
                    // }
                    // if sprite.flags().bg_prior()  {
                    //     if bg_pixels == 0 && sprite_pixels > 0 { fifo_front_8[i] = sprite_px; }
                    // } else {
                    //     if sprite_pixels > 0 { fifo_front_8[i] = sprite_px; }
                    // }
                }
                _ => {}
            }
        }
        pixel_fifo.push_tile_pixel_row_front(fifo_front_8);
        assert_eq!(self.cycle_available, 0, "Cycle available should 0 but {}", self.cycle_available);
        self.current_step = Step::Idle;
    }

    pub fn reset(&mut self, is_vblank: bool) {
        assert_eq!(self.cycle_available, 0, "Cycle should be 0 but {}", self.cycle_available);
        // assert_eq!(self.current_step, Step::Idle, "Step should be idle but {:?}", self.current_step);
    }
}