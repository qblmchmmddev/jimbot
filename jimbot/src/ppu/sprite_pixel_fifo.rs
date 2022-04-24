use std::collections::VecDeque;
use crate::mmu::sprite::Flag;

pub struct SpritePixelFifo {
    pixels: VecDeque<(u8, Flag)>,
}

impl Default for SpritePixelFifo {
    fn default() -> Self {
        Self {
            pixels: VecDeque::with_capacity(8)
        }
    }
}

impl SpritePixelFifo {
    pub(crate) fn can_pop(&self) -> bool { self.pixels.len() >= 8 }
    pub(crate) fn can_push(&self) -> bool { self.pixels.len() <= 8 }
    pub(crate) fn pop(&mut self) -> Option<(u8, Flag)> {
        self.pixels.pop_front()
    }
    pub(crate) fn push_tile_pixel_row(&mut self, tile_pixel_row: [(u8, Flag); 8]) {
        for i in 0..8 {
            if i < self.pixels.len() {
                if self.pixels[i].0 == 0 {
                    self.pixels[i] = tile_pixel_row[i];
                }
                continue
            }
            self.pixels.push_back(tile_pixel_row[i]);
        }
    }
    pub fn reset(&mut self) {
        self.pixels.clear();
    }
}