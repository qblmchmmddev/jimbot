#[derive(Debug)]
pub struct RomSize {
    pub size: u32,
    pub bank_size: u16,
}

impl From<u8> for RomSize {
    fn from(code: u8) -> Self {
        match code {
            0x00 => RomSize { size: (32 * 1024), bank_size: 2 },
            0x01 => RomSize { size: (64 * 1024), bank_size: 4 },
            0x02 => RomSize { size: (128 * 1024), bank_size: 8 },
            0x03 => RomSize { size: (256 * 1024), bank_size: 16 },
            0x04 => RomSize { size: (512 * 1024), bank_size: 32 },
            0x05 => RomSize { size: (1 * 1024 * 1024), bank_size: 64 },
            0x06 => RomSize { size: (2 * 1024 * 1024), bank_size: 128 },
            0x07 => RomSize { size: (4 * 1024 * 1024), bank_size: 256 },
            0x08 => RomSize { size: (8 * 1024 * 1024), bank_size: 512 },
            0x52 => RomSize { size: (1.1 * 1024.0 * 1024.0) as u32, bank_size: 72 },
            0x53 => RomSize { size: (1.2 * 1024.0 * 1024.0) as u32, bank_size: 80 },
            0x54 => RomSize { size: (3 * 1024 * 1024 / 2), bank_size: 96 },
            _ => panic!("Unknown rom size code {:02x}", code)
        }
    }
}