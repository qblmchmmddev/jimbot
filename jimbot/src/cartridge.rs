use std::str;
use crate::cartridge::cartridge_mbc1::CartridgeMBC1;
use crate::cartridge::cartridge_mbc1_ram::CartridgeMBC1Ram;
use crate::cartridge::cartridge_mbc1_ram_battery::CartridgeMBC1RamBattery;
use crate::cartridge::cartridge_mbc2_battery::CartridgeMBC2Battery;
use crate::cartridge::cartridge_mbc3_ram_battery::CartridgeMBC3RamBattery;
use crate::cartridge::cartridge_rom_only::CartridgeRomOnly;
use crate::cartridge::cartridge_type::CartridgeType;
use crate::cartridge::metadata::Metadata;
use crate::cartridge::ram_size_type::RamSize;
use crate::cartridge::rom_size_type::RomSize;
use crate::saver::Saver;

use self::cartridge_mbc5::CartridgeMBC5;
use self::cartridge_mbc5_ram_battery::CartridgeMBC5RamBattery;

pub mod cartridge_type;
pub mod ram_size_type;
pub mod rom_size_type;
mod cartridge_rom_only;
mod cartridge_mbc1;
mod cartridge_mbc1_ram_battery;
mod metadata;
mod cartridge_mbc3_ram_battery;
mod cartridge_mbc5_ram_battery;
mod cartridge_mbc5;
mod cartridge_mbc1_ram;
mod cartridge_mbc2_battery;

pub trait Cartridge: Sync + Send {
    // fn new(file_path: &str) -> Self;
    // fn new_bytes(bytes: Vec<u8>) -> Self;
    fn get(&self, address: usize) -> u8;
    fn set(&mut self, address: usize, val: u8);
    fn data(&self) -> &Vec<u8>;

    // fn get_title(&self) -> &str {
    //     str::from_utf8(self.data()[TITLE_ADDRESS_MIN..=Meta TITLE_ADDRESS_MAX]).expect("NO TITLE")
    // }
    //
    // fn get_rom_size(&self) -> RomSize {
    //     self.data()[Self::ROM_SIZE_ADDRESS].into()
    // }
    //
    // fn get_ram_size(&self) -> RamSize {
    //     self.data()[Self::RAM_SIZE_ADDRESS].into()
    // }
}

pub fn new_cartridge_from_bytes(saver: Option<Box<dyn Saver>>, bytes: Vec<u8>) -> Box<dyn Cartridge> {
    let metadata = Metadata::from(&bytes);
    match metadata.cartridge_type() {
        CartridgeType::RomOnly => Box::new(CartridgeRomOnly::new(bytes)),
        CartridgeType::RomMbc1 => Box::new(CartridgeMBC1::new(metadata, bytes)),
        CartridgeType::RomMbc5 => Box::new(CartridgeMBC5::new(bytes)),
        CartridgeType::RomMbc1Ram => Box::new(CartridgeMBC1Ram::new(metadata, bytes)),
        CartridgeType::RomMbc1RamBattery => Box::new(CartridgeMBC1RamBattery::new(saver, None, metadata, bytes)),
        CartridgeType::RomMbc2Battery => Box::new(CartridgeMBC2Battery::new(None, metadata, bytes)),
        CartridgeType::RomMbc3RamBattery => Box::new(CartridgeMBC3RamBattery::new(None, metadata, bytes)),
        CartridgeType::RomMbc5RamBattery => Box::new(CartridgeMBC5RamBattery::new(None, metadata, bytes)),
        _ => panic!("Unsupported cartridge: {:?}", metadata.cartridge_type()),
    }
}

pub fn new_cartridge_from_file_path(file_path: String) -> Box<dyn Cartridge> {
    let bytes = std::fs::read(&file_path).unwrap();
    let metadata = Metadata::from(&bytes);
    println!("{:#?}", metadata);
    match metadata.cartridge_type() {
        CartridgeType::RomOnly => Box::new(CartridgeRomOnly::new(bytes)),
        CartridgeType::RomMbc1 => Box::new(CartridgeMBC1::new(metadata, bytes)),
        CartridgeType::RomMbc5 => Box::new(CartridgeMBC5::new(bytes)),
        CartridgeType::RomMbc1Ram => Box::new(CartridgeMBC1Ram::new(metadata, bytes)),
        CartridgeType::RomMbc1RamBattery => Box::new(CartridgeMBC1RamBattery::new(None, Some(file_path), metadata, bytes)),
        CartridgeType::RomMbc2Battery => Box::new(CartridgeMBC2Battery::new(Some(file_path), metadata, bytes)),
        CartridgeType::RomMbc3RamBattery => Box::new(CartridgeMBC3RamBattery::new(Some(file_path), metadata, bytes)),
        CartridgeType::RomMbc5RamBattery => Box::new(CartridgeMBC5RamBattery::new(Some(file_path), metadata, bytes)),
        _ => panic!("Unsupported cartridge: {:?}", metadata.cartridge_type()),
    }
}


// pub fn new_cartridge_from_bytes(bytes: Vec<u8>) -> Box<dyn Cartridge> {
//     let metadata = Metadata::from(&bytes);
//     // let rom_size = RomSize::from(bytes[ROM_SIZE_ADDRESS]);
//     // let ram_size = RamSize::from(bytes[RAM_SIZE_ADDRESS]);
//     // let cart_type = CartridgeType::from(bytes[TYPE_ADDRESS]);
//     // let title = str::from_utf8(&bytes[TITLE_ADDRESS_MIN..=TITLE_ADDRESS_MAX]).unwrap_or("N/A");
//     //
//     // println!("Cartridge loaded:");
//     // println!("\tTitle: {}", title);
//     // println!("\tCartridge Type: {:?}", cart_type);
//     // println!("\tRom Size: {:?}", rom_size);
//     // println!("\tRam Size: {:?}", ram_size);
//     match metadata.cartridge_type() {
//         CartridgeType::RomOnly => Box::new(CartridgeRomOnly::new_bytes(bytes)),
//         // CartridgeType::RomMbc1  => Box::new(CartridgeMBC1::new_bytes(bytes, rom_size.banks_size, ram_size.banks_size, ram_size.size)),
//         CartridgeType::RomMbc1RamBattery => Box::new(CartridgeMBC1RamBattery::new(bytes)),
//         _ => panic!("Unsupported cartridge: {:?}", metadata.cartridge_type()),
//     }
// }

