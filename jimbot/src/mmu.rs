pub mod lcdstat;
pub mod sprite;
pub mod lcdc;
pub mod bgp;
pub mod tac;
pub mod interrupt_flag;
pub mod joypad;

use std::ptr::addr_of;
use crate::apu::APU;
use crate::cartridge::{Cartridge};
use crate::mmu::bgp::{BGP, OBP};
use crate::mmu::interrupt_flag::{InterruptRequest, Interrupts};
use crate::mmu::joypad::JoyPad;
use crate::mmu::lcdc::LCDC;
use crate::mmu::lcdstat::{LCDSTAT, Mode};
use crate::timer::Timer;

pub struct MMU {
    interrupt_flags: u8,
    interrupt_enables: u8,
    boot_mode: bool,
    boot_rom: [u8; 0x100],
    cart: Option<Box<dyn Cartridge>>,
    vram: [u8; 0x2000],
    bgp: u8,
    lcdc: u8,
    lcdstat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    pub(crate) apu: APU,
    wram: [u8; 0x2000],
    oam: [u8; 0xA0],
    hram: [u8; 0x7F],
    timer: Timer,
    wx: u8,
    wy: u8,
    obp0: u8,
    obp1: u8,
    lyc: u8,
    serial_transfer_data: u8,
    serial_transfer_control: u8,
    joypad: JoyPad,
    test: i8,
}

