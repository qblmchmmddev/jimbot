use std::env;
use crate::apu::APU;
use crate::cartridge;
use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::mmu::{joypad, MMU};
use crate::ppu::PPU;

pub struct Jimbot {
    mmu: MMU,
    cpu: CPU,
    ppu: PPU,
    error_message: Option<String>,
    i: u8,
}

impl Default for Jimbot {
    fn default() -> Self {
        let boot_rom = include_bytes!("../roms/boot_rom.bin").to_owned();
        #[cfg(not(target_arch = "wasm32"))]
            let args: Vec<String> = std::env::args().collect();
        #[cfg(not(target_arch = "wasm32"))]
            let cartridge_file_path = &args[1];
        #[cfg(target_arch = "wasm32")]
            let cartridge = cartridge::new_cartridge_from_bytes(include_bytes!("../roms/Super Mario Land (World) (Rev 1).gb").to_vec());
        #[cfg(not(target_arch = "wasm32"))]
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
    pub fn run(&mut self) {
        if self.error_message.is_some() { return; }
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
    pub fn error_message(&self) -> &Option<String> { &self.error_message }
    pub fn clear_error(&mut self) { self.error_message = None }
    pub fn ppu(&self) -> &PPU {
        &self.ppu
    }
    pub fn get_sound_data(&mut self) -> Vec<f32> {
        self.mmu.apu.get_data()
    }
    pub fn joypad_press(&mut self, key: joypad::Key) {
        self.mmu.joypad_press(key);
    }

    pub fn joypad_release(&mut self, key: joypad::Key) { self.mmu.joypad_release(key); }
}