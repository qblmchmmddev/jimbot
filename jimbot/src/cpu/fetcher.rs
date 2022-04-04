use OpArg::{AddrReg16, FetchI8, FetchInAddrU8, FetchSPI8, FetchU16, FetchU8};
use crate::cpu::CPU;
use crate::cpu::op::Op;
use crate::cpu::op_arg::OpArg;
use crate::cpu::op_arg::OpArg::{AddrU16, FetchAddrU16, I8, InAddrU8, SPI8, U16, U8};
use crate::cpu::registers::R16::PC;
use crate::cpu::registers::Registers;
use crate::mmu::MMU;

impl CPU {
    pub(crate) fn fetch(instruction: (Op, OpArg, OpArg), registers: &mut Registers, mmu: &MMU) -> (bool, (Op, OpArg, OpArg)) {
        match instruction {
            (op, FetchU8, p2) => (true, (op, U8(mmu.get(registers.get16i(PC, 1)).into()), p2)),
            (op, p1, FetchU8) => (true, (op, p1, U8(mmu.get(registers.get16i(PC, 1)).into()))),
            (op, FetchI8, p2) => (true, (op, I8(mmu.get(registers.get16i(PC, 1)) as i8), p2)),
            (op, p1, FetchI8) => (true, (op, p1, I8(mmu.get(registers.get16i(PC, 1)) as i8))),
            (op, FetchSPI8, p2) => (true, (op, SPI8(mmu.get(registers.get16i(PC, 1)) as i8), p2)),
            (op, p1, FetchSPI8) => (true, (op, p1, SPI8(mmu.get(registers.get16i(PC, 1)) as i8))),
            (op, FetchInAddrU8, p2) => (false, (op, InAddrU8(mmu.get(registers.get16i(PC, 1)).into()), p2)),
            (op, p1, FetchInAddrU8) => (false, (op, p1, InAddrU8(mmu.get(registers.get16i(PC, 1)).into()))),
            (op, FetchU16(None, None), p2) => (false, (op, FetchU16(None, Some(mmu.get(registers.get16i(PC, 1)).into())), p2)),
            (op, FetchU16(None, Some(u8)), p2) => (true, (op, U16(u16::from_be_bytes([mmu.get(registers.get16i(PC, 1)), u8.into()]).into()), p2)),
            (op, p1, FetchU16(None, None)) => (false, (op, p1, FetchU16(None, Some(mmu.get(registers.get16i(PC, 1)).into())))),
            (op, p1, FetchU16(None, Some(u8))) => (true, (op, p1, U16(u16::from_be_bytes([mmu.get(registers.get16i(PC, 1)), u8.into()]).into()))),
            (op, FetchAddrU16(None, None), p2) => (false, (op, FetchAddrU16(None, Some(mmu.get(registers.get16i(PC, 1)).into())), p2)),
            (op, FetchAddrU16(None, Some(u8)), p2) => (true, (op, AddrU16(u16::from_be_bytes([mmu.get(registers.get16i(PC, 1)), u8.into()]).into()), p2)),
            (op, p1, FetchAddrU16(None, None)) => (false, (op, p1, FetchAddrU16(None, Some(mmu.get(registers.get16i(PC, 1)).into())))),
            (op, p1, FetchAddrU16(None, Some(u8))) => (true, (op, p1, AddrU16(u16::from_be_bytes([mmu.get(registers.get16i(PC, 1)), u8.into()]).into()))),
            (op, p1, p2) => (true, (op, p1, p2)),
        }
    }
}