impl MMU {
    pub fn test_mut(&mut self) -> &mut i8 {
        &mut self.test
    }
    pub fn test(&self) -> i8 {
        self.test
    }
    pub fn new(boot_rom: [u8; 0x100], cartridge: Option<Box<dyn Cartridge>>) -> Self {
        Self {
            test: 0,
            interrupt_flags: 0,
            interrupt_enables: 0,
            boot_mode: true,
            boot_rom,
            cart: cartridge,
            vram: [0x00; 0x2000],
            bgp: 0,
            lcdc: 0,
            lcdstat:0x84,
            scy: 0,
            scx: 0,
            ly: 0,
            apu: APU::default(),
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            hram: [0; 0x7F],
            timer: Default::default(),
            wx: 0,
            wy: 0,
            obp0: 0,
            obp1: 0,
            lyc: 0,
            joypad: JoyPad::default(),
            serial_transfer_control: 0,
            serial_transfer_data: 0,
        }
    }
    pub fn get(&self, address: u16) -> u8 {
        let address_usize = address as usize;
        let val = match address {
            0x0000..=0x00FF => {
                if self.boot_mode {
                    self.boot_rom[address_usize]
                } else {
                    self.cart.as_ref().expect(&format!("No cartridge {:#06X}", address_usize)).get(address as usize)
                }
            }
            0x0100..=0x7FFF => self.cart.as_ref().expect(&format!("No cartridge {:#06X}", address_usize)).get(address as usize),
            0x8000..=0x9FFF => {
                // println!("GET VRAM {:#06X}={:#06X}", address_usize, address_usize - 0x8000);
                self.vram[address_usize - 0x8000]
            }
            0xA000..=0xBFFF => {
                if let Some(cart) = self.cart.as_ref() {
                    cart.get(address_usize)
                } else {
                    panic!("Get EXT RAM with no cartridge {:#06X}", address_usize)
                }
            }
            0xC000..=0xDFFF => self.wram[address_usize - 0xC000],
            0xE000..=0xFDFF => self.get(address - 0x2000),
            0xFE00..=0xFE9F => self.oam[address_usize - 0xFE00],
            0xFF00 => self.joypad.bytes(),
            0xFF01 => self.serial_transfer_data,
            0xFF02 => self.serial_transfer_control,
            0xFF04 => self.timer.get(address_usize),
            0xFF05 => self.timer.get(address_usize),
            0xFF06 => self.timer.get(address_usize),
            0xFF07 => self.timer.get(address_usize),
            0xFF0F => self.interrupt_flags,
            0xFF10..=0xFF14 => self.apu.get(address_usize),
            0xFF16..=0xFF1E => self.apu.get(address_usize),
            0xFF20..=0xFF26 => self.apu.get(address_usize),
            0xFF27..=0xFF2F => {
                println!("Get ??? {:#06X}", address_usize);
                0xFF
            }
            0xFF30..=0xFF3f => self.apu.get(address_usize),
            0xFF40 => self.lcdc,
            0xFF41 => self.lcdstat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF48 => self.obp0,
            0xFF47 => self.bgp,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            0xFF4C..=0xFF7F => {
                println!("Get to unusable io: {:#06X}", address_usize);
                0xFF
            }
            0xFF80..=0xFFFE => self.hram[address_usize - 0xFF80],
            0xFFFF => self.interrupt_enables.into(),
            _ => {
                println!("Get ??? {:#06X}", address_usize);
                0xFF
            }
        };
        // println!("GET: {:#06x}->{:#04x}", address, val);
        val
    }
    pub fn set(&mut self, address: u16, val: u8) {
        let address_usize = address as usize;
        // println!("SET: {:#06x}->{:#04x}", address, val);
        match address_usize {
            0x0000..=0x7FFF => if !self.boot_mode {
                if let Some(cart) = self.cart.as_mut() {
                    cart.set(address_usize, val);
                }
            }
            0x8000..=0x9FFF => {
                // println!("SET VRAM {:#06X}={:#06X} => {:#04X}", address_usize, address_usize - 0x8000, val);
                self.vram[address_usize - 0x8000] = val
            }
            0xA000..=0xBFFF => {
                if let Some(cart) = self.cart.as_mut() {
                    cart.set(address_usize, val);
                }
            }
            0xC000..=0xDFFF => self.wram[address_usize - 0xC000] = val,
            0xE000..=0xFDFF => self.set(address - 0x2000, val),
            0xFE00..=0xFE9F => self.oam[address_usize - 0xFE00] = val,
            0xFEA0..=0xFEFF => println!("Write to unusable io: {:#06X} {}", address_usize, val),
            0xFF00 => self.joypad.write(val),
            0xFF01 => self.serial_transfer_data = val,
            0xFF02 => self.serial_transfer_control = val,
            0xFF04 => self.timer.set(address_usize, val),
            0xFF05 => self.timer.set(address_usize, val),
            0xFF06 => self.timer.set(address_usize, val),
            0xFF07 => self.timer.set(address_usize, val),
            0xFF0F => self.interrupt_flags = val,
            0xFF10..=0xFF14 => self.apu.set(address_usize, val),
            0xFF16..=0xFF1E => self.apu.set(address_usize, val),
            0xFF20..=0xFF26 => self.apu.set(address_usize, val),
            0xFF27..=0xFF2F => println!("Set ??? {:#06X} {}", address_usize, val),
            0xFF30..=0xFF3f => self.apu.set(address_usize, val),
            0xFF40 => {
                self.lcdc = val;
                // println!("WRITE LCDC: {:08b} ly:{}", self.lcdc, self.ly);
            }
            0xFF41 => {
                // println!("WRITE LSTAT: {:08b}", val);
                self.lcdstat = (self.lcdstat & 0b11) | (val & 0b1111_1100);
                // println!("NOW LSTAT: {:08b}", self.lcdstat);
            }
            0xFF42 => self.scy = val,
            0xFF43 => {
                self.scx = val;
            },
            0xFF44 => {
                println!("Write to ly: {:#06X} {}", address_usize, val)
            },
            0xFF45 => self.lyc = val,
            0xFF46 => self.dma(val),
            0xFF47 => {
                self.bgp = val;
                // println!("NEW BGP : {:#08b}", self.bgp);
            }
            0xFF48 => {
                self.obp0 = val;
                // println!("NEW OBP0: {:#08b}", self.obp0);
            }
            0xFF49 => {
                self.obp1 = val;
                // println!("NEW OBP1: {:#08b}", self.obp1);
            }
            0xFF4A => self.wy = val,
            0xFF4B => self.wx = if val < 7 { 7 } else { val },
            0xFF50 => self.boot_mode = false,
            0xFF4C..=0xFF79 => println!("Write to unusable io: {:#06X} {}", address_usize, val),
            0xFF80..=0xFFFE => self.hram[address_usize - 0xFF80] = val,
            0xFFFF => self.interrupt_enables = val,
            _ => {}//println!("Set ??? {:#06X} {}", address_usize, val),
        }
    }
    // todo takes 160 m cycles
    fn dma(&mut self, source_high_nibble: u8) {
        let start_source: u16 = ((source_high_nibble as u16) << 8) | 0x0000;
        let end_source: u16 = ((source_high_nibble as u16) << 8) | 0x009F;

        for (i_to, i_from) in (start_source..end_source).enumerate() {
            self.oam[i_to] = self.get(i_from);
        }
    }
    pub fn boot_rom(&self) -> &[u8; 0x100] {
        &self.boot_rom
    }
    pub fn cartridge(&self) -> &Option<Box<dyn Cartridge>> {
        &self.cart
    }
    pub fn vram(&self) -> &[u8; 0x2000] {
        &self.vram
    }
    pub fn hram(&self) -> &[u8; 0x7F] {
        &self.hram
    }
    pub fn wram(&self) -> &[u8; 0x2000] {
        &self.wram
    }
    pub fn apu(&self) -> &APU {
        &self.apu
    }
    pub fn bgp(&self) -> BGP { self.bgp.into() }
    pub fn obp0(&self) -> OBP { self.obp0.into() }
    pub fn obp1(&self) -> OBP { self.obp1.into() }
    pub fn lcdstat(&self) -> LCDSTAT {
        self.lcdstat.into()
    }
    pub fn lcdc(&self) -> LCDC {
        self.lcdc.into()
    }
    pub fn ly(&self) -> u8 { self.ly }
    pub fn set_ly(&mut self, val: u8) { self.ly = val }
    pub fn scx(&self) -> u8 { self.scx }
    pub fn scy(&self) -> u8 { self.scy }
    pub fn set_lcdstat(&mut self, stat: LCDSTAT) {
        self.lcdstat = stat.into();
    }
    pub fn oam(&self) -> &[u8; 0xA0] {
        &self.oam
    }

