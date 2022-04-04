use crate::cartridge::Cartridge;
use crate::cartridge::metadata::Metadata;

pub struct CartridgeMBC1 {
    metadata: Metadata,
    rom_hi_bank_number: u16,
    ram_bank_number: u8,
    switchable_mode: bool,
    data: Vec<u8>,
}

impl Cartridge for CartridgeMBC1 {
    fn get(&self, address: usize) -> u8 {
        match address {
            0x0000..=0x3FFF => self.data[address],
            0x4000..=0x7FFF => self.data[(0x4000 * self.rom_hi_bank_number as usize) + (address - 0x4000)],
            0xA000..=0xBFFF => {
                println!("Reading external ram unsupported for type:{:?} address:{:#06x}", self.metadata.cartridge_type(), address);
                0xFF
            },
            _ => panic!("Cartridge MBC1 GET {:#06X}", address)
        }
    }

    fn set(&mut self, address: usize, val: u8) {
        match address {
            0x0000..=0x1FFF => {
                // println!("Enable/disable external ram unsupported for type:{:?} address:{:#06x} value:{:#04x}", self.metadata.cartridge_type(), address, val)
            },
            0x2000..=0x3FFF => {
                let new = (val as u16 & self.rom_number_bit_mask() as u16);
                self.rom_hi_bank_number = if new == 0 { 1 } else { new };
            }
            0x4000..=0x5FFF => println!("Switching external ram bank unsupported for type:{:?} address:{:#06x} value:{:#04x}", self.metadata.cartridge_type(), address, val),
            0x6000..=0x7FFF => self.switchable_mode = val & 1 == 1,
            0xA000..=0xBFFF => println!("Writing external ram unsupported for type:{:?} address:{:#06x} value:{:#04x}", self.metadata.cartridge_type(), address, val),
            _ => panic!("Cartridge MBC1 SET {:#06X} {:04X}", address, val)
        }
    }

    fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

impl CartridgeMBC1 {
    pub fn new(metadata: Metadata, bytes: Vec<u8>) -> Self {
        Self {
            metadata,
            rom_hi_bank_number: 1,
            ram_bank_number: 0,
            switchable_mode: false,
            data: bytes,
        }
    }


    fn rom_number_bit_mask(&self) -> u8 {
        match self.metadata.rom_size().bank_size {
            002 => 0b0000_0001,
            004 => 0b0000_0011,
            008 => 0b0000_0111,
            016 => 0b0000_1111,
            032 => 0b0001_1111,
            064 => 0b0001_1111,
            128 => 0b0001_1111,
            _ => panic!("Unknown rom bank size: {}", self.metadata.rom_size().bank_size)
        }
    }
}