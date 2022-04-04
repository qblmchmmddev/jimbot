use std::collections::VecDeque;
use crate::ppu::pixel_type::PixelType;

pub struct PixelFifo {
    pixels: VecDeque<PixelType>,
}

impl Default for PixelFifo {
    fn default() -> Self {
        Self {
            pixels: VecDeque::with_capacity(16)
        }
    }
}

impl PixelFifo {
    // pub(crate) fn is_fill_8(&self) -> bool {self.pixels.len() >= 8 }
    pub(crate) fn can_pop(&self) -> bool { self.pixels.len() >= 8 }
    pub(crate) fn can_push(&self) -> bool { self.pixels.len() <= 8 }
    pub(crate) fn pop(&mut self) -> PixelType {
        assert!(self.can_pop(), "Should not pop");
        self.pixels.pop_front().unwrap()
    }
    pub(crate) fn push_tile_pixel_row(&mut self, tile_pixel_row: [PixelType; 8]) {
        for pixel in tile_pixel_row {
            self.pixels.push_back(pixel);
        }
        assert!(self.pixels.len() <= 16, "[push back] Should not more than 16: {}", self.pixels.len());

    }
    pub(crate) fn push_tile_pixel_row_front(&mut self, tile_pixel_row: [PixelType; 8]) {
        assert!(self.can_push(), "Should not push current length: {}", self.pixels.len());
        for i in (0..8).rev() {
            self.pixels.push_front(tile_pixel_row[i]);
        }
        assert!(self.pixels.len() <= 16, "[push front] Should not more than 16: {}", self.pixels.len());

    }
    pub fn reset(&mut self) {
        // assert_eq!(self.cycle_available, 0, "Cycle should be 0 but {}", self.cycle_available);
        self.pixels.clear();
    }
    pub fn pop_front_8(&mut self) -> [PixelType; 8] {
        let mut res = [PixelType::Background(0); 8];
        for i in 0..8 {
            res[i] = self.pixels.pop_front().unwrap();
        }
        res
    }
}