    pub fn cycle_timer(&mut self) {
        if self.timer.cycle(&mut self.apu) {
            self.request_interrupt(InterruptRequest::Timer);
        }
    }

    pub fn cycle_apu(&mut self) {
        self.apu.cycle();
    }

    pub fn get_interrupt_flags(&self) -> Interrupts {
        self.interrupt_flags.into()
    }

    pub fn get_interrupt_enables(&self) -> Interrupts {
        self.interrupt_enables.into()
    }

    pub fn set_interrupt_flags(&mut self, flags: Interrupts) {
        self.interrupt_flags = flags.into();
    }

    pub fn set_interrupt_enables(&mut self, enables: Interrupts) {
        self.interrupt_enables = enables.into();
    }

    pub fn lyc(&self) -> u8 {
        self.lyc
    }

    pub fn request_interrupt(&mut self, request: InterruptRequest) {
        let mut iflag: Interrupts = self.interrupt_flags.into();
        iflag.enable_request(request);
        self.interrupt_flags = iflag.into();
    }

    pub fn joypad_press(&mut self, key: joypad::Key) {
        self.joypad.press(key);
        self.request_interrupt(InterruptRequest::Joypad);
    }

    pub fn joypad_release(&mut self, key: joypad::Key) {
        self.joypad.release(key);
    }
    pub fn wx(&self) -> u8 {
        self.wx
    }
    pub fn wy(&self) -> u8 {
        self.wy
    }
}