pub struct WRAM([u8; WRAM::SIZE]);

impl Default for WRAM {
    fn default() -> Self {
        Self([0; Self::SIZE])
    }
}

impl WRAM {
    const SIZE: usize = 0x1fff;
}