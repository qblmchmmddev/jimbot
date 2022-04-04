use crate::cartridge::Cartridge;

pub struct CartridgeRomOnly {
    data: Vec<u8>,
}

impl Cartridge for CartridgeRomOnly {

    fn get(&self, address: usize) -> u8 {
        self.data[address]
    }

    fn set(&mut self, address: usize, val: u8) {}

    fn data(&self) -> &Vec<u8> {
        &self.data
    }
}



impl CartridgeRomOnly {
    // pub fn new(file_path: &str) -> Self {
    //     let data = std::fs::read(file_path).unwrap();
    //     // let size = RamSize::from(data[Self::RAM_SIZE_ADDRESS]);
    //     let cartridge = CartridgeRomOnly {
    //         data,
    //         // ram: vec![0; size.size as usize],
    //     };
    //     // println!("Cartridge loaded:");
    //     // println!("\tTitle: {}", cartridge.get_title());
    //     // println!("\tCartridge Type: {:?}", cartridge.get_type());
    //     // println!("\tRom Size: {:?}", cartridge.get_rom_size());
    //     // println!("\tRam Size: {:?}", cartridge.get_ram_size());
    //     cartridge
    // }

    pub fn new(bytes: Vec<u8>) -> Self {
        let data = bytes;
        // let size = RamSize::from(data[Self::RAM_SIZE_ADDRESS]);
        let cartridge = CartridgeRomOnly {
            data,
            // ram: vec![0; size.size as usize],
        };
        // println!("Cartridge loaded:");
        // println!("\tTitle: {}", cartridge.get_title());
        // println!("\tCartridge Type: {:?}", cartridge.get_type());
        // println!("\tRom Size: {:?}", cartridge.get_rom_size());
        // println!("\tRam Size: {:?}", cartridge.get_ram_size());
        cartridge
    }






    // pub fn get_title(&self) -> &str {
    //     str::from_utf8(&self.data[CartridgeRomOnly::TITLE_ADDRESS_MIN..CartridgeRomOnly::TITLE_ADDRESS_MAX]).unwrap_or("N/A")
    // }
    //
    // pub fn get_type(&self) -> CartridgeType {
    //     self.data[CartridgeRomOnly::TYPE_ADDRESS].into()
    // }
    //
    // pub fn get_rom_size(&self) -> RomSize {
    //     self.data[CartridgeRomOnly::ROM_SIZE_ADDRESS].into()
    // }
    //
    // pub fn get_ram_size(&self) -> RamSize {
    //     self.data[CartridgeRomOnly::RAM_SIZE_ADDRESS].into()
    // }
    //
    // pub fn get(&self, address: u16) -> u8 {
    //     self.data[address as usize]
    // }
    //
    // pub fn set_ram(&mut self, address: u16, to: u8) {
    //     let ram_size = self.get_ram_size();
    //     let address = match ram_size.banks_size {
    //         1 | 2 => (address as usize - 0xA000) % ram_size.size as usize,
    //         _ => panic!("Writing ext ram unsupported size:{:?} {:#06x} {:#02x}", ram_size, address, to),
    //     };
    //     self.ram[address] = to;
    // }
    //
    // pub fn get_ram(&self, address: u16) -> u8 {
    //     let ram_size = self.get_ram_size();
    //     let address = match ram_size.banks_size {
    //         1 | 2 => (address as usize - 0xA000) % ram_size.size as usize,
    //         _ => panic!("Reading ext ram unsupported size:{:?} {:#06x}", ram_size, address),
    //     };
    //     self.ram[address as usize]
    // }
    // pub fn data(&self) -> &Vec<u8> {
    //     &self.data
    // }
}