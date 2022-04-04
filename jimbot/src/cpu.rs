/*use std::fmt::Error;
use std::mem;
use std::sync::{Arc, Mutex};
use R16::PC;
use crate::cpu::CycleP::{AddrReg16, CC, I8};
use crate::cpu::fflag::FFlag;
use crate::cpu::instruction::Op::{Ld, UnfetchOp, Xor};
use crate::cpu::instruction::P::{AddrReg16d, Reg16, Reg8, U16, UnfetchP, UnfetchU16};
use crate::cpu::registers::{R16, R8, Registers};
use crate::cpu::registers::R16::{AF, BC, DE, HL, SP};
use crate::cpu::registers::R8::{A, B, C, D, E, F, H, L, P, PCh, PCl, S};
use crate::mmu::MMU;

pub mod registers;
pub mod fflag;
mod instruction;

// Runs at 4.194304 MHz/ 4_194_304 Cycle per second
// if we wanna run at 60 fps then per fps must run 69905.07 cycles
pub struct CPU {
    ime: bool,
    registers: Registers,
    cycle_instruction: CycleInstruction,
}

impl Default for CPU {
    fn default() -> Self {
        let mut registers = Registers::default();
        Self {
            ime: false,
            cycle_instruction: CycleInstruction {
                op: CycleOp::Fetch(registers.get16i(PC, 1)),
                p1: CycleP::Non,
                p2: CycleP::Non,
                next: None,
            },
            registers,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CycleOp {
    Nop,
    Fetch(u16),
    FetchCB(u16),
    Read,
    Dec,
    Adc,
    Or,
    Jp,
    Call,
    Cp,
    Sub,
    And,
    Add,
    Xor,
    Internal,
    Jr,
    Inc,
    Write,
    Ret,
}

#[derive(Debug, Copy, Clone)]
pub enum Condition { Z, NZ, C, NC }

#[derive(Debug, Copy, Clone)]
pub enum CycleP {
    Reg8(R8),
    InaddrReg8(R8),
    Reg16(R16),
    AddrReg16d(R16),
    AddrRegd16(R16),
    AddrReg16i(R16),
    AddrReg16(R16),
    UnfetchAddrU16(Option<u8>, Option<u8>),
    AddrU16(u16),
    CC(Condition),
    I8(i8),
    UnfetchI8,
    U16(Option<u16>),
    UnfetchU16(Option<u8>, Option<u8>),
    UnfetchU8,
    InaddrUnfetchU8,
    U8(u8),
    InaddrU8(u8),
    Non,
}

#[derive(Debug)]
pub struct CycleInstruction {
    pub op: CycleOp,
    pub p1: CycleP,
    pub p2: CycleP,
    pub next: Option<Box<CycleInstruction>>,
}


impl CycleInstruction {
    fn fetch_cb(pc: u16) -> Self {
        CycleInstruction {
            op: CycleOp::FetchCB(pc),
            p1: CycleP::Non,
            p2: CycleP::Non,
            next: None,
        }
    }

    fn ret_c(c: Condition) -> Self {
        Self::internal(
            Some(Box::new(
                Self {
                    op: CycleOp::Ret,
                    p1: CC(c),
                    p2: CycleP::Non,
                    next: None,
                }
            ))
        )
    }

    fn internal(next: Option<Box<Self>>) -> Self {
        CycleInstruction {
            op: CycleOp::Internal,
            p1: CycleP::Non,
            p2: CycleP::Non,
            next,
        }
    }

    fn pop_u16(r81: R8, r82: R8) -> Self {
        Self::pop_u16_next(r81, r82, None)
    }

    fn pop_u16_next(r81: R8, r82: R8, next: Option<Box<Self>>) -> Self {
        CycleInstruction {
            op: CycleOp::Read,
            p1: CycleP::Reg8(r82),
            p2: CycleP::AddrReg16i(SP),
            next: Some(Box::new(
                CycleInstruction {
                    op: CycleOp::Read,
                    p1: CycleP::Reg8(r81),
                    p2: CycleP::AddrReg16i(SP),
                    next,
                }
            )),
        }
    }

    fn ret() -> Self {
        CycleInstruction::pop_u16_next(PCh, PCl, Some(Box::new(CycleInstruction::internal(None))))
    }

    fn push_u16(u16: u16) -> Self {
        let upper = ((u16 >> 8) & 0xFF) as u8;
        let lower = (u16 & 0xFF) as u8;
        CycleInstruction {
            op: CycleOp::Write,
            p1: CycleP::AddrRegd16(SP),
            p2: CycleP::U8(upper),
            next: Some(Box::new(
                CycleInstruction {
                    op: CycleOp::Write,
                    p1: CycleP::AddrRegd16(SP),
                    p2: CycleP::U8(lower),
                    next: None,
                }
            )),
        }
    }

    fn ld_addrreg16_reg8(r16: R16, r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Write,
            p1: CycleP::AddrReg16(r16),
            p2: CycleP::Reg8(r8),
            next: None,
        }
    }

    fn ld_addrreg16d_reg8(r16: R16, r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Write,
            p1: CycleP::AddrReg16d(r16),
            p2: CycleP::Reg8(r8),
            next: None,
        }
    }

    fn ld_addrreg16i_reg8(r16: R16, r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Write,
            p1: CycleP::AddrReg16i(r16),
            p2: CycleP::Reg8(r8),
            next: None,
        }
    }

    fn ld_reg8_addrreg16(r8: R8, r16: R16) -> Self {
        CycleInstruction {
            op: CycleOp::Read,
            p1: CycleP::Reg8(r8),
            p2: CycleP::AddrReg16(r16),
            next: None,
        }
    }

    fn ld_addrreg16_unfetchu8(r16: R16) -> Self {
        CycleInstruction {
            op: CycleOp::Read,
            p1: CycleP::AddrReg16(r16),
            p2: CycleP::UnfetchU8,
            next: None,
        }
    }

    fn dec_addrreg16(r16: R16) -> Self {
        CycleInstruction {
            op: CycleOp::Dec,
            p1: CycleP::AddrReg16(r16),
            p2: CycleP::Non,
            next: None,
        }
    }

    fn inc_addrreg16(r16: R16) -> Self {
        CycleInstruction {
            op: CycleOp::Inc,
            p1: CycleP::AddrReg16(r16),
            p2: CycleP::Non,
            next: None,
        }
    }

    fn dec_reg16(r16: R16) -> Self {
        CycleInstruction {
            op: CycleOp::Dec,
            p1: CycleP::Reg16(r16),
            p2: CycleP::Non,
            next: None,
        }
    }

    fn adc_reg8_addrreg16(r8: R8, r16: R16) -> Self {
        CycleInstruction {
            op: CycleOp::Adc,
            p1: CycleP::Reg8(r8),
            p2: CycleP::AddrReg16(r16),
            next: None,
        }
    }

    fn and_reg8_addrreg16(r8: R8, r16: R16) -> Self {
        CycleInstruction {
            op: CycleOp::And,
            p1: CycleP::Reg8(r8),
            p2: CycleP::AddrReg16(r16),
            next: None,
        }
    }

    fn or_reg8_addrreg16(r8: R8, r16: R16) -> Self {
        CycleInstruction {
            op: CycleOp::Or,
            p1: CycleP::Reg8(r8),
            p2: CycleP::AddrReg16(r16),
            next: None,
        }
    }

    fn ld_reg8_addru16(r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Read,
            p1: CycleP::Reg8(r8),
            p2: CycleP::UnfetchAddrU16(None, None),
            next: None,
        }
    }

    fn ld_reg8_addrreg16i(r8: R8, r16: R16) -> Self {
        CycleInstruction {
            op: CycleOp::Read,
            p1: CycleP::Reg8(r8),
            p2: CycleP::AddrReg16i(r16),
            next: None,
        }
    }

    fn jr_c_i8(c: Condition) -> Self {
        CycleInstruction {
            op: CycleOp::Jr,
            p1: CycleP::CC(c),
            p2: CycleP::UnfetchI8,
            next: None,
        }
    }

    fn jr_i8() -> Self {
        CycleInstruction {
            op: CycleOp::Jr,
            p1: CycleP::UnfetchI8,
            p2: CycleP::Non,
            next: None,
        }
    }

    fn jp_u16() -> Self {
        CycleInstruction {
            op: CycleOp::Jp,
            p1: CycleP::UnfetchU16(None, None),
            p2: CycleP::Non,
            next: None,
        }
    }

    fn jp_c_u16(c: Condition) -> Self {
        CycleInstruction {
            op: CycleOp::Jp,
            p1: CycleP::CC(c),
            p2: CycleP::UnfetchU16(None, None),
            next: None,
        }
    }

    fn call_unfetch_u16() -> Self {
        CycleInstruction {
            op: CycleOp::Call,
            p1: CycleP::UnfetchU16(None, None),
            p2: CycleP::Non,
            next: None,
        }
    }

    fn call_cc_unfetch_u16(c: Condition) -> Self {
        CycleInstruction {
            op: CycleOp::Call,
            p1: CycleP::CC(c),
            p2: CycleP::UnfetchU16(None, None),
            next: None,
        }
    }

    fn call_u16(u16: u16) -> Self {
        CycleInstruction {
            op: CycleOp::Call,
            p1: CycleP::U16(Some(u16)),
            p2: CycleP::Non,
            next: None,
        }
    }

    fn cp_reg8_u8(r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Cp,
            p1: CycleP::Reg8(r8),
            p2: CycleP::UnfetchU8,
            next: None,
        }
    }

    fn sub_reg8_unfetchu8(r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Sub,
            p1: CycleP::Reg8(r8),
            p2: CycleP::UnfetchU8,
            next: None,
        }
    }

    fn and_reg8_unfetchu8(r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::And,
            p1: CycleP::Reg8(r8),
            p2: CycleP::UnfetchU8,
            next: None,
        }
    }

    fn xor_reg8_unfetchu8(r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Xor,
            p1: CycleP::Reg8(r8),
            p2: CycleP::UnfetchU8,
            next: None,
        }
    }

    fn or_reg8_unfetchu8(r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Or,
            p1: CycleP::Reg8(r8),
            p2: CycleP::UnfetchU8,
            next: None,
        }
    }

    fn ld_reg8_unfetchu8(r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Read,
            p1: CycleP::Reg8(r8),
            p2: CycleP::UnfetchU8,
            next: None,
        }
    }

    fn cp_reg8_addru16(r8: R8, u16: u16) -> Self {
        CycleInstruction {
            op: CycleOp::Cp,
            p1: CycleP::Reg8(r8),
            p2: CycleP::AddrU16(u16),
            next: None,
        }
    }

    fn add_reg8_addru16(r8: R8, u16: u16) -> Self {
        CycleInstruction {
            op: CycleOp::Add,
            p1: CycleP::Reg8(r8),
            p2: CycleP::AddrU16(u16),
            next: None,
        }
    }

    fn xor_reg8_addru16(r8: R8, u16: u16) -> Self {
        CycleInstruction {
            op: CycleOp::Xor,
            p1: CycleP::Reg8(r8),
            p2: CycleP::AddrU16(u16),
            next: None,
        }
    }

    fn add_reg8_unfetchu8(r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Add,
            p1: CycleP::Reg8(r8),
            p2: CycleP::UnfetchU8,
            next: None,
        }
    }

    fn adc_reg8_unfetchu8(r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Adc,
            p1: CycleP::Reg8(r8),
            p2: CycleP::UnfetchU8,
            next: None,
        }
    }

    fn inc_r16(r16: R16) -> Self {
        CycleInstruction {
            op: CycleOp::Inc,
            p1: CycleP::Reg16(r16),
            p2: CycleP::Non,
            next: None,
        }
    }

    fn ld_reg16_u16(r81: R8, r82: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Read,
            p1: CycleP::Reg8(r82),
            p2: CycleP::UnfetchU8,
            next: Some(Box::new(CycleInstruction {
                op: CycleOp::Read,
                p1: CycleP::Reg8(r81),
                p2: CycleP::UnfetchU8,
                next: None,
            })),
        }
    }

    fn ld_inaddrreg8_reg8(r81: R8, r82: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Write,
            p1: CycleP::InaddrReg8(r81),
            p2: CycleP::Reg8(r82),
            next: None,
        }
    }

    fn ld_reg8_inaddru8(r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Write,
            p1: CycleP::Reg8(A),
            p2: CycleP::InaddrUnfetchU8,
            next: None,
        }
    }

    fn ld_inaddru8_reg8(r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Write,
            p1: CycleP::InaddrUnfetchU8,
            p2: CycleP::Reg8(r8),
            next: None,
        }
    }

    fn read_r8_u8(r8: R8) -> Self {
        CycleInstruction {
            op: CycleOp::Read,
            p1: CycleP::Reg8(r8),
            p2: CycleP::UnfetchU8,
            next: None,
        }
    }

    fn write_r16_u16(r16: R16, val: u16) -> Self {
        CycleInstruction {
            op: CycleOp::Write,
            p1: CycleP::Reg16(r16),
            p2: CycleP::U16(Some(val)),
            next: None,
        }
    }

    fn write_addrreg16_u8(r16: R16, val: u8) -> Self {
        CycleInstruction {
            op: CycleOp::Write,
            p1: CycleP::AddrReg16(r16),
            p2: CycleP::U8(val),
            next: None,
        }
    }

    fn write_r8_u8(r8: R8, val: u8) -> Self {
        CycleInstruction {
            op: CycleOp::Write,
            p1: CycleP::Reg8(r8),
            p2: CycleP::U8(val),
            next: None,
        }
    }
}

impl CPU {
    pub fn cycle(&mut self, mmu: &mut MMU) -> Result<(), String> {
        // if self.registers.get8(F) & 0x0F != 0{
        //     panic!("AF: {:#04X}", self.registers.get8(F));
        // }
        // if let CycleOp::Fetch(0xc0cf) = self.cycle_instruction.op { return Err(format!("{:06X}", self.registers.get16(PC))) };
        if let CycleOp::Fetch(_) = self.cycle_instruction.op {
            let mut interrupt_flags = mmu.get_interrupt_flags();
            let mut interrupt_enables = mmu.get_interrupt_enables();
            if let Some(request) = interrupt_flags.get_request_by_priority() {
                if interrupt_enables.is_enable(request) {
                    interrupt_flags.disable_request(request);
                    interrupt_enables.disable_request(request);
                    mmu.set_interrupt_flags(interrupt_flags);
                    mmu.set_interrupt_enables(interrupt_enables);
                    let location = request.routine_location();
                    self.cycle_instruction = CycleInstruction {
                        op: CycleOp::Nop,
                        p1: CycleP::Non,
                        p2: CycleP::Non,
                        next: Some(Box::new(
                            CycleInstruction {
                                op: CycleOp::Nop,
                                p1: CycleP::Non,
                                p2: CycleP::Non,
                                next: Some(Box::new(CycleInstruction::call_u16(location))),
                            }
                        )),
                    }
                }
            }
        }
        let next_cycle_instruction = match self.cycle_instruction {
            CycleInstruction { op: CycleOp::Nop, p1: _, p2: _, next: _ } => Ok(None),
            CycleInstruction { op: CycleOp::Internal, p1: _, p2: _, next: _ } => Ok(None),
            CycleInstruction { op: CycleOp::Fetch(u16), p1: _, p2: _, next: _ } => self.fetch(u16, mmu),
            CycleInstruction { op: CycleOp::FetchCB(u16), p1: _, p2: _, next: _ } => self.fetch_cb(u16, mmu),
            CycleInstruction { op, p1, p2: CycleP::UnfetchU16(None, None), next: _ } => {
                Ok(Some(
                    CycleInstruction {
                        op,
                        p1,
                        p2: CycleP::UnfetchU16(Some(mmu.get(self.registers.get16i(PC, 1))), None),
                        next: mem::replace(&mut self.cycle_instruction.next, None),
                    }
                ))
            }
            CycleInstruction { op, p1, p2: CycleP::UnfetchU16(Some(u16l), None), next: _ } => {
                Ok(Some(
                    CycleInstruction {
                        op,
                        p1,
                        p2: CycleP::U16(Some(u16::from_ne_bytes([u16l, mmu.get(self.registers.get16i(PC, 1))]))),
                        next: mem::replace(&mut self.cycle_instruction.next, None),
                    }
                ))
            }
            CycleInstruction { op, p1: CycleP::UnfetchU16(None, None), p2, next: _ } => {
                Ok(Some(
                    CycleInstruction {
                        op,
                        p1: CycleP::UnfetchU16(Some(mmu.get(self.registers.get16i(PC, 1))), None),
                        p2,
                        next: mem::replace(&mut self.cycle_instruction.next, None),
                    }
                ))
            }
            CycleInstruction { op, p1: CycleP::UnfetchU16(Some(u16l), None), p2, next: _ } => {
                Ok(Some(
                    CycleInstruction {
                        op,
                        p1: CycleP::U16(Some(u16::from_ne_bytes([u16l, mmu.get(self.registers.get16i(PC, 1))]))),
                        p2,
                        next: mem::replace(&mut self.cycle_instruction.next, None),
                    }
                ))
            }
            CycleInstruction { op, p1: CycleP::UnfetchAddrU16(None, None), p2, next: _ } => {
                Ok(Some(
                    CycleInstruction {
                        op,
                        p1: CycleP::UnfetchAddrU16(Some(mmu.get(self.registers.get16i(PC, 1))), None),
                        p2,
                        next: mem::replace(&mut self.cycle_instruction.next, None),
                    }
                ))
            }
            CycleInstruction { op, p1: CycleP::UnfetchAddrU16(Some(u16l), None), p2, next: _ } => {
                Ok(Some(
                    CycleInstruction {
                        op,
                        p1: CycleP::AddrU16(u16::from_ne_bytes([u16l, mmu.get(self.registers.get16i(PC, 1))])),
                        p2,
                        next: mem::replace(&mut self.cycle_instruction.next, None),
                    }
                ))
            }
            CycleInstruction { op, p1, p2: CycleP::UnfetchAddrU16(None, None), next: _ } => {
                Ok(Some(
                    CycleInstruction {
                        op,
                        p1,
                        p2: CycleP::UnfetchAddrU16(Some(mmu.get(self.registers.get16i(PC, 1))), None),
                        next: mem::replace(&mut self.cycle_instruction.next, None),
                    }
                ))
            }
            CycleInstruction { op, p1, p2: CycleP::UnfetchAddrU16(Some(u16l), None), next: _ } => {
                Ok(Some(
                    CycleInstruction {
                        op,
                        p1,
                        p2: CycleP::AddrU16(u16::from_ne_bytes([u16l, mmu.get(self.registers.get16i(PC, 1))])),
                        next: mem::replace(&mut self.cycle_instruction.next, None),
                    }
                ))
            }
            CycleInstruction { op: CycleOp::Read, p1: CycleP::Reg8(r8), p2: CycleP::UnfetchU8, next: _ } => {
                let pc = self.registers.get16i(R16::PC, 1);
                self.registers.set8(r8, mmu.get(pc));
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Read, p1: CycleP::AddrReg16(r16), p2: CycleP::UnfetchU8, next: _ } => {
                let pc = self.registers.get16i(R16::PC, 1);
                let u8 = mmu.get(pc);
                Ok(Some(CycleInstruction::write_addrreg16_u8(HL, u8)))
            }
            CycleInstruction { op: CycleOp::Read, p1: CycleP::Reg8(r8), p2: CycleP::AddrU16(u16), next: _ } => {
                self.registers.set8(r8, mmu.get(u16));
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Cp, p1: CycleP::Reg8(r8), p2: CycleP::UnfetchU8, next: _ } => {
                let pc = self.registers.get16i(R16::PC, 1);
                let u8 = mmu.get(pc);
                self.cp_reg8u8(r8, u8);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Add, p1: CycleP::Reg8(r8), p2: CycleP::UnfetchU8, next: _ } => {
                let pc = self.registers.get16i(R16::PC, 1);
                let u8 = mmu.get(pc);
                self.add_reg8u8(r8, u8);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Adc, p1: CycleP::Reg8(r8), p2: CycleP::UnfetchU8, next: _ } => {
                let pc = self.registers.get16i(R16::PC, 1);
                let u8 = mmu.get(pc);
                self.adc_reg8u8(r8, u8);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Or, p1: CycleP::Reg8(r8), p2: CycleP::UnfetchU8, next: _ } => {
                let pc = self.registers.get16i(R16::PC, 1);
                let u8 = mmu.get(pc);
                self.or_reg8u8(r8, u8);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Sub, p1: CycleP::Reg8(r8), p2: CycleP::UnfetchU8, next: _ } => {
                let pc = self.registers.get16i(R16::PC, 1);
                let u8 = mmu.get(pc);
                self.sub_reg8u8(r8, u8);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::And, p1: CycleP::Reg8(r8), p2: CycleP::UnfetchU8, next: _ } => {
                let pc = self.registers.get16i(R16::PC, 1);
                let u8 = mmu.get(pc);
                self.and_reg8u8(r8, u8);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Xor, p1: CycleP::Reg8(r8), p2: CycleP::UnfetchU8, next: _ } => {
                let pc = self.registers.get16i(R16::PC, 1);
                let u8 = mmu.get(pc);
                self.xor_reg8u8(r8, u8);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Cp, p1: CycleP::Reg8(r8), p2: CycleP::AddrU16(u16), next: _ } => {
                let val = mmu.get(u16);
                self.cp_reg8u8(r8, val);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Xor, p1: CycleP::Reg8(r8), p2: CycleP::AddrU16(u16), next: _ } => {
                let val = mmu.get(u16);
                self.xor_reg8u8(r8, val);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Add, p1: CycleP::Reg8(r8), p2: CycleP::AddrU16(u16), next: _ } => {
                let val = mmu.get(u16);
                self.add_reg8u8(r8, val);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Inc, p1: CycleP::Reg16(r16), p2: _, next: _ } => {
                self.registers.inc16(r16, 1);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Call, p1: CycleP::U16(Some(u16)), p2: CycleP::Non, next: _ } => {
                let pc = self.registers.get16(PC);
                self.registers.set16(PC, u16);
                Ok(Some(CycleInstruction::push_u16(pc)))
            }
            CycleInstruction { op: CycleOp::Call, p1: CycleP::CC(c), p2: CycleP::U16(Some(u16)), next: _ } => {
                if self.check_cc(c) {
                    let pc = self.registers.get16(PC);
                    self.registers.set16(PC, u16);
                    Ok(Some(CycleInstruction::push_u16(pc)))
                } else {
                    Ok(None)
                }
            }
            CycleInstruction { op: CycleOp::Ret, p1: CycleP::CC(c), p2: CycleP::Non, next: _ } => {
                if self.check_cc(c) {
                    Ok(Some(CycleInstruction::ret()))
                } else {
                    Ok(None)
                }
            }
            CycleInstruction { op: CycleOp::Jp, p1: CycleP::CC(c), p2: CycleP::U16(Some(u16)), next: _ } => {
                if self.check_cc(c) {
                    Ok(Some(CycleInstruction::write_r16_u16(PC, u16)))
                } else {
                    Ok(None)
                }
            }
            CycleInstruction { op: CycleOp::Jp, p1: CycleP::U16(Some(u16)), p2: CycleP::Non, next: _ } => {
                Ok(Some(CycleInstruction::write_r16_u16(PC, u16)))
            }
            CycleInstruction { op, p1: CycleP::UnfetchU8, p2, next: _ } => {
                Ok(Some(
                    CycleInstruction {
                        op,
                        p1: CycleP::U8(mmu.get(self.registers.get16i(PC, 1))),
                        p2,
                        next: mem::replace(&mut self.cycle_instruction.next, None),
                    }
                ))
            }
            CycleInstruction { op, p1: CycleP::InaddrUnfetchU8, p2, next: _ } => {
                Ok(Some(
                    CycleInstruction {
                        op,
                        p1: CycleP::InaddrU8(mmu.get(self.registers.get16i(PC, 1))),
                        p2,
                        next: mem::replace(&mut self.cycle_instruction.next, None),
                    }
                ))
            }
            CycleInstruction { op, p1, p2: CycleP::InaddrUnfetchU8, next: _ } => {
                Ok(Some(
                    CycleInstruction {
                        op,
                        p1,
                        p2: CycleP::InaddrU8(mmu.get(self.registers.get16i(PC, 1))),
                        next: mem::replace(&mut self.cycle_instruction.next, None),
                    }
                ))
            }
            CycleInstruction { op: CycleOp::Write, p1: CycleP::InaddrU8(u8), p2: CycleP::Reg8(r8), next: _ } => {
                let address = 0xFF00 + u8 as u16;
                mmu.set(address, self.registers.get8(r8));
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Write, p1: CycleP::Reg8(r8), p2: CycleP::InaddrU8(u8), next: _ } => {
                let address = 0xFF00 + u8 as u16;
                self.registers.set8(r8, mmu.get(address));
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Jr, p1: CC(c), p2: CycleP::UnfetchI8, next: _ } => {
                let i8 = mmu.get(self.registers.get16i(PC, 1)) as i8;
                if self.check_cc(c) {
                    let pc = self.registers.get16(PC);
                    let next_pc = (((pc as u32 as i32) + (i8 as i32)) as u16);
                    Ok(Some(CycleInstruction::write_r16_u16(PC, next_pc)))
                } else {
                    Ok(None)
                }
            }
            CycleInstruction { op: CycleOp::Jr, p1: CycleP::UnfetchI8, p2: CycleP::Non, next: _ } => {
                let i8 = mmu.get(self.registers.get16i(PC, 1)) as i8;
                let pc = self.registers.get16(PC);
                let next_pc = (((pc as u32 as i32) + (i8 as i32)) as u16);
                Ok(Some(CycleInstruction::write_r16_u16(PC, next_pc)))
            }
            CycleInstruction { op: CycleOp::Write, p1: CycleP::Reg8(r8), p2: CycleP::U8(u8), next: _ } => {
                self.registers.set8(r8, u8);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Write, p1: CycleP::Reg16(r16), p2: CycleP::U16(Some(u16)), next: _ } => {
                self.registers.set16(r16, u16);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Write, p1: CycleP::InaddrReg8(r81), p2: CycleP::Reg8(r82), next: _ } => {
                let address = 0xFF00 + self.registers.get8(r81) as u16;
                mmu.set(address, self.registers().get8(r82));
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Read, p1: CycleP::Reg8(r8), p2: CycleP::AddrReg16(r16), next: _ } => {
                let address = self.registers.get16(r16);
                self.registers.set8(r8, mmu.get(address));
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Adc, p1: CycleP::Reg8(r8), p2: CycleP::AddrReg16(r16), next: _ } => {
                let address = self.registers.get16(r16);
                self.adc_reg8u8(r8, mmu.get(address));
                Ok(None)
            }
            CycleInstruction { op: CycleOp::And, p1: CycleP::Reg8(r8), p2: CycleP::AddrReg16(r16), next: _ } => {
                let address = self.registers.get16(r16);
                self.and_reg8u8(r8, mmu.get(address));
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Or, p1: CycleP::Reg8(r8), p2: CycleP::AddrReg16(r16), next: _ } => {
                let address = self.registers.get16(r16);
                self.or_reg8u8(r8, mmu.get(address));
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Read, p1: CycleP::Reg8(r8), p2: CycleP::AddrReg16i(r16), next: _ } => {
                let address = self.registers.get16i(r16, 1);
                let val = mmu.get(address);
                self.registers.set8(r8, val);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Write, p1: CycleP::AddrReg16d(r16), p2: CycleP::Reg8(r8), next: _ } => {
                let addr = self.registers.get16d(r16, 1);
                let val = self.registers.get8(r8);
                mmu.set(addr, val);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Write, p1: CycleP::AddrReg16i(r16), p2: CycleP::Reg8(r8), next: _ } => {
                let addr = self.registers.get16i(r16, 1);
                let val = self.registers.get8(r8);
                mmu.set(addr, val);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Write, p1: CycleP::AddrReg16(r16), p2: CycleP::Reg8(r8), next: _ } => {
                let addr = self.registers.get16(r16);
                let val = self.registers.get8(r8);
                mmu.set(addr, val);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Dec, p1: CycleP::AddrReg16(r16), p2: CycleP::Non, next: _ } => {
                let addr = self.registers.get16(r16);
                let val = mmu.get(addr);
                let res = self.dec_u8(val);
                Ok(Some(CycleInstruction::write_addrreg16_u8(r16, res)))
            }
            CycleInstruction { op: CycleOp::Inc, p1: CycleP::AddrReg16(r16), p2: CycleP::Non, next: _ } => {
                let addr = self.registers.get16(r16);
                let val = mmu.get(addr);
                let res = self.inc_u8(val);
                Ok(Some(CycleInstruction::write_addrreg16_u8(r16, res)))
            }
            CycleInstruction { op: CycleOp::Dec, p1: CycleP::Reg16(r16), p2: CycleP::Non, next: _ } => {
                self.registers.dec16(r16, 1);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Write, p1: CycleP::AddrRegd16(r16), p2: CycleP::U8(u8), next: _ } => {
                let addr = self.registers.getd16(r16, 1);
                mmu.set(addr, u8);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Write, p1: CycleP::AddrReg16(r16), p2: CycleP::U8(u8), next: _ } => {
                let addr = self.registers.get16(r16);
                mmu.set(addr, u8);
                Ok(None)
            }
            CycleInstruction { op: CycleOp::Write, p1: CycleP::AddrU16(addr), p2: CycleP::Reg8(r8), next: _ } => {
                mmu.set(addr, self.registers.get8(r8));
                Ok(None)
            }
            CycleInstruction { op, p1, p2, next: _ } => Err(format!("Unhandled cycle: {:?} {:?} {:?}", op, p1, p2)),
        };
        if next_cycle_instruction.is_ok() {
            if let Some(next_cycle_instruction) = next_cycle_instruction.unwrap() {
                self.cycle_instruction = next_cycle_instruction;
            } else if self.cycle_instruction.next.is_some() {
                let old = mem::replace(&mut self.cycle_instruction.next, None);
                self.cycle_instruction = *old.unwrap();
            } else {
                self.cycle_instruction = CycleInstruction {
                    op: CycleOp::Fetch(self.registers.get16i(PC, 1)),
                    p1: CycleP::Non,
                    p2: CycleP::Non,
                    next: None,
                };
            }
            Ok(())
        } else {
            Err(next_cycle_instruction.err().unwrap())
        }
    }

    fn fetch(&mut self, pc: u16, mmu: &mut MMU) -> Result<Option<CycleInstruction>, String> {
        let op = mmu.get(pc);
        let next_cycle_instruction = match op {
            0x00 => None,
            0x01 => Some(CycleInstruction::ld_reg16_u16(B, C)),
            0x03 => Some(CycleInstruction::inc_r16(BC)),
            0x13 => Some(CycleInstruction::inc_r16(DE)),
            0x23 => Some(CycleInstruction::inc_r16(HL)),
            0x27 => {
                self.daa();
                None
            }
            0x2F => {
                self.cpl();
                None
            },
            0x33 => Some(CycleInstruction::inc_r16(SP)),
            0x05 => {
                self.dec_r8(B);
                None
            }
            0x04 => {
                self.inc_r8(B);
                None
            }
            0x14 => {
                self.inc_r8(D);
                None
            }
            0x24 => {
                self.inc_r8(H);
                None
            }
            0x34 => Some(CycleInstruction::inc_addrreg16(HL)),
            0x06 => Some(CycleInstruction::read_r8_u8(B)),
            0x09 => {
                self.add_reg16reg16(HL, BC);
                Some(CycleInstruction::internal(None))
            },
            0x19 => {
                self.add_reg16reg16(HL, DE);
                Some(CycleInstruction::internal(None))
            },
            0x29 => {
                self.add_reg16reg16(HL, HL);
                Some(CycleInstruction::internal(None))
            },
            0x39 => {
                self.add_reg16reg16(HL, SP);
                Some(CycleInstruction::internal(None))
            },
            0x0B => Some(CycleInstruction::dec_reg16(BC)),
            0x1B => Some(CycleInstruction::dec_reg16(DE)),
            0x2B => Some(CycleInstruction::dec_reg16(HL)),
            0x3B => Some(CycleInstruction::dec_reg16(SP)),
            0x0C => {
                self.inc_r8(C);
                None
            }
            0x1C => {
                self.inc_r8(E);
                None
            }
            0x2C => {
                self.inc_r8(L);
                None
            }
            0x3C => {
                self.inc_r8(A);
                None
            }
            0x0D => {
                self.dec_r8(C);
                None
            }
            0x1D => {
                self.dec_r8(E);
                None
            }
            0x2D => {
                self.dec_r8(L);
                None
            }
            0x3D => {
                self.dec_r8(A);
                None
            }
            0x0E => Some(CycleInstruction::read_r8_u8(C)),
            0x2E => Some(CycleInstruction::read_r8_u8(L)),
            0x11 => Some(CycleInstruction::ld_reg16_u16(D, E)),
            0x12 => Some(CycleInstruction::ld_addrreg16_reg8(DE, A)),
            0x15 => {
                self.dec_r8(D);
                None
            }
            0x16 => Some(CycleInstruction::read_r8_u8(D)),
            0x17 => {
                self.rla();
                None
            }
            0x18 => Some(CycleInstruction::jr_i8()),
            0x1A => Some(CycleInstruction::ld_reg8_addrreg16(A, DE)),
            0x1E => Some(CycleInstruction::read_r8_u8(E)),
            0x1F => {
                self.rra();
                None
            }
            0x20 => Some(CycleInstruction::jr_c_i8(Condition::NZ)),
            0x21 => Some(CycleInstruction::ld_reg16_u16(H, L)),
            0x22 => Some(CycleInstruction::ld_addrreg16i_reg8(HL, A)),
            0x25 => {
                self.dec_r8(H);
                None
            }
            0x26 => Some(CycleInstruction::ld_reg8_unfetchu8(H)),
            0x28 => Some(CycleInstruction::jr_c_i8(Condition::Z)),
            0x2A => Some(CycleInstruction::ld_reg8_addrreg16i(A, HL)),
            0x30 => Some(CycleInstruction::jr_c_i8(Condition::NC)),
            0x31 => Some(CycleInstruction::ld_reg16_u16(S, P)),
            0x32 => Some(CycleInstruction::ld_addrreg16d_reg8(HL, A)),
            0x35 => Some(CycleInstruction::dec_addrreg16(HL)),
            0x36 => Some(CycleInstruction::ld_addrreg16_unfetchu8(HL)),
            0x3E => Some(CycleInstruction::read_r8_u8(A)),
            0x46 => Some(CycleInstruction::ld_reg8_addrreg16(B, HL)),
            0x47 => {
                self.registers.set8(B, self.registers.get8(A));
                None
            }
            0x4E => Some(CycleInstruction::ld_reg8_addrreg16(C, HL)),
            0x4F => {
                self.registers.set8(C, self.registers().get8(A));
                None
            }
            0x56 => Some(CycleInstruction::ld_reg8_addrreg16(D, HL)),
            0x57 => {
                let a = self.registers.get8(A);
                self.registers.set8(D, a);
                None
            }
            0x58 => {
                let val = self.registers.get8(B);
                self.registers.set8(E, val);
                None
            }
            0x59 => {
                let val = self.registers.get8(C);
                self.registers.set8(E, val);
                None
            }
            0x5A => {
                let val = self.registers.get8(D);
                self.registers.set8(E, val);
                None
            }
            0x5B => {
                let val = self.registers.get8(E);
                self.registers.set8(E, val);
                None
            }
            0x5C => {
                let val = self.registers.get8(H);
                self.registers.set8(E, val);
                None
            }
            0x5D => {
                let val = self.registers.get8(L);
                self.registers.set8(E, val);
                None
            }
            0x5E => Some(CycleInstruction::ld_reg8_addrreg16(E, HL)),
            0x5F => {
                let val = self.registers.get8(A);
                self.registers.set8(E, val);
                None
            }
            0x67 => {
                let a = self.registers.get8(A);
                self.registers.set8(H, a);
                None
            }
            0x68 => {
                self.registers.set8(L, self.registers.get8(B));
                None
            }
            0x69 => {
                self.registers.set8(L, self.registers.get8(C));
                None
            }
            0x6A => {
                self.registers.set8(L, self.registers.get8(D));
                None
            }
            0x6B => {
                self.registers.set8(L, self.registers.get8(E));
                None
            }
            0x6C => {
                self.registers.set8(L, self.registers.get8(H));
                None
            }
            0x6D => {
                self.registers.set8(L, self.registers.get8(L));
                None
            }
            0x6E => Some(CycleInstruction::ld_reg8_addrreg16(L, HL)),
            0x6F => {
                self.registers.set8(L, self.registers.get8(A));
                None
            }
            0x70 => Some(CycleInstruction::ld_addrreg16_reg8(HL, B)),
            0x71 => Some(CycleInstruction::ld_addrreg16_reg8(HL, C)),
            0x72 => Some(CycleInstruction::ld_addrreg16_reg8(HL, D)),
            0x73 => Some(CycleInstruction::ld_addrreg16_reg8(HL, E)),
            0x74 => Some(CycleInstruction::ld_addrreg16_reg8(HL, H)),
            0x75 => Some(CycleInstruction::ld_addrreg16_reg8(HL, L)),
            0x77 => Some(CycleInstruction::ld_addrreg16_reg8(HL, A)),
            0x78 => {
                self.registers.set8(A, self.registers.get8(B));
                None
            }
            0x79 => {
                self.registers.set8(A, self.registers.get8(C));
                None
            }
            0x7A => {
                self.registers.set8(A, self.registers.get8(D));
                None
            }
            0x7B => {
                let val = self.registers.get8(E);
                self.registers.set8(A, val);
                None
            }
            0x7C => {
                let val = self.registers.get8(H);
                self.registers.set8(A, val);
                None
            }
            0x7D => {
                let val = self.registers.get8(L);
                self.registers.set8(A, val);
                None
            }
            0x7E => Some(CycleInstruction::ld_reg8_addrreg16(A, HL)),
            0x7F => {
                let val = self.registers.get8(A);
                self.registers.set8(A, val);
                None
            }
            0x80 => {
                self.add_reg8reg8(A,B);
                None
            }
            0x81 => {
                self.add_reg8reg8(A,C);
                None
            }
            0x82 => {
                self.add_reg8reg8(A,D);
                None
            }
            0x83 => {
                self.add_reg8reg8(A,E);
                None
            }
            0x84 => {
                self.add_reg8reg8(A,H);
                None
            }
            0x85 => {
                self.add_reg8reg8(A,L);
                None
            }
            0x86 => Some(CycleInstruction::add_reg8_addru16(A, self.registers.get16(HL))),
            0x87 => {
                self.add_reg8reg8(A,A);
                None
            }
            0x88 => {
                self.adc_reg8reg8(A, B);
                None
            }
            0x89 => {
                self.adc_reg8reg8(A, C);
                None
            }
            0x8A => {
                self.adc_reg8reg8(A, D);
                None
            }
            0x8B => {
                self.adc_reg8reg8(A, E);
                None
            }
            0x8C => {
                self.adc_reg8reg8(A, H);
                None
            }
            0x8D => {
                self.adc_reg8reg8(A, L);
                None
            }
            0x8E => Some(CycleInstruction::adc_reg8_addrreg16(A, HL)),
            0x8F => {
                self.adc_reg8reg8(A, A);
                None
            }
            0x90 => {
                self.sub_reg8reg8(A, B);
                None
            }
            0xA0 => {
                self.and_reg8reg8(A, B);
                None
            }
            0xA1 => {
                self.and_reg8reg8(A, C);
                None
            }
            0xA2 => {
                self.and_reg8reg8(A, D);
                None
            }
            0xA3 => {
                self.and_reg8reg8(A, E);
                None
            }
            0xA4 => {
                self.and_reg8reg8(A, H);
                None
            }
            0xA5 => {
                self.and_reg8reg8(A, L);
                None
            }
            0xA6 => Some(CycleInstruction::and_reg8_addrreg16(A, HL)),
            0xA7 => {
                self.and_reg8reg8(A, A);
                None
            }
            0xA8 => {
                self.xor_reg8reg8(A, B);
                None
            }
            0xA9 => {
                self.xor_reg8reg8(A, C);
                None
            }
            0xAA => {
                self.xor_reg8reg8(A, D);
                None
            }
            0xAB => {
                self.xor_reg8reg8(A, E);
                None
            }
            0xAC => {
                self.xor_reg8reg8(A, H);
                None
            }
            0xAD => {
                self.xor_reg8reg8(A, L);
                None
            }
            0xAE => Some(CycleInstruction::xor_reg8_addru16(A, self.registers.get16(HL))),
            0xAF => {
                self.xor(self.registers.get8(R8::A), self.registers.get8(R8::A));
                None
            }
            0xB0 => {
                self.or_reg8reg8(A, B);
                None
            }
            0xB1 => {
                self.or_reg8reg8(A, C);
                None
            }
            0xB2 => {
                self.or_reg8reg8(A, D);
                None
            }
            0xB3 => {
                self.or_reg8reg8(A, E);
                None
            }
            0xB4 => {
                self.or_reg8reg8(A, H);
                None
            }
            0xB5 => {
                self.or_reg8reg8(A, L);
                None
            }
            0xB6 => Some(CycleInstruction::adc_reg8_addrreg16(A, HL)),
            0xB7 => {
                self.or_reg8reg8(A, A);
                None
            }
            0xB8 => {
                self.cp_reg8reg8(A, B);
                None
            }
            0xB9 => {
                self.cp_reg8reg8(A, C);
                None
            }
            0xBA => {
                self.cp_reg8reg8(A, D);
                None
            }
            0xBB => {
                self.cp_reg8reg8(A, E);
                None
            }
            0xBC => {
                self.cp_reg8reg8(A, H);
                None
            }
            0xBD => {
                self.cp_reg8reg8(A, L);
                None
            }
            0xBE => Some(CycleInstruction::cp_reg8_addru16(A, self.registers.get16(HL))),
            0xBF => {
                self.cp_reg8reg8(A, A);
                None
            }
            0xC0 => Some(CycleInstruction::ret_c(Condition::NZ)),
            0xC1 => Some(CycleInstruction::pop_u16(B, C)),
            0xC2 => Some(CycleInstruction::jp_c_u16(Condition::NZ)),
            0xC3 => Some(CycleInstruction::jp_u16()),
            0xC4 => Some(CycleInstruction::call_cc_unfetch_u16(Condition::NZ)),
            0xC5 => {
                Some(
                    CycleInstruction::internal(
                        Some(Box::new(CycleInstruction::push_u16(self.registers.get16(BC))))
                    )
                )
            }
            0xC6 => Some(CycleInstruction::add_reg8_unfetchu8(A)),
            0xC8 => Some(CycleInstruction::ret_c(Condition::Z)),
            0xC9 => Some(CycleInstruction::ret()),
            0xCA => Some(CycleInstruction::jp_c_u16(Condition::Z)),
            0xCB => Some(CycleInstruction::fetch_cb(self.registers.get16i(PC, 1))),
            0xCD => Some(CycleInstruction::call_unfetch_u16()),
            0xCE => Some(CycleInstruction::adc_reg8_unfetchu8(A)),
            0xD0 => Some(CycleInstruction::ret_c(Condition::NC)),
            0xD1 => Some(CycleInstruction::pop_u16(D, E)),
            0xD2 => Some(CycleInstruction::jp_c_u16(Condition::NC)),
            0xD5 => Some(CycleInstruction::push_u16(u16::from_be_bytes([self.registers.get8(D), self.registers.get8(E)]))),
            0xD6 => Some(CycleInstruction::sub_reg8_unfetchu8(A)),
            0xD8 => Some(CycleInstruction::ret_c(Condition::C)),
            0xDA => Some(CycleInstruction::jp_c_u16(Condition::C)),
            0xFA => Some(CycleInstruction::ld_reg8_addru16(A)),
            0xFE => Some(CycleInstruction::cp_reg8_u8(A)),
            0xF0 => Some(CycleInstruction::ld_reg8_inaddru8(A)),
            0xE0 => Some(CycleInstruction::ld_inaddru8_reg8(A)),
            0xE1 => Some(CycleInstruction::pop_u16(H, L)),
            0xE2 => Some(CycleInstruction::ld_inaddrreg8_reg8(C, A)),
            0xE5 => Some(CycleInstruction::push_u16(u16::from_be_bytes([self.registers.get8(H), self.registers.get8(L)]))),
            0xE6 => Some(CycleInstruction::and_reg8_unfetchu8(A)),
            0xE9 => {
                self.registers.set16(PC, self.registers.get16(HL));
                None
            },
            0xEA => {
                Some(
                    CycleInstruction {
                        op: CycleOp::Write,
                        p1: CycleP::UnfetchAddrU16(None, None),
                        p2: CycleP::Reg8(A),
                        next: None,
                    }
                )
            }
            0xEE => Some(CycleInstruction::xor_reg8_unfetchu8(A)),
            0xF1 => Some(CycleInstruction::pop_u16(A, F)),
            0xF3 => {
                self.ime = false;
                None
            }
            0xF5 => Some(CycleInstruction::push_u16(u16::from_be_bytes([self.registers.get8(A), self.registers.get8(F)]))),
            0xF6 => Some(CycleInstruction::or_reg8_unfetchu8(A)),
            _ => return Err(format!("Unknown Instruction [{:#06X}] {:#04X}", pc, op)),
        };
        Ok(next_cycle_instruction)
    }

    fn fetch_cb(&mut self, pc: u16, mmu: &mut MMU) -> Result<Option<CycleInstruction>, String> {
        let op = mmu.get(pc);
        let next_cycle_instruction = match op {
            0x18 => {
                self.rr_reg8(B);
                None
            }
            0x19 => {
                self.rr_reg8(C);
                None
            }
            0x1A => {
                self.rr_reg8(D);
                None
            }
            0x1B => {
                self.rr_reg8(E);
                None
            }
            0x1C => {
                self.rr_reg8(H);
                None
            }
            0x1D => {
                self.rr_reg8(L);
                None
            }
            0x1E => {
                self.rr_addrreg16(HL, mmu);
                None
            }
            0x1F => {
                self.rr_reg8(A);
                None
            }
            0x38 => {
                self.srl_reg8(B);
                None
            }
            0x11 => {
                self.rl_reg8(C);
                None
            }
            0x7C => {
                self.bit(7, self.registers.get8(R8::H));
                None
            }
            _ => return Err(format!("Unknown Instruction [{:#06X}] 0xCB {:#04X}", pc, op)),
        };
        Ok(next_cycle_instruction)
    }

    fn daa(&mut self) {
        let a = self.registers.get8(A);
        let carry =  self.registers.get_f(FFlag::C);
        let half_carry =  self.registers.get_f(FFlag::H);
        let subtraction =  self.registers.get_f(FFlag::N);

        if !subtraction {
            let mut correction = 0;
            if half_carry || (a & 0xf > 0x9) { correction |= 0x6; }

            if carry || (a > 0x99) {
                correction |= 0x60;
                self.registers.set_f(FFlag::C, true);
            }

            self.registers.set8(A, a.wrapping_add(correction));
        } else if carry {
            self.registers.set_f(FFlag::C, true);
            self.registers.set8(A, a.wrapping_add(if half_carry { 0x9a } else { 0xa0 }));
        } else if half_carry {
            self.registers.set8(A, a.wrapping_add(0xFA));
        }
        self.registers.set_f(FFlag::Z, self.registers.get8(A) == 0);
        self.registers.set_f(FFlag::H, false);
    }

    fn cpl(&mut self) {
        let mut a = self.registers.get8(A);
        let a = !a;
        self.registers.set_f(FFlag::N, true);
        self.registers.set_f(FFlag::H, true);
        self.registers.set8(A, a);
    }

    fn adc_reg8reg8(&mut self, r81: R8, r82: R8) {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        let res = self.adc(r81_val, r82_val);
        self.registers.set8(r81, res);
    }

    fn adc_reg8u8(&mut self, r8: R8, u8: u8) {
        let r8_val = self.registers.get8(r8);
        let res = self.adc(r8_val, u8);
        self.registers.set8(r8, res);
    }

    fn adc(&mut self, u81: u8, u82: u8) -> u8 {
        let u161 = u81 as u16;
        let u162 = u82 as u16;
        let carry = self.registers.get_f(FFlag::C) as u16;
        let res = u161.wrapping_add(u162).wrapping_add(carry);

        self.registers.set_f(FFlag::Z, res & 0xFF == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, (u161 & 0x0F) + (u162 & 0x0F) + carry > 0x0F);
        self.registers.set_f(FFlag::C, res & 0xFF00 != 0);
        (res & 0xff) as u8
    }

    fn rra(&mut self) {
        let a = self.registers.get8(A);
        let res = (a >> 1) | if self.registers.get_f(FFlag::C) { 0x80 } else { 0 };
        self.registers.set_f(FFlag::Z, false);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, a & 1 == 1);
        self.registers.set8(A, res);
    }

    fn rr_addrreg16(&mut self, r16: R16, mmu: &mut MMU) {
        let reg16_val = self.registers.get16(r16);
        let addrreg16_val = mmu.get(reg16_val);
        let res = self.rr(addrreg16_val);
        mmu.set(reg16_val, res);
    }

    fn rr_reg8(&mut self, r8: R8) {
        let r8_val = self.registers.get8(r8);
        let res = self.rr(r8_val);
        self.registers.set8(r8, res);
    }

    fn rr(&mut self, u8: u8) -> u8 {
        let res = (u8 >> 1) | if self.registers.get_f(FFlag::C) { 0x80 } else { 0 };
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, u8 & 1 == 1);
        res
    }

    fn srl_addrreg16(&mut self, r16: R16, mmu: &mut MMU) {
        let reg16_val = self.registers.get16(r16);
        let addrreg16_val = mmu.get(reg16_val);
        let res = self.srl(addrreg16_val);
        mmu.set(reg16_val, res);
    }

    fn srl_reg8(&mut self, r8: R8) {
        let r8_val = self.registers.get8(r8);
        let res = self.srl(r8_val);
        self.registers.set8(r8, res);
    }

    fn srl(&mut self, u8: u8) -> u8 {
        let res = u8 >> 1;
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, u8 & 1 == 1);
        res
    }

    fn and_reg8reg8(&mut self, r81: R8, r82: R8) {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        let res = self.and(r81_val, r82_val);
        self.registers.set8(r81, res);
    }

    fn and_reg8addrreg16(&mut self, r8: R8, r16: R16, mmu: &MMU) {
        let r8_val = self.registers.get8(r8);
        let addrreg16_val = mmu.get(self.registers.get16(r16));
        let res = self.and(r8_val, addrreg16_val);
        self.registers.set8(r8, res);
    }

    fn and_reg8u8(&mut self, r8: R8, u8: u8) {
        let r8_val = self.registers.get8(r8);
        let res = self.and(r8_val, u8);
        self.registers.set8(r8, res);
    }

    fn and(&mut self, u81: u8, u82: u8) -> u8 {
        let res = u81 & u82;
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, true);
        self.registers.set_f(FFlag::C, false);
        res
    }

    fn or_reg8reg8(&mut self, r81: R8, r82: R8) {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        let res = self.or(r81_val, r82_val);
        self.registers.set8(r81, res);
    }

    fn or_reg8addrreg16(&mut self, r8: R8, r16: R16, mmu: &MMU) {
        let r8_val = self.registers.get8(r8);
        let addrr16_val = mmu.get(self.registers.get16(r16));
        let res = self.or(r8_val, addrr16_val);
        self.registers.set8(r8, res);
    }

    fn or_reg8u8(&mut self, r8: R8, u8: u8) {
        let r8_val = self.registers.get8(r8);
        let res = self.or(r8_val, u8);
        self.registers.set8(r8, res);
    }

    fn or(&mut self, u81: u8, u82: u8) -> u8 {
        let res = u81 | u82;
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, false);
        res
    }

    fn check_cc(&self, c: Condition) -> bool {
        match c {
            Condition::NZ => !self.registers.get_f(FFlag::Z),
            Condition::Z => self.registers.get_f(FFlag::Z),
            Condition::NC => !self.registers.get_f(FFlag::C),
            Condition::C => self.registers.get_f(FFlag::C),
        }
    }

    fn add_reg8reg8(&mut self, r81: R8, r82: R8) {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        let res = self.add(r81_val, r82_val);
        self.registers.set8(r81, res);
    }

    fn add_reg16i8(&mut self, r16: R16, i8: i8) {
        let u16 = i8 as u16;
        let r16_val = self.registers.get16(r16);
        let res = r16_val.wrapping_add(u16);
        self.registers.set16(r16, res);

        self.registers.set_f(FFlag::Z, false);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, (r16_val & 0x0F) + (u16 & 0x0F) > 0x0F);
        self.registers.set_f(FFlag::C, (r16_val & 0xFF) + (u16 & 0xFF) > 0xFF);
    }

    fn add_reg16reg16(&mut self, r161: R16, r162: R16) {
        let r161_val = self.registers.get16(r161);
        let r162_val = self.registers.get16(r162);
        let res = r161_val as u32 + r162_val as u32;
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, (r161_val & 0x0FFF) + (r162_val & 0x0FFF) > 0xFFF);
        self.registers.set_f(FFlag::C, res & 0xFF0000 != 0);
        self.registers.set16(r161, res as u16);
    }

    fn add_reg8addrreg16(&mut self, r8: R8, r16: R16, mmu: &MMU) {
        let r8_val = self.registers.get8(r8);
        let addrreg16_val = mmu.get(self.registers.get16(r16));
        let res = self.add(r8_val, addrreg16_val);
        self.registers.set8(r8, res);
    }

    fn add_reg8u8(&mut self, r8: R8, u8: u8) {
        let r8_val = self.registers.get8(r8);
        let res = self.add(r8_val, u8);
        self.registers.set8(r8, res);
    }

    fn add(&mut self, u81: u8, u82: u8) -> u8 {
        let res = u81 as u16 + u82 as u16;
        self.registers.set_f(FFlag::Z, res & 0xFF == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, (u81 & 0x0F) + (u82 & 0x0F) > 0x0F);
        self.registers.set_f(FFlag::C, res & 0xFF00 != 0);
        (res & 0xff) as u8
    }

    fn sub_reg8reg8(&mut self, r81: R8, r82: R8) {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        let res = self.sub(r81_val, r82_val);
        self.registers.set8(r81, res);
    }

    fn sub_reg8addrreg16(&mut self, r8: R8, r16: R16, mmu: &MMU) {
        let r8_val = self.registers.get8(r8);
        let addrreg16_val = mmu.get(self.registers.get16(r16));
        let res = self.sub(r8_val, addrreg16_val);
        self.registers.set8(r8, res);
    }

    fn sub_reg8u8(&mut self, r8: R8, u8: u8) {
        let r8_val = self.registers.get8(r8);
        let res = self.sub(r8_val, u8);
        self.registers.set8(r8, res);
    }

    fn sub(&mut self, u81: u8, u82: u8) -> u8 {
        let (res, overflow) = u81.overflowing_sub(u82);
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, true);
        self.registers.set_f(FFlag::H, (u81 & 0x0F) < (u82 & 0x0F));
        self.registers.set_f(FFlag::C, overflow);
        (res & 0xff) as u8
    }

    fn cp_reg8reg8(&mut self, r81: R8, r82: R8) {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        self.cp(r81_val, r82_val);
    }
    fn cp_reg8addrreg16(&mut self, r8: R8, r16: R16, mmu: &MMU) {
        let r8_val = self.registers.get8(r8);
        let addrreg16_val = mmu.get(self.registers.get16(r16));
        self.cp(r8_val, addrreg16_val);
    }
    fn cp_reg8u8(&mut self, r8: R8, u8: u8) {
        let r8_val = self.registers.get8(r8);
        self.cp(r8_val, u8);
    }

    fn cp(&mut self, u81: u8, u82: u8) {
        let (res, overflow) = u81.overflowing_sub(u82);
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, true);
        self.registers.set_f(FFlag::H, (u81 & 0x0F) < (u82 & 0x0F));
        self.registers.set_f(FFlag::C, overflow);
    }

    fn dec_r8(&mut self, r8: R8) {
        let r8_val = self.registers.get8(r8);
        let res = r8_val.wrapping_sub(1);
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, true);
        self.registers.set_f(FFlag::H, (r8_val & 0x0F) == 0);
        self.registers.set8(r8, res)
    }

    fn dec_u8(&mut self, u8: u8) -> u8 {
        let res = u8.wrapping_sub(1);
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, true);
        self.registers.set_f(FFlag::H, (u8 & 0x0F) == 0);
        res
    }

    // fn pop(&mut self, mmu: &MMU) -> u16 {
    //     let data = mmu.pop(self.get16(SP));
    //     self.inc16(SP, 2);
    //     data
    // }

    fn rla(&mut self) {
        let a_val = self.registers.get8(A);
        let res = (a_val << 1) | if self.registers.get_f(FFlag::C) { 1 } else { 0 };
        self.registers.set_f(FFlag::Z, false);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, a_val & 0x80 == 0x80);
        self.registers.set8(A, res);
    }

    // fn rl_addrreg16(&mut self, r16: R16, mmu: &mut MMU) -> CycleT {
    //     let reg16_val =  self.get16(r16);
    //     let addrreg16_val = mmu.get(reg16_val);
    //     let res = self.rl(addrreg16_val);
    //     mmu.set(reg16_val, res);
    //     16
    // }

    fn rl_reg8(&mut self, r8: R8) {
        let r8_val = self.registers.get8(r8);
        let res = self.rl(r8_val);
        self.registers.set8(r8, res);
    }

    fn rl(&mut self, u8: u8) -> u8 {
        let res = (u8 << 1) | if self.registers.get_f(FFlag::C) { 1 } else { 0 };
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, u8 & 0x80 == 0x80);
        res
    }

    fn inc_r8(&mut self, r8: R8) {
        let r8_val = self.registers.get8(r8);
        let r8_inced = r8_val.wrapping_add(1);
        self.registers.set_f(FFlag::Z, r8_inced == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, (r8_val & 0x0F) + 1 > 0x0F);
        self.registers.set8(r8, r8_inced);
    }

    fn inc_u8(&mut self, u8: u8) -> u8 {
        let inced = u8.wrapping_add(1);
        self.registers.set_f(FFlag::Z, inced == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, (u8 & 0x0F) + 1 > 0x0F);
        inced
    }
    // fn inc_addrreg16(&mut self, r16: R16, mmu: &mut MMU) -> CycleT {
    //     let r16_val =  self.get16(r16);
    //     let addrreg16_val = mmu.get(r16_val);
    //     let res = addrreg16_val.wrapping_add(1);
    //
    //     self.set_f(FFlag::Z, res == 0);
    //     self.set_f(FFlag::N, false);
    //     self.set_f(FFlag::H, (res & 0x0F) == 0);
    //     mmu.set(r16_val, res);
    //     12
    // }
    //
    // fn bit_u8addrreg16(&mut self, u8: u8, r16: R16, mmu: &mut MMU) -> CycleT {
    //     let r16_val =  self.get16(r16);
    //     let addrreg16_val = mmu.get(r16_val);
    //     let res = self.res(u8, addrreg16_val);
    //     mmu.set(r16_val, res);
    //     16
    // }
    //
    // fn bit_u8reg8(&mut self, u8: u8, r8: R8) -> CycleT {
    //     let r8_val = self.get8(r8);
    //     self.bit(u8, r8_val);
    //     8
    // }

    fn bit(&mut self, u81: u8, u82: u8) {
        self.registers.set_f(FFlag::Z, (u82 >> u81) & 1 == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, true);
    }

    fn ld_addrreg16d_reg8(&mut self, r81: R8, r82: R8, mmu: &mut MMU) {
        // let reg16_val =  self.get16dec(r16);
        // mmu.set(reg16_val, self.get8(r8));
    }


    fn xor_reg8reg8(&mut self, r81: R8, r82: R8) {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        let res = self.xor(r81_val, r82_val);
        self.registers.set8(r81, res);
    }

    fn xor_reg8addrreg16(&mut self, r8: R8, r16: R16, mmu: &MMU) {
        let r8_val = self.registers.get8(r8);
        let addrreg16_val = mmu.get(self.registers.get16(r16));
        let res = self.xor(r8_val, addrreg16_val);
        self.registers.set8(r8, res);
    }

    fn xor_reg8u8(&mut self, r8: R8, u8: u8) {
        let r8_val = self.registers.get8(r8);
        let res = self.xor(r8_val, u8);
        self.registers.set8(r8, res);
    }

    fn xor(&mut self, u81: u8, u82: u8) -> u8 {
        let res = u81 ^ u82;
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, false);
        res
    }

    pub fn registers(&self) -> &Registers {
        &self.registers
    }
    pub fn cycle_op(&self) -> &CycleInstruction {
        &self.cycle_instruction
    }
}


// #[cfg(test)]
// mod tests {
//     use crate::cpu::CPU;
//     use crate::cpu::instruction::Op::Ld;
//     use crate::cpu::instruction::P::Reg16;
//     use crate::cpu::registers::R16::{PC, SP};
//     use crate::mmu::MMU;
//
//     #[test]
//     fn op31() {
//         let mut boot_rom = [0u8; 0x100];
//         boot_rom[0] = 0x31;
//         boot_rom[1] = 0x12;
//         boot_rom[2] = 0x34;
//         let mut mmu = MMU::new(boot_rom);
//         let mut sut = CPU::default();
//         for _ in 0..3 {
//             sut.cycle(&mut mmu);
//         }
//         assert_eq!(sut.op, Ld);
//         assert_eq!(sut.p1, Reg16(SP));
//         assert_eq!(sut.registers.get16(PC), 3);
//         assert_eq!(sut.registers.get16(SP), 0x3412);
//     }
// }
*/

