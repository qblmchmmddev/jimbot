use crate::cartridge::metadata::Metadata;
use crate::cartridge::Cartridge;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
#[cfg(not(target_arch = "wasm32"))]
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};

pub struct CartridgeMBC2Battery {
    save_file: Option<File>,
    metadata: Metadata,
    rom_hi_bank_number: u16,
    ram_enabled: bool,
    data: Vec<u8>,
    ram: Vec<u8>,
}

impl Cartridge for CartridgeMBC2Battery {
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
            0xA000..=0xBFFF => {
                if !self.ram_enabled { return 0xFF; };
                self.ram[address as usize & 0x1FF]
            }
            _ => panic!("Cartridge MBC1 GET {:#06X}", address),
        }
    }

    fn set(&mut self, address: usize, val: u8) {
        match address {
            0x0000..=0x3FFF => {
                if address & 0x100 == 0 {
                    self.ram_enabled = val & 0xF == 0xA;
                } else {
                    self.rom_hi_bank_number = if val & 0xF == 0 { 1 } else { (val & 0xF) as u16 };
                }
            }
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    let address = address & 0x1FF;
                    self.ram[address] = val & 0xF;
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(save_file) = self.save_file.as_mut() {
                        save_file.write_at(&self.ram[address..=address], address as u64);
                    }
                } else {
                    println!(
                        "Cartridge MBC1 WRITE TO DISABLED RAM {:#06X} {:04X}",
                        address, val
                    )
                }
            }
            _ => println!("Cartridge MBC2RAM SET {:#06X} {:04X}", address, val),
        }
    }

    fn data(&self) -> &Vec<u8> {
        &self.data
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn save_data_mut(&mut self) -> Option<&mut Vec<u8>> {
        Some(&mut self.ram)
    }

    fn save_data(&self) -> Option<&Vec<u8>> { Some(&self.ram) }

}

impl CartridgeMBC2Battery {
    pub fn new(file_path: Option<String>, metadata: Metadata, bytes: Vec<u8>) -> Self {
        let mut ram = vec![0u8; 512];

        let save_file = if let Some(file_path) = file_path {
            let mut save_file_path = Path::new(&file_path).to_owned();
            save_file_path.pop();
            save_file_path.push(format!("{}.sav", metadata.title()));
            let save_file_path = save_file_path
                .to_str()
                .expect("Cannot create save file")
                .to_string();
            let save_file = if let Ok(mut save_file) = OpenOptions::new()
                .create(false)
                .write(true)
                .read(true)
                .open(&save_file_path)
            {
                // Save file exist
                println!("SAVE FILE FOUND: {:?}", save_file);
                let file_metadata = save_file.metadata().expect("Cannot get metadata");
                if file_metadata.len() != ram.len() as u64 {
                    panic!(
                        "Corrupted save file: {}, size: {}, ram size: {}",
                        save_file_path,
                        file_metadata.len(),
                        ram.len()
                    )
                }
                save_file
                    .read(&mut ram)
                    .expect(&format!("Cannot read save file: {}", &save_file_path));
                save_file
            } else {
                // Create file
                let mut file = OpenOptions::new()
                    .create(true)
                    .read(true)
                    .write(true)
                    .open(&save_file_path)
                    .expect(&format!("Cannot create save file: {}", save_file_path));
                file.write_all(&ram)
                    .expect(&format!("Filed to write file: {}", save_file_path));
                file
            };
            Some(save_file)
        } else {
            None
        };

        Self {
            save_file,
            metadata,
            rom_hi_bank_number: 1,
            ram_enabled: false,
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
            _ => panic!("Unknown rom bank size: {}", rom_bank_size),
        }
    }
}
