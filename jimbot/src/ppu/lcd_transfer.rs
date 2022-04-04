use crate::mmu::MMU;
use crate::mmu::sprite::Sprite;
use crate::ppu::pixel_fetcher::PixelFetcher;
use crate::ppu::pixel_fifo::PixelFifo;
use crate::ppu::pixel_type::PixelType;

pub struct LCDTransfer {
    is_initial_scanline: bool,
    pixel_fifo: PixelFifo,
    pixel_fetcher: PixelFetcher,
    x: usize,
    pixel_to_discard: u8,
    window_line: bool,
}

impl Default for LCDTransfer {
    fn default() -> Self {
        Self {
            is_initial_scanline: true,
            pixel_fifo: PixelFifo::default(),
            pixel_fetcher: PixelFetcher::default(),
            x: 0,
            window_line: false,
            pixel_to_discard: 0,
        }
    }
}

impl LCDTransfer {
    pub fn cycle(&mut self, mmu: &MMU, sprite_buffer: &mut Vec<Sprite>, lcd: &mut [[u8; 144]; 160]) -> bool {
        if self.is_initial_scanline {
            let wy = mmu.wy();
            let ly = mmu.ly();
            if ly == wy { self.window_line = true; }
            self.pixel_to_discard = mmu.scx() % 8;
            self.is_initial_scanline = false;
            // println!("LY:{}, SCX:{}", mmu.ly(), mmu.scx());
        }

        if !self.pixel_fetcher.fetching_sprite() {
            if self.pixel_to_discard == 0 && !self.pixel_fetcher.is_window_mode() && mmu.lcdc().is_window_enable() && self.window_line && self.x >= (mmu.wx() - 7) as usize {
                self.pixel_fifo.reset();
                self.pixel_fetcher.fetch_window(mmu, &mut self.pixel_fifo);
                // sprite_buffer.clear();
            } else {
                if self.pixel_fifo.can_pop() {
                    if self.pixel_to_discard > 0 {
                        self.pixel_fifo.pop();
                        self.pixel_to_discard -= 1;
                        self.pixel_fetcher.step(mmu, &mut self.pixel_fifo);
                    } else {
                        if let Some(sprite) = self.get_sprite(sprite_buffer) {
                            self.pixel_fetcher.fetch_sprite(sprite, mmu, &mut self.pixel_fifo);
                        } else {
                            let pixel = match self.pixel_fifo.pop() {
                                PixelType::Background(u8) => mmu.bgp().get_color(u8),
                                PixelType::Window(u8) => mmu.bgp().get_color(u8),
                                PixelType::Sprite { pixels, palette_1 } => if palette_1 {
                                    mmu.obp1().get_color(pixels)
                                } else {
                                    mmu.obp0().get_color(pixels)
                                },
                            };
                            lcd[self.x as usize][mmu.ly() as usize] = pixel;
                            self.x += 1;
                            self.pixel_fetcher.step(mmu, &mut self.pixel_fifo);
                        }
                    }
                } else {
                    self.pixel_fetcher.step(mmu, &mut self.pixel_fifo)
                }
            }
        } else {
            self.pixel_fetcher.step(mmu, &mut self.pixel_fifo)
        }

        if self.x == 160 {
            self.reset(mmu.ly() == 143);
            true
        } else {
            false
        }
    }

    fn get_sprite(&self, sprite_buffer: &mut Vec<Sprite>) -> Option<Sprite> {
        if sprite_buffer.is_empty() { return None; };

        for i in 0..sprite_buffer.len() {
            let s = sprite_buffer[i];
            if s.x() <= self.x as u8 + 8 {
                return Some(sprite_buffer.remove(i));
            }
        }
        None
    }

    pub fn reset(&mut self, all: bool) {
        self.is_initial_scanline = true;
        self.x = 0;
        self.pixel_fifo.reset();
        if all { self.window_line = false; }
        self.pixel_fetcher.reset(all);
    }
}