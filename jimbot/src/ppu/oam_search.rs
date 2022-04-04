use crate::mmu::MMU;
use crate::mmu::sprite::Sprite;

pub struct OAMSearch {
    current_entry: usize,
    cycle_available: u8,
}

impl Default for OAMSearch {
    fn default() -> Self {
        Self {
            current_entry: 0,
            cycle_available: 0,
        }
    }
}

impl OAMSearch {
    pub fn cycle(&mut self, mmu: &MMU, sprite_buffer: &mut Vec<Sprite>) -> bool {
        self.cycle_available += 1;

        if self.cycle_available >= 2 {
            self.cycle_available -= 2;
            let ly = mmu.ly();
            let lcdc = mmu.lcdc();
            let sprite_height = lcdc.sprite_height();
            let oam_index = self.current_entry * 4;
            let sprite: Sprite = [
                mmu.oam()[oam_index + 0],
                mmu.oam()[oam_index + 1],
                mmu.oam()[oam_index + 2],
                mmu.oam()[oam_index + 3],
            ].into();
            if sprite.x() > 0 &&
                ly + 16 >= sprite.y() &&
                ly + 16 < sprite.y() + sprite_height &&
                sprite_buffer.len() < 10 {
                sprite_buffer.push(sprite)
            }
            self.current_entry += 1;
        }

        if self.current_entry == 40 {
            self.reset();
            assert_eq!(self.cycle_available, 0, "Cycle should be 0 but {}", self.cycle_available);
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.cycle_available = 0;
        self.current_entry = 0;
    }
}