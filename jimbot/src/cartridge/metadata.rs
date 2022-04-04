use std::str;
use crate::cartridge::cartridge_type::CartridgeType;
use crate::cartridge::ram_size_type::RamSize;
use crate::cartridge::rom_size_type::RomSize;

#[derive(Debug)]
pub struct Metadata {
    title: String,
    cartridge_type: CartridgeType,
    rom_size: RomSize,
    ram_size: RamSize,
}

impl Metadata {
    const TITLE_ADDRESS_MIN: usize = 0x0134;
    const TITLE_ADDRESS_MAX: usize = 0x0142;
    const TYPE_ADDRESS: usize = 0x147;
    const ROM_SIZE_ADDRESS: usize = 0x148;
    const RAM_SIZE_ADDRESS: usize = 0x149;

    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn ram_size(&self) -> &RamSize {
        &self.ram_size
    }
    pub fn rom_size(&self) -> &RomSize {
        &self.rom_size
    }
    pub fn cartridge_type(&self) -> &CartridgeType {
        &self.cartridge_type
    }
}

impl From<&Vec<u8>> for Metadata {
    fn from(from: &Vec<u8>) -> Self {
        let title = str::from_utf8(&from[Self::TITLE_ADDRESS_MIN..=Self::TITLE_ADDRESS_MAX]).expect("NO TITLE").trim_matches(char::from(0)).to_string();
        let ram_size = RamSize::from(from[Self::RAM_SIZE_ADDRESS]);
        let rom_size = RomSize::from(from[Self::ROM_SIZE_ADDRESS]);
        let cartridge_type = CartridgeType::from(from[Self::TYPE_ADDRESS]);
        Self {
            title,
            ram_size,
            rom_size,
            cartridge_type,
        }
    }
}

