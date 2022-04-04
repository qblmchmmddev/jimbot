use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
#[cfg(not(target_arch = "wasm32"))]
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use crate::cartridge::Cartridge;
use crate::cartridge::metadata::Metadata;

pub struct CartridgeMBC1RamBattery {
    save_file: File,
    metadata: Metadata,
    rom_hi_bank_number: u16,
    ram_bank_number: u8,
    ram_enabled: bool,
    switchable_mode: bool,
    data: Vec<u8>,
    ram: Vec<u8>,
}

impl Cartridge for CartridgeMBC1RamBattery {
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
            0x4000..=0x7FFF => self.data[(0x4000 * self.rom_hi_bank_number as usize) + (address - 0x4000)],
            0xA000..=0xBFFF => {
                if !self.ram_enabled { return 0xFF; };
                let address = match self.metadata.ram_size().bank_size {
                    1 | 2 => (address as usize - 0xA000) % self.metadata.ram_size().size as usize,
                    _ => panic!("Reading ext ram unsupported bank size:{:?} {:#06x}", self.metadata.ram_size().bank_size, address),
                };
                self.ram[address as usize]
            }
            _ => panic!("Cartridge MBC1 GET {:#06X}", address)
        }
    }

    fn set(&mut self, address: usize, val: u8) {
        match address {
            0x0000..=0x1FFF => {
                let enable_ram = val & 0xF == 0xA;
                self.ram_enabled = enable_ram;
            }
            0x2000..=0x3FFF => {
                let new = (val as u16 & self.rom_number_bit_mask() as u16);
                // println!("NEW RBANK: {}", new);
                self.rom_hi_bank_number = if new == 0 { 1 } else { new };
            }
            0x4000..=0x5FFF => self.ram_bank_number = self.ram_bank_number | val & 0b11,
            0x6000..=0x7FFF => self.switchable_mode = val & 1 == 1,
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    let address = match self.metadata.ram_size().bank_size {
                        1 | 2 => (address as usize - 0xA000) % self.metadata.ram_size().size as usize,
                        _ => panic!("Cartridge MBC1 WRITE UNSUPPORTED RAM:{:?} {:#06x} {:#02x}", self.metadata.ram_size().bank_size, address, val),
                    };
                    self.ram[address] = val;
                    #[cfg(not(target_arch = "wasm32"))]
                    self.save_file.write_at(&self.ram[address..=address], address as u64);
                } else {
                    println!("Cartridge MBC1 WRITE TO DISABLED RAM {:#06X} {:04X}", address, val)
                }
            }
            _ => panic!("Cartridge MBC1 SET {:#06X} {:04X}", address, val)
        }
    }

    fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

impl CartridgeMBC1RamBattery {
    pub fn new(file_path: String, metadata: Metadata, bytes: Vec<u8>) -> Self {
        let mut save_file_path = Path::new(&file_path).to_owned();
        save_file_path.pop();
        save_file_path.push(format!("{}.sav", metadata.title()));
        let save_file_path = save_file_path.to_str().expect("Cannot create save file").to_string();
        let mut ram = vec![0u8; metadata.ram_size().size as usize];
        let save_file = if let Ok(mut save_file) = OpenOptions::new()
            .create(false)
            .write(true)
            .read(true)
            .open(&save_file_path) {
            // Save file exist
            println!("SAVE FILE FOUND: {:?}", save_file);
            let file_metadata = save_file.metadata().expect("Cannot get metadata");
            if file_metadata.len() != metadata.ram_size().size { panic!("Corrupted save file: {}, size: {}, ram size: {}", save_file_path, file_metadata.len(), metadata.ram_size().size) }
            save_file.read(&mut ram).expect(&format!("Cannot read save file: {}", &save_file_path));
            save_file
        } else {
            // Create file
            let mut file = OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(&save_file_path)
                .expect(&format!("Cannot create save file: {}", save_file_path));
            file.write_all(&ram).expect(&format!("Filed to write file: {}", save_file_path));
            file
        };
        Self {
            save_file,
            metadata,
            rom_hi_bank_number: 1,
            ram_bank_number: 0,
            ram_enabled: false,
            switchable_mode: false,
            data: bytes,
            ram,
        }
    }

    fn rom_number_bit_mask(&self) -> u8 {
        let rom_bank_size = self.metadata.rom_size().bank_size;
        match rom_bank_size {
            002 => 0b0000_0001,
            004 => 0b0000_0011,
            008 => 0b0000_0111,
            016 => 0b0000_1111,
            032 => 0b0001_1111,
            064 => 0b0001_1111,
            128 => 0b0001_1111,
            _ => panic!("Unknown rom bank size: {}", rom_bank_size)
        }
    }
}