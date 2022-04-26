use crate::cartridge::metadata::Metadata;
use crate::cartridge::Cartridge;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
#[cfg(not(target_arch = "wasm32"))]
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};

enum RamRtcMode {
    Ram,
    Rtc,
    None,
}

pub struct CartridgeMBC5 {
    rom_hi_bank_number: u16,
    ram_bank_number: u8,
    switchable_mode: bool,
    data: Vec<u8>,
}

impl Cartridge for CartridgeMBC5 {
    fn get(&self, address: usize) -> u8 {
        match address {
            0x0000..=0x3FFF => {
                // println!("GET SWITCH MODE: {} {:#06x}", self.switchable_mode, address);
                // if !self.switchable_mode {
                self.data[address]
                // } else {
                //     self.data[0x4000 + address]
                // }
            }
            0x4000..=0x7FFF => {
                self.data[(0x4000 * self.rom_hi_bank_number as usize) + (address - 0x4000)]
            }
            _ => panic!("Cartridge MBC5 GET {:#06X}", address),
        }
    }

    fn set(&mut self, address: usize, val: u8) {
        match address {
            0x2000..=0x2FFF => {
                // let new = (val as u16 & self.rom_number_bit_mask() as u16);
                // println!("NEW ROMBANK: {}", new);
                self.rom_hi_bank_number = (self.rom_hi_bank_number & 0xFF00) | val as u16;
            }
            0x3000..=0x3FFF => {
                // let new = (val as u16 & self.rom_number_bit_mask() as u16);
                // println!("NEW ROMBANK: {}", new);
                self.rom_hi_bank_number = (self.rom_hi_bank_number & 0x00FF) | ((val as u16) << 8);
            }
            0x4000..=0x5FFF => self.ram_bank_number = val,
            // if self.ram_rtc_enabled {
            //     let address = match self.metadata.ram_size().bank_size {
            //         1 | 2 => (address as usize - 0xA000) % self.metadata.ram_size().size as usize,
            //         _ => panic!("Cartridge MBC1 WRITE UNSUPPORTED RAM:{:?} {:#06x} {:#02x}", self.metadata.ram_size().bank_size, address, val),
            //     };
            //     self.ram[address] = val;
            //     self.save_file.write_at(&self.ram[address..=address], address as u64);
            // } else {
            //     println!("Cartridge MBC1 WRITE TO DISABLED RAM {:#06X} {:04X}", address, val)
            // }
            _ => println!("Cartridge MBC5 SET {:#06X} {:04X}", address, val),
        }
    }

    fn data(&self) -> &Vec<u8> {
        &self.data
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata()
    }

    fn save_data_mut(&mut self) -> Option<&mut Vec<u8>> {
        None
    }

    fn save_data(&self) -> Option<&Vec<u8>> { None }

}

impl CartridgeMBC5 {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            rom_hi_bank_number: 1,
            ram_bank_number: 0,
            switchable_mode: false,
            data: bytes,
        }
    }
}
