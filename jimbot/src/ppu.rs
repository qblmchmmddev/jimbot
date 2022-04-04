mod oam_search;
mod lcd_transfer;
mod pixel_fetcher;
mod pixel_fifo;
mod pixel_type;
mod sprite_pixel_fetcher;

use crate::mmu::interrupt_flag::InterruptRequest;
use crate::mmu::lcdstat::Mode;
use crate::mmu::MMU;
use crate::mmu::sprite::Sprite;
use crate::ppu::lcd_transfer::LCDTransfer;
use crate::ppu::oam_search::OAMSearch;

pub struct PPU {
    sprite_buffer: Vec<Sprite>,
    oam_search: OAMSearch,
    lcd_transfer: LCDTransfer,
    draw_hblank_cycle_used: u16,
    vblank_cycle_used: u16,
    lcd: [[[u8; 144]; 160]; 2],
    current_buffer: usize,
    stat_interrupt_line: bool,
}

impl Default for PPU {
    fn default() -> Self {
        Self {
            sprite_buffer: Vec::with_capacity(10),
            oam_search: OAMSearch::default(),
            lcd_transfer: LCDTransfer::default(),
            draw_hblank_cycle_used: 0,
            vblank_cycle_used: 0,
            lcd: [[[0; 144]; 160]; 2],
            current_buffer: 0,
            stat_interrupt_line: false,
        }
    }
}

impl PPU {
    const INITIAL_HBLANK_CYCLE_NEED: u16 = 80;