use std::mem;
use crate::cpu::hex_u8::HexU8;
use crate::cpu::instruction::Instruction;
use crate::cpu::op::Op;
use crate::cpu::op::Op::{Dcd, DcdCB, Ld, Nop};
use crate::cpu::op_arg::OpArg;
use crate::cpu::op_arg::OpArg::{AddrRegd16, FetchU8, Non, Reg16, U16, U8};
use crate::cpu::registers::R16::{PC, SP};
use crate::cpu::registers::R8::{PCh, PCl};
use crate::cpu::registers::{R16, Registers};
use crate::mmu::MMU;

pub mod instruction;
mod decoder;
pub mod registers;
mod fflag;
mod fetcher;
pub mod op;
mod op_arg;
mod hex_u8;
mod executor;
mod condition;
mod hex_u16;

pub struct CPU {
    halted: bool,
    ime_requested: bool,
    ime: bool,
    registers: Registers,
    instruction: Instruction,
}

impl Default for CPU {
    fn default() -> Self {
        let mut registers = Registers::default();
        Self {
            halted: false,
            ime_requested: false,
            ime: false,
            instruction: Instruction::new((Dcd, FetchU8, Non), None),
            registers,
        }
    }
}

impl CPU {
    pub fn cycle(&mut self, mmu: &mut MMU) -> Result<(), String> {
        // let pc = self.registers.get16(PC);
        // if pc == 0x2EF { return Err(format!("{:06X}", pc)) };

        if !self.halted {
            let instruction = self.instruction.ins();
            // let pc = self.registers().get16(R16::PC);
            // if pc >= 0x185 {
            //     let (op, p1, p2) = instruction;
            //     let af = self.registers().get16(R16::AF);
            //     let bc = self.registers().get16(R16::BC);
            //     let de = self.registers().get16(R16::DE);
            //     let hl = self.registers().get16(R16::HL);
            //     let sp = self.registers().get16(R16::SP);
            //     match (op, p1, p2) {
            //         // (Op::Dcd, _, _) => println!("[{:#06X}] {:?} {:?} {:?} [AF:{:#06X} BC:{:#06X} DE:{:#06X}, HL:{:#06X}, SP:{:#06X}, IME:{}, IF:{:#08b}, IE:{:#08b}] ly:{}", pc, op, p1, p2, af, bc, de, hl, sp, jimbot.cpu().ime(), jimbot.mmu().get(0xFF0F), jimbot.mmu().get(0xFFFF), ly),
            //         (Op::Dcd, _, _) => println!("[{:#06X}] {:?} {:?} {:?} [AF:{:#06X} BC:{:#06X} DE:{:#06X}, HL:{:#06X}, SP:{:#06X}] ly:{} lyc:{} stat:{:#010b} if:{:#010b} ie:{:#010b} ime:{}", pc, op, p1, p2, af, bc, de, hl, sp, mmu.ly(), mmu.lyc(), mmu.get(0xFF41), mmu.get(0xFF0F), mmu.get(0xFFFF), self.ime),
            //         // (Op::DcdCB, _, _) => println!("[{:#06X}] {:?} {:?} {:?} [AF:{:#06X} BC:{:#06X} DE:{:#06X}, HL:{:#06X}, SP:{:#06X}]", pc, op, p1, p2, af, bc, de, hl, sp),
            //         (_, _, _) => {}//println!("\t-----> {:?} {:?} {:?}", op, p1, p2),
            //     }
            // }
            let (done, instruction) = Self::fetch(instruction, &mut self.registers, mmu);
            if !done {
                self.instruction.set_ins(instruction);
                return Ok(());
            }
            let next_instruction = self.execute(instruction, mmu)?;
            if let Some(next_instruction) = next_instruction {
                self.instruction = next_instruction;
            } else if let Some(next_instruction) = self.instruction.next_mut() {
                self.instruction = mem::take(next_instruction);
            } else {
                self.instruction = Instruction::new((Dcd, FetchU8, Non), None);
            }
        }

        self.handle_interrupt(mmu);
        Ok(())
    }

