#[derive(Debug)]
pub struct RamSize {
    pub size: u64,
    pub bank_size: u16,
}

impl From<u8> for RamSize {
    fn from(code: u8) -> Self {
        match code {
            0x00 => RamSize { size: 0, bank_size: 0 },
            0x01 => RamSize { size: 2 * 1024, bank_size: 1 },
            0x02 => RamSize { size: 8 * 1024, bank_size: 1 },
            0x03 => RamSize { size: 32 * 1024, bank_size: 4 },
            0x04 => RamSize { size: 128 * 1024, bank_size: 16 },
            0x05 => RamSize { size: 64 * 1024, bank_size: 8 },
            _ => panic!("Unknown ram size code {:02x}", code)
        }
    }
}