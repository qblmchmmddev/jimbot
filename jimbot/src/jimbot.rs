use crate::apu::APU;
use crate::cartridge;
use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::mmu::{joypad, MMU};
use crate::ppu::PPU;
use std::env;
use crate::saver::Saver;

pub struct Jimbot {
    mmu: MMU,
    cpu: CPU,
    ppu: PPU,
    error_message: Option<String>,
    i: u8,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for Jimbot {
    fn default() -> Self {
        let boot_rom = include_bytes!("../roms/dmg_boot.bin").to_owned();
        let args: Vec<String> = std::env::args().collect();
        let cartridge_file_path = &args[1];
        let cartridge = cartridge::new_cartridge_from_file_path(cartridge_file_path.to_owned());
        Self {
            mmu: MMU::new(boot_rom, Some(cartridge)),
            cpu: CPU::default(),
            ppu: PPU::default(),
            error_message: None,
            i: 0,
        }
    }
}

impl Jimbot {
    pub fn new_with_cartridge_bytes(saver: Option<Box<dyn Saver>> ,bytes: Vec<u8>) -> Self {
        let boot_rom = include_bytes!("../roms/dmg_boot.bin").to_owned();
        let cartridge = cartridge::new_cartridge_from_bytes(saver, bytes);
        Self {
            mmu: MMU::new(boot_rom, Some(cartridge)),
            cpu: CPU::default(),
            ppu: PPU::default(),
            error_message: None,
            i: 0,
        }
    }

    pub fn run(&mut self) {
        if self.error_message.is_some() {
            return;
        }
        self.error_message = self.cpu.cycle(&mut self.mmu).err();
        for _ in 0..4 {
            self.mmu.cycle_timer();
            self.mmu.cycle_apu();
            self.ppu.cycle(&mut self.mmu);
        }
    }
    pub fn mmu(&self) -> &MMU {
        &self.mmu
    }

    pub fn test(&mut self) -> &mut i8 {
        self.mmu.test_mut()
    }

    pub fn cpu(&self) -> &CPU {
        &self.cpu
    }
    pub fn error_message(&self) -> &Option<String> {
        &self.error_message
    }
    pub fn clear_error(&mut self) {
        self.error_message = None
    }
    pub fn ppu(&self) -> &PPU {
        &self.ppu
    }
    pub fn get_sound_data(&mut self) -> Vec<f32> {
        self.mmu.apu.get_data()
    }
    pub fn joypad_press(&mut self, key: joypad::Key) {
        self.mmu.joypad_press(key);
    }

    pub fn joypad_release(&mut self, key: joypad::Key) {
        self.mmu.joypad_release(key);
    }
}