    fn handle_interrupt(&mut self, mmu: &mut MMU) {
        // Disable halt when interrupt is pending
        if mmu.ly() == 144 && mmu.lyc() == 144 {
            // println!("[{:#06X}] {:?} {:?} {:?} [AF:{:#06X} BC:{:#06X} DE:{:#06X}, HL:{:#06X}, SP:{:#06X}] ly:{} lyc:{} if:{:#08b} ie:{:#08b} ime:{}", self.repc, op, p1, p2, af, bc, de, hl, sp, jimbot.mmu().ly(), jimbot.mmu().lyc(), jimbot.mmu().get(0xFF0F), jimbot.mmu().get(0xFFFF), jimbot.cpu().ime());
            // println!("Handling interrupt ime ins: {:?}", self.instruction.ins());
        }
        if let (Dcd, _, _) = self.instruction.ins() {
            if mmu.get(0xFFFF) & mmu.get(0xFF0F) != 0 { self.halted = false; }
            if self.ime {
                let mut iflag = mmu.get_interrupt_flags();
                let requests = iflag.get_request_by_priority();
                // if mmu.ly() == 144 && mmu.lyc() == 144 {
                //     println!("Handling interrupt if:{:#08b}: ie:{:#08b} req:{:?}", mmu.get(0xFF0F), mmu.get(0xFFFF), requests);
                // }
                let ie = mmu.get_interrupt_enables();
                for request in requests {
                    if ie.is_enable(request) {
                        // println!("Do interrupt ({},{}): {:?}", mmu.ly(), mmu.lyc(), request);

                        // do interrupt
                        // if mmu.ly() == 144 && mmu.lyc() == 144 {
                        //     println!("Do interrupt if:{:#08b}: ie:{:#08b}", mmu.get(0xFF0F), mmu.get(0xFFFF));
                        // }
                        iflag.disable_request(request);
                        mmu.set_interrupt_flags(iflag);
                        self.ime = false;

                        // mmu.set(self.registers.getd16(R16::SP,1), self.registers.get8(PCh) );
                        // mmu.set(self.registers.getd16(R16::SP,1), self.registers.get8(PCl) );
                        // self.registers.set16(PC, request.routine_location());

                        let ins0 = (Nop, Non, Non);
                        let ins1 = (Nop, Non, Non);
                        let ins2 = (Ld, AddrRegd16(SP), U8(self.registers.get8(PCh).into()));
                        let ins3 = (Ld, AddrRegd16(SP), U8(self.registers.get8(PCl).into()));
                        let ins4 = (Ld, Reg16(PC), U16(request.routine_location().into()));
                        self.instruction = Instruction::new(
                            ins0, Some(Box::new(Instruction::new(
                                ins1, Some(Box::new(Instruction::new(
                                    ins2, Some(Box::new(Instruction::new(
                                        ins3, Some(Box::new(Instruction::new(ins4, None,
                                        ))),
                                    ))),
                                ))),
                            ))),
                        );
                        break;
                    }
                }
            } else if self.ime_requested {
                self.ime_requested = false;
                self.ime = true;
            }
        }
    }

    pub fn ime(&self) -> bool {
        self.ime
    }
    pub fn registers(&self) -> &Registers {
        &self.registers
    }
    pub fn instruction(&self) -> &Instruction {
        &self.instruction
    }
}