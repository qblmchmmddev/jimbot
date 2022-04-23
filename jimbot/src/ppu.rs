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
    enable: bool,
    scanline: u8,
    init_enable: bool,
    sprite_buffer: Vec<Sprite>,
    oam_search: OAMSearch,
    lcd_transfer: LCDTransfer,
    scanline_cycle: u16,
    lcd: [[[u8; 144]; 160]; 2],
    current_buffer: usize,
    stat_interrupt_line: bool,
}

impl Default for PPU {
    fn default() -> Self {
        Self {
            scanline: 0,
            enable: false,
            init_enable: false,
            scanline_cycle: 0,
            sprite_buffer: Vec::with_capacity(10),
            oam_search: OAMSearch::default(),
            lcd_transfer: LCDTransfer::default(),
            lcd: [[[0; 144]; 160]; 2],
            current_buffer: 0,
            stat_interrupt_line: false,
        }
    }
}

// 43 to 44 cycle after turn on to change ly to 1
// wait 5 cycle after lcd enable from oamsearch mode (lcdstat:80) to drawing mode (lcdstat:0x83) 0x1CF statcount breakpoint interest
impl PPU {
    pub fn cycle(&mut self, mmu: &mut MMU) {
        let lcdc = mmu.lcdc();
        let mut stat = mmu.lcdstat();
        if self.enable && !lcdc.is_display_enable() {
            if stat.mode() == Mode::VBlank { println!("Disable lcd outside vblank may damage hardware") }
            self.scanline_cycle = 0;
            self.scanline = 0;
            mmu.set_ly(0);
            self.enable = false;
            for lcd in self.lcd.iter_mut() {
                for pixel_rows in lcd.iter_mut() {
                    for pixel in pixel_rows {
                        *pixel = 0;
                    }
                }
            }
            stat.set_mode(Mode::HBlank);
            self.sprite_buffer.clear();
            mmu.set_lcdstat(stat);
            return;
        } else if !self.enable && lcdc.is_display_enable() {
            self.enable = true;
            self.init_enable = true;
            stat.set_mode(Mode::OAMSearch);
        }
        if !self.enable { return; }
        let mut new_stat_interrupt = false;

        match self.scanline {
            0 => match self.scanline_cycle {
                0 => {
                    mmu.set_ly(0);
                    new_stat_interrupt = new_stat_interrupt || stat.vblank_interrupt();
                },
                4 => {
                    stat.set_mode(Mode::OAMSearch);
                    new_stat_interrupt = new_stat_interrupt || stat.oam_interrupt();
                },
                84 => stat.set_mode(Mode::LCDTransfer),
                _ => {}
            }
            1..=143 => match self.scanline_cycle {
                0 => {
                    new_stat_interrupt = new_stat_interrupt || stat.oam_interrupt();
                    mmu.set_ly(self.scanline)
                },
                4 => {
                    let ly_eq_lyc = mmu.ly() == mmu.lyc();
                    new_stat_interrupt = new_stat_interrupt || (ly_eq_lyc && stat.ly_eq_lyc());
                    stat.set_coincidence(ly_eq_lyc);
                    stat.set_mode(Mode::OAMSearch)
                }
                84 => stat.set_mode(Mode::LCDTransfer),
                _ => {}
            }
            144 => match self.scanline_cycle {
                0 => {
                    mmu.set_ly(144);
                    mmu.request_interrupt(InterruptRequest::VBlank);
                },
                4 => {
                    let ly_eq_lyc = mmu.ly() == mmu.lyc();
                    new_stat_interrupt = new_stat_interrupt || (ly_eq_lyc && stat.ly_eq_lyc());
                    new_stat_interrupt = new_stat_interrupt || stat.vblank_interrupt();
                    new_stat_interrupt = new_stat_interrupt || stat.oam_interrupt();
                    stat.set_coincidence(ly_eq_lyc);
                    stat.set_mode(Mode::VBlank)
                }
                5..=u16::MAX => {
                    new_stat_interrupt = new_stat_interrupt || stat.vblank_interrupt();
                }
                _ => {}
            }
            145..=152 => match self.scanline_cycle {
                0 => mmu.set_ly(self.scanline),
                4 => {
                    let ly_eq_lyc = mmu.ly() == mmu.lyc();
                    new_stat_interrupt = new_stat_interrupt || (ly_eq_lyc && stat.ly_eq_lyc());
                    new_stat_interrupt = new_stat_interrupt || stat.oam_interrupt();
                    stat.set_coincidence(ly_eq_lyc);
                }
                _ => {}
            }
            153 => {
                new_stat_interrupt = new_stat_interrupt || stat.vblank_interrupt();
                match self.scanline_cycle {
                    0 => mmu.set_ly(153),
                    4 => {
                        new_stat_interrupt = new_stat_interrupt || stat.oam_interrupt();
                        let ly_eq_lyc = mmu.ly() == mmu.lyc();
                        new_stat_interrupt = new_stat_interrupt || (ly_eq_lyc && stat.ly_eq_lyc());
                        stat.set_coincidence(ly_eq_lyc);
                        mmu.set_ly(0)
                    }
                    12 => {
                        new_stat_interrupt = new_stat_interrupt || stat.oam_interrupt();
                        let ly_eq_lyc = mmu.ly() == mmu.lyc();
                        new_stat_interrupt = new_stat_interrupt || (ly_eq_lyc && stat.ly_eq_lyc());
                        stat.set_coincidence(ly_eq_lyc);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        match stat.mode() {
            Mode::OAMSearch => {
                if self.oam_search.cycle(mmu, &mut self.sprite_buffer) {}
            }
            Mode::LCDTransfer => {
                if self.lcd_transfer.cycle(mmu, &mut self.sprite_buffer, &mut self.lcd[self.current_buffer]) {
                    self.sprite_buffer.clear();
                    stat.set_mode(Mode::HBlank);
                    new_stat_interrupt = new_stat_interrupt || stat.hblank_interrupt();
                }
            }
            Mode::HBlank => {}
            Mode::VBlank => {}
        };
        mmu.set_lcdstat(stat);
        if !self.stat_interrupt_line && new_stat_interrupt { mmu.request_interrupt(InterruptRequest::LCDStat) }
        self.stat_interrupt_line = new_stat_interrupt;

        self.scanline_cycle += 1;
        if self.scanline_cycle >= 456 {
            self.scanline_cycle = 0;
            self.scanline += 1;
            if self.scanline >= 154 {
                self.current_buffer = (self.current_buffer + 1) % 2;
                self.scanline = 0
            }
        }
    }
    pub fn lcd(&self) -> [[u8; 144]; 160] {
        self.lcd[(self.current_buffer + 1) % 2]
    }
}