    pub fn cycle(&mut self, mmu: &mut MMU) {
        let lcdc = mmu.lcdc();
        if !lcdc.is_display_enable() { return; }
        let mut ly = mmu.ly();
        let mut stat = mmu.lcdstat();
        let mut new_stat_interrupt = false;
        match stat.mode() {
            Mode::OAMSearch => {
                if self.oam_search.cycle(mmu, &mut self.sprite_buffer) {
                    stat.set_mode(Mode::LCDTransfer);
                }
            }
            Mode::LCDTransfer => {
                self.draw_hblank_cycle_used += 1;
                if self.lcd_transfer.cycle(mmu, &mut self.sprite_buffer, &mut self.lcd[self.current_buffer]) {
                    stat.set_mode(Mode::HBlank);
                    new_stat_interrupt = new_stat_interrupt || stat.hblank_interrupt();
                }
            }
            Mode::HBlank => {
                self.draw_hblank_cycle_used += 1;
                if self.draw_hblank_cycle_used >= 376 {
                    self.draw_hblank_cycle_used = 0;
                    ly += 1;
                    stat.set_mode(if ly <= 143 {
                        new_stat_interrupt = new_stat_interrupt || stat.oam_interrupt();
                        self.sprite_buffer.clear();
                        Mode::OAMSearch
                    } else {
                        new_stat_interrupt = new_stat_interrupt || stat.vblank_interrupt();
                        mmu.request_interrupt(InterruptRequest::VBlank);
                        Mode::VBlank
                    });
                }
            }
            Mode::VBlank => {
                self.vblank_cycle_used += 1;
                if self.vblank_cycle_used >= 456 {
                    self.vblank_cycle_used = 0;
                    ly = (ly + 1) % 154;
                    if ly == 0 {
                        new_stat_interrupt = new_stat_interrupt || stat.oam_interrupt();
                        stat.set_mode(Mode::OAMSearch);
                        self.current_buffer = (self.current_buffer + 1) % 2;
                    }
                }
            }
        };
        let lyc = mmu.lyc();
        let ly_eq_lyc = ly == lyc;
        new_stat_interrupt = new_stat_interrupt || (ly_eq_lyc && stat.ly_eq_lyc());
        mmu.set_ly(ly);
        stat.set_coincidence(ly_eq_lyc);
        mmu.set_lcdstat(stat);

        if !self.stat_interrupt_line && new_stat_interrupt { mmu.request_interrupt(InterruptRequest::LCDStat) }

        self.stat_interrupt_line = new_stat_interrupt;
    }
    pub fn lcd(&self) -> [[u8; 144]; 160] {
        self.lcd[(self.current_buffer + 1) % 2]
    }
}
//
// pub struct PPU {
//     initial: bool,
//     enabled: bool,
//     sprite_buffer: Vec<Sprite>,
//     oam_cycle_used: u16,
//     oam_search: OAMSearch,
//     lcd_transfer: LCDTransfer,
//     draw_hblank_cycle_used: u16,
//     vblank_cycle_used: u16,
//     lcd: [[[u8; 144]; 160]; 2],
//     current_buffer: usize,
//     stat_interrupt_line: bool,
//     internal_ly: u8,
//     mode: Mode,
// }
//
// impl Default for PPU {
//     fn default() -> Self {
//         Self {
//             initial: true,
//             enabled: false,
//             sprite_buffer: Vec::with_capacity(10),
//             oam_cycle_used: 0,
//             oam_search: OAMSearch::default(),
//             lcd_transfer: LCDTransfer::default(),
//             draw_hblank_cycle_used: 0,
//             vblank_cycle_used: 0,
//             lcd: [[[0; 144]; 160]; 2],
//             current_buffer: 0,
//             stat_interrupt_line: false,
//             internal_ly: 0,
//             mode: Mode::OAMSearch,
//         }
//     }
// }
//
// impl PPU {
//     const INITIAL_HBLANK_CYCLE_NEED: u16 = 80;
//
//     pub fn cycle(&mut self, mmu: &mut MMU) {
//         let lcdc = mmu.lcdc();
//         if self.enabled && !lcdc.is_display_enable() {
//             self.enabled = false;
//             self.reset(mmu);
//             return;
//         }
//         let mut stat = mmu.lcdstat();
//         if !self.enabled && lcdc.is_display_enable() {
//             self.enabled = true;
//             self.initial = true;
//         }
//         if !self.enabled { return; }
//         let mut new_stat_interrupt = false;
//         let mut ly = mmu.ly();
//         let lyc = mmu.lyc();
//         let mut ly_eq_lyc = ly == lyc;
//         // if mmu.get(0xFFFF) == 0x3 { println!("PPU: MODE:{:?} oam_cycle:{} draw_hblank_cycle_used:{}" , self.mode, self.oam_cycle_used, self.draw_hblank_cycle_used); }
//         match self.mode {
//             Mode::OAMSearch => {
//                 // if self.oam_cycle_used == 4 {
//                 //     stat.set_mode(Mode::OAMSearch);
//                 //     // new_stat_interrupt = new_stat_interrupt || stat.oam_interrupt();
//                 //     new_stat_interrupt = new_stat_interrupt || (ly_eq_lyc && stat.ly_eq_lyc());
//                 // }
//                 // if self.internal_ly > 0 {
//                 //     if self.oam_cycle_used < 4 {
//                 //         ly_eq_lyc = false;
//                 //     } else if self.oam_cycle_used == 4 {
//                 //         stat.set_mode(Mode::OAMSearch);
//                 //         new_stat_interrupt = new_stat_interrupt || stat.oam_interrupt();
//                 //         new_stat_interrupt = new_stat_interrupt || (ly_eq_lyc && stat.ly_eq_lyc());
//                 //     }
//                 // }
//                 self.oam_cycle_used += 1;
//                 if self.oam_search.cycle(mmu, &mut self.sprite_buffer) {
//                     self.mode = Mode::LCDTransfer;
//                     stat.set_mode(Mode::LCDTransfer);
//                     self.oam_cycle_used = 0;
//                 }
//             }
//             Mode::LCDTransfer => {
//                 if self.lcd_transfer.cycle(mmu, &mut self.sprite_buffer, &mut self.lcd[self.current_buffer]) {
//                     self.mode = Mode::HBlank;
//                     stat.set_mode(Mode::HBlank);
//                     // new_stat_interrupt = new_stat_interrupt || stat.hblank_interrupt();
//                 }
//                 self.draw_hblank_cycle_used += 1;
//             }
//             Mode::HBlank => {
//                 self.draw_hblank_cycle_used += 1;
//                 if self.draw_hblank_cycle_used >= if self.initial { Self::INITIAL_HBLANK_CYCLE_NEED } else { 376 } {
//                     self.draw_hblank_cycle_used = 0;
//                     if self.initial {
//                         self.initial = false;
//                         ly = 0;
//                         self.internal_ly = 0;
//                         stat.set_mode(Mode::LCDTransfer);
//                         self.mode = Mode::LCDTransfer;
//                     } else {
//                         ly += 1;
//                         self.internal_ly += 1;
//                         self.mode = if self.internal_ly <= 143 {
//                             // println!("ly = lc:{}, oam:{}, vb:{}, h:{}",stat.ly_eq_lyc(), stat.oam_interrupt(), stat.vblank_interrupt(), stat.hblank_interrupt());
//                             self.sprite_buffer.clear();
//                             Mode::OAMSearch
//                         } else {
//                             stat.set_mode(Mode::VBlank);
//                             mmu.request_interrupt(InterruptRequest::VBlank);
//                             Mode::VBlank
//                         };
//                     }
//                 }
//             }
//             Mode::VBlank => {
//                 if self.internal_ly == 144 {
//                     if self.vblank_cycle_used < 4 {
//                         ly_eq_lyc = false;
//                     } else if self.vblank_cycle_used == 4 {
//                         new_stat_interrupt = new_stat_interrupt || (ly_eq_lyc && stat.ly_eq_lyc());
//                         // new_stat_interrupt = new_stat_interrupt || stat.vblank_interrupt();
//                         // new_stat_interrupt = new_stat_interrupt || stat.oam_interrupt();
//                     }
//                 } else if self.internal_ly < 153 {
//                     if self.vblank_cycle_used < 4 {
//                         ly_eq_lyc = false;
//                     } else if self.vblank_cycle_used == 4 {
//                         // new_stat_interrupt = new_stat_interrupt || (ly_eq_lyc && stat.ly_eq_lyc());
//                     }
//                 } else if self.internal_ly == 153 {
//                     match self.vblank_cycle_used {
//                         0..=3 | 8..=11 => ly_eq_lyc = false,
//                         _ => {}
//                     };
//                     if self.vblank_cycle_used == 4 {
//                         // new_stat_interrupt = new_stat_interrupt || (ly_eq_lyc && stat.ly_eq_lyc());
//                     } else if self.vblank_cycle_used == 12 {
//                         ly = 0;
//                         ly_eq_lyc = ly == lyc;
//                         // println!("WOW");
//                         // new_stat_interrupt = new_stat_interrupt || (ly_eq_lyc && stat.ly_eq_lyc());
//                     }
//                 }
//                 self.vblank_cycle_used += 1;
//                 // println!("VBU: {}", self.vblank_cycle_used);
//                 if self.vblank_cycle_used >= 456 {
//                     self.vblank_cycle_used = 0;
//                     if ly != 0 { ly += 1; }
//                     self.internal_ly = (self.internal_ly + 1) % 154;
//                     if self.internal_ly == 0 {
//                         ly = 0;
//                         self.current_buffer = (self.current_buffer + 1) % 2;
//                         self.mode = Mode::OAMSearch;
//                         stat.set_mode(Mode::OAMSearch);
//                     }
//                 }
//                 if self.internal_ly == 153 && self.vblank_cycle_used >= 4 {
//                     ly = 0;
//                 }
//             }
//         };
//         mmu.set_ly(ly);
//         stat.set_coincidence(ly_eq_lyc);
//         mmu.set_lcdstat(stat);
//
//         if !self.stat_interrupt_line && new_stat_interrupt { mmu.request_interrupt(InterruptRequest::LCDStat) }
//
//         self.stat_interrupt_line = new_stat_interrupt;
//     }
//     pub fn lcd(&self) -> [[u8; 144]; 160] {
//         self.lcd[(self.current_buffer + 1) % 2]
//     }
//
//     fn reset(&mut self, mmu: &mut MMU) {
//         for i in self.lcd.iter_mut() {
//             for j in i {
//                 for k in j {
//                     *k = 0;
//                 }
//             }
//         }
//         self.internal_ly = 0;
//         self.oam_cycle_used = 0;
//         self.sprite_buffer.clear();
//         self.vblank_cycle_used = 0;
//         self.draw_hblank_cycle_used = 0;
//         self.oam_search.reset();
//         self.lcd_transfer.reset(true);
//         self.stat_interrupt_line = false;
//         self.mode = Mode::HBlank;
//         let mut stat = mmu.lcdstat();
//         stat.set_mode(Mode::HBlank);
//         mmu.set_lcdstat(stat);
//         mmu.set_ly(0);
//     }
// }