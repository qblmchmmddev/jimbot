use crate::cpu::condition::Condition;
use crate::cpu::hex_u16::HexU16;
use crate::cpu::hex_u8::HexU8;
use crate::cpu::registers::{R16, R8};

#[derive(Debug, Copy, Clone)]
pub enum OpArg {
    FetchU8,
    FetchInAddrU8,
    InAddrU8(HexU8),
    Reg8(R8),
    Reg16(R16),
    AddrReg16(R16),
    InAddrReg8(R8),
    AddrU16(HexU16),
    FetchAddrU16(Option<HexU8>, Option<HexU8>),
    FetchI8,
    FetchSPI8,
    SPI8(i8),
    AddrReg16d(R16),
    AddrReg16i(R16),
    AddrRegd16(R16),
    CC(Condition),
    U8(HexU8),
    I8(i8),
    FetchU16(Option<HexU8>, Option<HexU8>),
    U16(HexU16),
    Non,
}