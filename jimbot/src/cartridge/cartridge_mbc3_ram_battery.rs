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

pub struct CartridgeMBC3RamBattery {
    save_file: Option<File>,
    metadata: Metadata,
    rom_hi_bank_number: u16,
    ram_bank_number: u8,
    ram_rtc_enabled: bool,
    switchable_mode: bool,
    ram_rtc_mode: RamRtcMode,
    rtc_data_latch_writes: u8,
    data: Vec<u8>,
    ram: Vec<u8>,
}

impl Cartridge for CartridgeMBC3RamBattery {
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
                match self.ram_rtc_mode {
                    RamRtcMode::Ram => {
                        if self.ram_rtc_enabled {
                            let address =
                                0x2000 * self.ram_bank_number as usize + (address - 0xA000);
                            self.ram[address]
                        } else {
                            0xFF
                            // println!("Cartridge MBC3 Battery WRITE TO DISABLED RAM {:#06X} {:04X}", address, val)
                        }
                    }
                    RamRtcMode::Rtc => {
                        panic!("Read to RTC registers {:#06X}", address)
                    }
                    RamRtcMode::None => panic!("Write to Ram Rtc with mode None"),
                }
            }
            _ => panic!("Cartridge MBC1 GET {:#06X}", address),
        }
    }

    fn set(&mut self, address: usize, val: u8) {
        match address {
            0x0000..=0x1FFF => {
                let enable_ram_rtc = val & 0xF == 0xA;
                self.ram_rtc_enabled = enable_ram_rtc;
            }
            0x2000..=0x3FFF => {
                self.rom_hi_bank_number = if val == 0 { 1 } else { val as u16 };
            }
            0x4000..=0x5FFF => match val {
                0x00..=0x03 => {
                    self.ram_rtc_mode = RamRtcMode::Ram;
                    self.ram_bank_number = val;
                }
                0x08..=0x0C => self.ram_rtc_mode = RamRtcMode::Rtc,
                _ => println!("Unknown ram bank / rtc select {:#04X}", val),
            },
            0x6000..=0x7FFF => {
                if self.rtc_data_latch_writes == 0x00 && val == 0x01 {
                    println!("Enable/start data latch {:#06X} {:#04X}", address, val)
                }
                self.rtc_data_latch_writes = val;
                println!("RTC data latch {:#06X} {:#04X}", address, val)
                // todo!("\
                // A write of the value $00 followed by another write with the value $01 will “latch” (effectively copy) the current state of the RTC registers and make them accessible by using the RTC Register Select feature mentioned above. For details, check out the RTC Section of this documentation.
                //                 ")
                // self.switchable_mode = val & 1 == 1
            }
            0xA000..=0xBFFF => {
                match self.ram_rtc_mode {
                    RamRtcMode::Ram => {
                        if self.ram_rtc_enabled {
                            let address =
                                0x2000 * self.ram_bank_number as usize + (address - 0xA000);
                            self.ram[address] = val;

                            #[cfg(not(target_arch ="wasm32"))]
                            if let Some(save_file) = self.save_file.as_mut() {
                                save_file.write_at(&self.ram[address..=address], address as u64);
                            }
                        } else {
                            println!(
                                "Cartridge MBC3 Battery WRITE TO DISABLED RAM {:#06X} {:04X}",
                                address, val
                            )
                        }
                    }
                    RamRtcMode::Rtc => {
                        todo!("Write to RTC registers {:#06X} {:04X}", address, val);
                    }
                    RamRtcMode::None => panic!("Write to Ram Rtc with mode None"),
                }
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
            }
            _ => panic!("Cartridge MBC1 SET {:#06X} {:04X}", address, val),
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

impl CartridgeMBC3RamBattery {
    pub fn new(file_path: Option<String>, metadata: Metadata, bytes: Vec<u8>) -> Self {
        let mut ram = vec![0u8; metadata.ram_size().size as usize];

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
                if file_metadata.len() != metadata.ram_size().size {
                    panic!(
                        "Corrupted save file: {}, size: {}, ram size: {}",
                        save_file_path,
                        file_metadata.len(),
                        metadata.ram_size().size
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
            ram_bank_number: 0,
            ram_rtc_enabled: false,
            switchable_mode: false,
            ram_rtc_mode: RamRtcMode::None,
            rtc_data_latch_writes: 0xFF,
            data: bytes,
            ram,
        }
    }
}
