use Op::{Adc, Add, And, Bit, Call, Ccf, Cp, Cpl, Daa, Dcd, DcdCB, Dec, Di, Ei, Halt, Inc, Internal, Jp, Jr, Ld, Nop, Or, Res, Ret, EiImm, Rl, Rla, Rlc, Rlca, Rr, Rra, Rrc, Rrca, Rst, Sbc, Scf, Set, Sla, Sra, Srl, Sub, Swap, Write, Xor};
use OpArg::Non;
use crate::cpu::condition::Condition;
use crate::cpu::CPU;
use crate::cpu::fflag::FFlag;
use crate::cpu::instruction::Instruction;
use crate::cpu::op::Op;
use crate::cpu::op::Op::Read;
use crate::cpu::op_arg::OpArg;
use crate::cpu::op_arg::OpArg::{AddrReg16, AddrReg16d, AddrReg16i, AddrRegd16, AddrU16, CC, I8, InAddrReg8, InAddrU8, Reg16, Reg8, SPI8, U16, U8};
use crate::cpu::registers::{R16, R8};
use crate::cpu::registers::R16::{PC, SP};
use crate::cpu::registers::R8::{A, PCh, PCl};
use crate::mmu::MMU;

impl CPU {
    pub(crate) fn execute(&mut self, instruction: (Op, OpArg, OpArg), mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        match instruction {
            (Adc, Reg8(r8), AddrReg16(r16)) => self.exe_adc_r8_addrr16(r8, r16, mmu),
            (Adc, Reg8(r8), U8(u8)) => self.exe_adc_r8_u8(r8, u8.into()),
            (Adc, Reg8(r81), Reg8(r82)) => self.exe_adc_r8_r8(r81, r82),
            (Add, Reg16(r16), I8(i8)) => self.exe_add_r16_i8(r16, i8),
            (Add, Reg16(r161), Reg16(r162)) => self.exe_add_r16_r16(r161, r162),
            (Add, Reg8(r8), AddrReg16(r16)) => self.exe_add_r8_addrr16(r8, r16, mmu),
            (Add, Reg8(r8), Reg8(u8)) => self.exe_add_r8_r8(r8, u8.into()),
            (Add, Reg8(r8), U8(u8)) => self.exe_add_r8_u8(r8, u8.into()),
            (And, Reg8(r8), AddrReg16(r16)) => self.exe_and_r8_addrr16(r8, r16, mmu),
            (And, Reg8(r8), U8(u8)) => self.exe_and_r8_u8(r8, u8.into()),
            (And, Reg8(r81), Reg8(r82)) => self.exe_and_r8_r8(r81, r82),
            (Bit, U8(u8), AddrReg16(r16)) => self.exe_bit_u8_addrr16(u8.into(), r16, mmu),
            (Bit, U8(u8), Reg8(r8)) => self.exe_bit_u8_r8(u8.into(), r8),
            (Call, CC(c), U16(u16)) => self.exe_call_c_u16(c, u16.into()),
            (Call, U16(u16), Non) => self.exe_call_u16(u16.into()),
            (Ccf, Non, Non) => self.exe_ccf(),
            (Cp, Reg8(r8), AddrReg16(r16)) => self.exe_cp_r8_addrr16(r8, r16, mmu),
            (Cp, Reg8(r8), U8(u8)) => self.exe_cp_r8_u8(r8, u8.into()),
            (Cp, Reg8(r81), Reg8(r82)) => self.exe_cp_r8_r8(r81, r82),
            (Cpl, Non, Non) => self.exe_cpl(),
            (Daa, Non, Non) => self.exe_daa(),
            (Dcd, U8(u8), Non) => self.exe_decode_u8(u8.into(), mmu),
            (DcdCB, U8(u8), Non) => self.exe_decode_cb_u8(u8.into(), mmu),
            (Dec, AddrReg16(r16), Non) => self.exe_dec_addrr16(r16, mmu),
            (Dec, Reg16(r16), Non) => self.exe_dec_r16(r16),
            (Dec, Reg8(r8), Non) => self.exe_dec_r8(r8),
            (Di, Non, Non) => self.exe_di(),
            (Ei, Non, Non) => self.exe_ei(),
            (EiImm, Non, Non) => self.exe_ei_imm(),
            (Halt, Non, Non) => self.exe_halt(),
            (Inc, AddrReg16(r16), Non) => self.exe_inc_addrr16(r16, mmu),
            (Inc, Reg16(r16), Non) => self.exe_inc_r16(r16),
            (Inc, Reg8(r8), Non) => self.exe_inc_r8(r8),
            (Internal, Non, Non) => Ok(None),
            (Jp, CC(c), U16(u16)) => self.exe_jp_c_u16(c, u16.into()),
            (Jp, Reg16(r16), Non) => self.exe_jp_r16(r16),
            (Jp, U16(u16), Non) => self.exe_jp_u16(u16.into()),
            (Jr, CC(c), I8(i8)) => self.exe_jr_c_i8(c, i8),
            (Jr, I8(i8), Non) => self.exe_jr_i8(i8),
            (Ld, AddrReg16(r16), Reg8(r8)) => self.exe_ld_addrr16_r8(r16, r8, mmu),
            (Ld, AddrReg16(r16), U8(u8)) => self.exe_ld_addrr16_u8(r16, u8.into()),
            (Ld, AddrReg16d(r16), Reg8(r8)) => self.exe_ld_addrr16d_r8(r16, r8, mmu),
            (Ld, AddrReg16i(r16), Reg8(r8)) => self.exe_ld_addrr16i_r8(r16, r8, mmu),
            (Ld, AddrRegd16(r16), Reg8(r8)) => self.exe_ld_addrdr16_r8(r16, r8, mmu),
            (Ld, AddrRegd16(r16), U8(u8)) => self.exe_ld_addrdr16_u8(r16, u8.into(), mmu),
            (Ld, AddrU16(u16), Reg16(r16)) => self.exe_ld_addru16_r16(u16.into(), r16, mmu),
            (Ld, AddrU16(u16), Reg8(r8)) => self.exe_ld_addru16_r8(u16.into(), r8, mmu),
            (Ld, AddrU16(u16), U8(u8)) => self.exe_ld_addru16_u8(u16.into(), u8.into(), mmu),
            (Ld, InAddrReg8(r81), Reg8(r82)) => self.exe_ld_inaddrr8_r8(r81, r82, mmu),
            (Ld, InAddrU8(u8), Reg8(r8)) => self.exe_ld_inaddru8_r8(u8.into(), r8, mmu),
            (Ld, Reg16(r16), SPI8(i8)) => self.exe_ld_r16_spi8(r16, i8.into()),
            (Ld, Reg16(r16), U16(u16)) => self.exe_ld_r16_u16(r16, u16.into()),
            (Ld, Reg16(r161), Reg16(r162)) => self.exe_ld_r16_r16(r161, r162),
            (Ld, Reg8(r8), AddrReg16(r16)) => self.exe_ld_r8_addrr16(r8, r16, mmu),
            (Ld, Reg8(r8), AddrReg16d(r16)) => self.exe_ld_r8_addrr16d(r8, r16, mmu),
            (Ld, Reg8(r8), AddrReg16i(r16)) => self.exe_ld_r8_addrr16i(r8, r16, mmu),
            (Ld, Reg8(r8), AddrU16(u16)) => self.exe_ld_r8_addru16(r8, u16.into()),
            (Ld, Reg8(r8), InAddrU8(u8)) => self.exe_ld_r8_inaddru8(r8, u8.into(), mmu),
            (Ld, Reg8(r8), U8(u8)) => self.exe_ld_r8_u8(r8, u8.into()),
            (Ld, Reg8(r81), InAddrReg8(r82)) => self.exe_ld_r8_inaddrr8(r81, r82, mmu),
            (Ld, Reg8(r81), Reg8(r82)) => self.exe_ld_r8_r8(r81, r82),
            (Nop, Non, Non) => Ok(None),
            (Or, Reg8(r8), AddrReg16(r16)) => self.exe_or_r8_addrr16(r8, r16, mmu),
            (Or, Reg8(r8), U8(u8)) => self.exe_or_r8_u8(r8, u8.into()),
            (Or, Reg8(r81), Reg8(r82)) => self.exe_or_r8_r8(r81, r82),
            (Read, Reg8(r8), AddrU16(u16)) => self.exe_read_r8_addru16(r8, u16.into(), mmu),
            (Res, U8(u8), AddrReg16(r16)) => self.exe_res_u8_addrr16(u8.into(), r16, mmu),
            (Res, U8(u8), Reg8(r8)) => self.exe_res_u8_r8(u8.into(), r8),
            (Ret, CC(c), Non) => self.exe_ret_c(c),
            (Rl, AddrReg16(r16), Non) => self.exe_rl_addrr16(r16, mmu),
            (Rl, Reg8(r8), Non) => self.exe_rl_r8(r8),
            (Rla, Non, Non) => self.exe_rla(),
            (Rlc, AddrReg16(r16), Non) => self.exe_rlc_addrr16(r16, mmu),
            (Rlc, Reg8(r8), Non) => self.exe_rlc_r8(r8),
            (Rlca, Non, Non) => self.exe_rlca(),
            (Rr, AddrReg16(r16), Non) => self.exe_rr_addrr16(r16, mmu),
            (Rr, Reg8(r8), Non) => self.exe_rr_r8(r8),
            (Rra, Non, Non) => self.exe_rra(),
            (Rrc, AddrReg16(r16), Non) => self.exe_rrc_addrr16(r16, mmu),
            (Rrc, Reg8(r8), Non) => self.exe_rrc_r8(r8),
            (Rrca, Non, Non) => self.exe_rrca(),
            (Rst, U16(u16), Non) => self.exe_rst_u16(u16.into()),
            (Sbc, Reg8(r8), AddrReg16(r16)) => self.exe_sbc_r8_addrr16(r8, r16, mmu),
            (Sbc, Reg8(r8), U8(u8)) => self.exe_sbc_r8_u8(r8, u8.into()),
            (Sbc, Reg8(r81), Reg8(r82)) => self.exe_sbc_r8_r8(r81, r82),
            (Scf, Non, Non) => self.exe_scf(),
            (Set, U8(u8), AddrReg16(r16)) => self.exe_set_u8_addrr16(u8.into(), r16, mmu),
            (Set, U8(u8), Reg8(r8)) => self.exe_set_u8_r8(u8.into(), r8),
            (Sla, AddrReg16(r16), Non) => self.exe_sla_addrr16(r16, mmu),
            (Sla, Reg8(r8), Non) => self.exe_sla_r8(r8),
            (Sra, AddrReg16(r16), Non) => self.exe_sra_addrr16(r16, mmu),
            (Sra, Reg8(r8), Non) => self.exe_sra_r8(r8),
            (Srl, AddrReg16(r16), Non) => self.exe_srl_addrr16(r16, mmu),
            (Srl, Reg8(r8), Non) => self.exe_srl_r8(r8),
            (Sub, Reg8(r8), AddrReg16(r16)) => self.exe_sub_r8_addrr16(r8, r16, mmu),
            (Sub, Reg8(r8), U8(u8)) => self.exe_sub_r8_u8(r8, u8.into()),
            (Sub, Reg8(r81), Reg8(r82)) => self.exe_sub_r8_r8(r81, r82),
            (Swap, AddrReg16(r16), Non) => self.exe_swap_addrr16(r16, mmu),
            (Swap, Reg8(r8), Non) => self.exe_swap_r8(r8),
            (Write, AddrU16(u16), U8(u8)) => self.exe_write_addru16_u8(u16.into(), u8.into(), mmu),
            (Xor, Reg8(r8), AddrReg16(r16)) => self.exe_xor_r8_addrr16(r8, r16, mmu),
            (Xor, Reg8(r8), U8(u8)) => self.exe_xor_r8_u8(r8, u8.into()),
            (Xor, Reg8(r81), Reg8(r82)) => self.exe_xor_r8_r8(r81, r82),
            (op, p1, p2) => Err(format!("[EXE] Unknown instruction {:?} {:?} {:?}", op, p1, p2)),
        }
    }

    fn exe_add_r16_i8(&mut self, r16: R16, i8: i8) -> Result<Option<Instruction>, String> {
        let u16 = i8 as u16;
        let r16_val = self.registers.get16(r16);
        let res = r16_val.wrapping_add(u16);
        self.registers.set16(r16, res);

        self.registers.set_f(FFlag::Z, false);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, (r16_val & 0x0F) + (u16 & 0x0F) > 0x0F);
        self.registers.set_f(FFlag::C, (r16_val & 0xFF) + (u16 & 0xFF) > 0xFF);
        Ok(Some(Instruction::new((Internal, Non, Non), Some(Box::new(
            Instruction::new((Internal, Non, Non), None),
        )))))
    }

    fn exe_ret_c(&mut self, c: Condition) -> Result<Option<Instruction>, String> {
        if self.check_cc(c) {
            Ok(Some(Instruction::new(
                (Ld, Reg8(PCl), AddrReg16i(SP)),
                Some(Box::new(Instruction::new(
                    (Ld, Reg8(PCh), AddrReg16i(SP)),
                    Some(Box::new(Instruction::new(
                        (Internal, Non, Non), None,
                    ))),
                ))),
            )))
        } else {
            Ok(None)
        }
    }

    fn exe_scf(&mut self) -> Result<Option<Instruction>, String> {
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, true);
        Ok(None)
    }

    fn exe_rra(&mut self) -> Result<Option<Instruction>, String> {
        let a = self.registers.get8(A);
        let res = (a >> 1) | if self.registers.get_f(FFlag::C) { 0x80 } else { 0 };
        self.registers.set_f(FFlag::Z, false);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, a & 1 == 1);
        self.registers.set8(A, res);
        Ok(None)
    }

    fn exe_swap_addrr16(&mut self, r16: R16, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let address = self.registers.get16(r16);
        let res = self.exe_swap(mmu.get(address));
        Ok(Some(Instruction::new(
            (Ld, AddrU16(address.into()), U8(res.into())), None,
        )))
    }

    fn exe_swap_r8(&mut self, r8: R8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.exe_swap(r8_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_swap(&mut self, u8: u8) -> u8 {
        let res = ((u8 >> 4) & 0x0F) | ((u8 & 0x0F) << 4);
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, false);
        res
    }

    fn exe_rr_r8(&mut self, r8: R8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.rr(r8_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_rr_addrr16(&mut self, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let address = self.registers.get16(r16);
        let res = self.rr(mmu.get(address));
        Ok(Some(Instruction::new(
            (Ld, AddrU16(address.into()), U8(res.into())), None,
        )))
    }

    fn rr(&mut self, u8: u8) -> u8 {
        let res = (u8 >> 1) | if self.registers.get_f(FFlag::C) { 0x80 } else { 0 };
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, u8 & 1 == 1);
        res
    }

    fn exe_di(&mut self) -> Result<Option<Instruction>, String> {
        self.ime_requested = false;
        self.ime = false;
        Ok(None)
    }

    fn exe_halt(&mut self) -> Result<Option<Instruction>, String> {
        self.halted = true;
        Ok(None)
    }

    fn exe_ei(&mut self) -> Result<Option<Instruction>, String> {
        self.ime_requested = true;
        Ok(None)
    }

    fn exe_ei_imm(&mut self) -> Result<Option<Instruction>, String> {
        self.ime = true;
        Ok(None)
    }

    fn exe_jp_u16(&mut self, u16: u16) -> Result<Option<Instruction>, String> {
        self.registers.set16(PC, u16);
        Ok(Some(Instruction::new((Internal, Non, Non), None)))
    }

    fn exe_jp_c_u16(&mut self, c: Condition, u16: u16) -> Result<Option<Instruction>, String> {
        if self.check_cc(c) {
            self.registers.set16(PC, u16);
            Ok(Some(Instruction::new(
                (Internal, Non, Non), None,
            )))
        } else {
            Ok(None)
        }
    }

    fn exe_jp_r16(&mut self, r16: R16) -> Result<Option<Instruction>, String> {
        self.registers.set16(PC, self.registers.get16(r16));
        Ok(None)
    }

    fn exe_cpl(&mut self) -> Result<Option<Instruction>, String> {
        let mut a = self.registers.get8(A);
        let a = !a;
        self.registers.set_f(FFlag::N, true);
        self.registers.set_f(FFlag::H, true);
        self.registers.set8(A, a);
        Ok(None)
    }

    fn exe_rlca(&mut self) -> Result<Option<Instruction>, String> {
        let a = self.registers.get8(A);
        let carry = (a >> 7) & 1;
        self.registers.set8(A, a.wrapping_shl(1) | carry);

        self.registers.set_f(FFlag::Z, false);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, carry == 1);
        Ok(None)
    }

    fn exe_rrca(&mut self) -> Result<Option<Instruction>, String> {
        let a = self.registers.get8(A);
        let carry = a & 1;
        let res = a.wrapping_shr(1) | (carry << 7);
        self.registers.set_f(FFlag::Z, false);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, carry == 1);
        self.registers.set8(A, res);
        Ok(None)
    }

    fn exe_rla(&mut self) -> Result<Option<Instruction>, String> {
        let a_val = self.registers.get8(A);
        let res = (a_val << 1) | if self.registers.get_f(FFlag::C) { 1 } else { 0 };
        self.registers.set_f(FFlag::Z, false);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, a_val & 0x80 == 0x80);
        self.registers.set8(A, res);
        Ok(None)
    }

    fn exe_ccf(&mut self) -> Result<Option<Instruction>, String> {
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, !self.registers.get_f(FFlag::C));
        Ok(None)
    }

    fn exe_daa(&mut self) -> Result<Option<Instruction>, String> {
        let a = self.registers.get8(A);
        let carry = self.registers.get_f(FFlag::C);
        let half_carry = self.registers.get_f(FFlag::H);
        let subtraction = self.registers.get_f(FFlag::N);

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
        Ok(None)
    }

    fn exe_rl_addrr16(&mut self, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let address = self.registers.get16(r16);
        let res = self.exe_rl(mmu.get(address));
        Ok(Some(Instruction::new(
            (Ld, AddrU16(address.into()), U8(res.into())), None,
        )))
    }

    fn exe_rl_r8(&mut self, r8: R8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.exe_rl(r8_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_rl(&mut self, u8: u8) -> u8 {
        let res = (u8 << 1) | if self.registers.get_f(FFlag::C) { 1 } else { 0 };
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, u8 & 0x80 == 0x80);
        res
    }

    fn exe_rlc_addrr16(&mut self, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let address = self.registers.get16(r16);
        let res = self.exe_rlc(mmu.get(address));
        Ok(Some(Instruction::new(
            (Ld, AddrU16(address.into()), U8(res.into())), None,
        )))
    }

    fn exe_rlc_r8(&mut self, r8: R8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.exe_rlc(r8_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_rlc(&mut self, u8: u8) -> u8 {
        let carry = (u8 >> 7) & 1;
        let res = u8.wrapping_shl(1) | carry;
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, carry == 1);
        res
    }

    fn exe_sra_addrr16(&mut self, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let address = self.registers.get16(r16);
        let res = self.exe_sra(mmu.get(address));
        Ok(Some(Instruction::new(
            (Ld, AddrU16(address.into()), U8(res.into())), None,
        )))
    }

    fn exe_sra_r8(&mut self, r8: R8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.exe_sra(r8_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_sra(&mut self, u8: u8) -> u8 {
        let carry = u8 & 1;
        let result = u8.wrapping_shr(1) | (u8 & 0x80);

        self.registers.set_f(FFlag::Z, result == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, carry == 1);

        result
    }

    fn exe_sla_addrr16(&mut self, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let address = self.registers.get16(r16);
        let res = self.exe_sla(mmu.get(address));
        Ok(Some(Instruction::new(
            (Ld, AddrU16(address.into()), U8(res.into())), None,
        )))
    }

    fn exe_sla_r8(&mut self, r8: R8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.exe_sla(r8_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_sla(&mut self, u8: u8) -> u8 {
        let carry = (u8 >> 7) & 1;
        let result = u8.wrapping_shl(1);

        self.registers.set_f(FFlag::Z, result == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, carry == 1);

        result
    }

    fn exe_rrc_addrr16(&mut self, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let address = self.registers.get16(r16);
        let res = self.exe_rrc(mmu.get(address));
        Ok(Some(Instruction::new(
            (Ld, AddrU16(address.into()), U8(res.into())), None,
        )))
    }

    fn exe_rrc_r8(&mut self, r8: R8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.exe_rrc(r8_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_rrc(&mut self, u8: u8) -> u8 {
        let carry = u8 & 1;
        let res = u8.wrapping_shr(1) | (carry << 7);
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, carry == 1);
        res
    }

    fn exe_call_u16(&mut self, u16: u16) -> Result<Option<Instruction>, String> {
        let pc = self.registers.get16(PC);
        self.registers.set16(PC, u16);
        let push_ins = self.exe_push_u16(((pc >> 8) & 0xFF) as u8, (pc & 0xFF) as u8).ok().unwrap().unwrap();
        Ok(Some(Instruction::new((Internal, Non, Non), Some(Box::new(push_ins)))))
    }

    fn exe_rst_u16(&mut self, u16: u16) -> Result<Option<Instruction>, String> {
        let pc = self.registers.get16(PC);
        self.registers.set16(PC, u16);
        self.exe_push_u16(((pc >> 8) & 0xFF) as u8, (pc & 0xFF) as u8)
    }

    fn exe_call_c_u16(&mut self, c: Condition, u16: u16) -> Result<Option<Instruction>, String> {
        if self.check_cc(c) {
            self.exe_call_u16(u16)
        } else {
            Ok(None)
        }
    }

    fn exe_push_u16(&mut self, u8h: u8, u8l: u8) -> Result<Option<Instruction>, String> {
        Ok(Some(Instruction::new(
            (Ld, AddrRegd16(SP), U8(u8h.into())),
            Some(Box::new(Instruction::new(
                (Ld, AddrRegd16(SP), U8(u8l.into())),
                None,
            ))),
        )))
    }

    fn exe_dec_r16(&mut self, r16: R16) -> Result<Option<Instruction>, String> {
        self.registers.dec16(r16, 1);
        Ok(None)
    }

    fn exe_dec_r8(&mut self, r8: R8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.exe_dec_u8(r8_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_dec_addrr16(&mut self, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let address = self.registers.get16(r16);
        let res = self.exe_dec_u8(mmu.get(address));
        Ok(Some(Instruction::new(
            (Ld, AddrU16(address.into()), U8(res.into())), None,
        )))
    }

    fn exe_inc_addrr16(&mut self, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let address = self.registers.get16(r16);
        let addrreg16_val = mmu.get(address);
        let res = addrreg16_val.wrapping_add(1);

        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, (res & 0x0F) == 0);
        Ok(Some(Instruction::new(
            (Ld, AddrU16(address.into()), U8(res.into())), None,
        )))
    }

    fn exe_dec_u8(&mut self, u8: u8) -> u8 {
        let res = u8.wrapping_sub(1);
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, true);
        self.registers.set_f(FFlag::H, (u8 & 0x0F) == 0);
        res
    }

    fn exe_inc_r8(&mut self, r8: R8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let r8_inced = r8_val.wrapping_add(1);
        self.registers.set_f(FFlag::Z, r8_inced == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, (r8_val & 0x0F) + 1 > 0x0F);
        self.registers.set8(r8, r8_inced);
        Ok(None)
    }

    fn exe_inc_r16(&mut self, r16: R16) -> Result<Option<Instruction>, String> {
        self.registers.inc16(r16, 1);
        Ok(None)
    }

    fn check_cc(&self, c: Condition) -> bool {
        match c {
            Condition::NZ => !self.registers.get_f(FFlag::Z),
            Condition::Z => self.registers.get_f(FFlag::Z),
            Condition::NC => !self.registers.get_f(FFlag::C),
            Condition::C => self.registers.get_f(FFlag::C),
        }
    }

    fn exe_jr_i8(&mut self, i8: i8) -> Result<Option<Instruction>, String> {
        let pc = self.registers.get16(PC);
        let next_pc = (((pc as u32 as i32) + (i8 as i32)) as u16);
        Ok(Some(
            Instruction::new((Ld, Reg16(PC), U16(next_pc.into())), None)
        ))
    }
    fn exe_jr_c_i8(&mut self, c: Condition, i8: i8) -> Result<Option<Instruction>, String> {
        if self.check_cc(c) {
            self.exe_jr_i8(i8)
        } else {
            Ok(None)
        }
    }
    fn exe_set_u8_r8(&mut self, u8: u8, r8: R8) -> Result<Option<Instruction>, String> {
        let res = self.exe_set(u8, self.registers.get8(r8));
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_set_u8_addrr16(&mut self, u8: u8, r16: R16, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let address = self.registers.get16(r16);
        let res = self.exe_set(u8, mmu.get(self.registers.get16(r16)));
        Ok(Some(Instruction::new(
            (Write, AddrU16(address.into()), U8(res.into())), None,
        )))
    }

    fn exe_set(&mut self, u81: u8, u82: u8) -> u8 {
        u82 | (1 << u81)
    }
    fn exe_res_u8_r8(&mut self, u8: u8, r8: R8) -> Result<Option<Instruction>, String> {
        let res = self.exe_res(u8, self.registers.get8(r8));
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_res_u8_addrr16(&mut self, u8: u8, r16: R16, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let address = self.registers.get16(r16);
        let res = self.exe_res(u8, mmu.get(self.registers.get16(r16)));
        Ok(Some(Instruction::new(
            (Write, AddrU16(address.into()), U8(res.into())), None,
        )))
    }

    fn exe_res(&mut self, u81: u8, u82: u8) -> u8 {
        u82 & !(1 << u81)
    }

    fn exe_bit_u8_r8(&mut self, u8: u8, r8: R8) -> Result<Option<Instruction>, String> {
        self.bit(u8, self.registers.get8(r8));
        Ok(None)
    }

    fn exe_bit_u8_addrr16(&mut self, u8: u8, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        self.bit(u8, mmu.get(self.registers.get16(r16)));
        Ok(None)
    }

    fn bit(&mut self, u81: u8, u82: u8) {
        self.registers.set_f(FFlag::Z, (u82 >> u81) & 1 == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, true);
    }

    fn exe_srl_r8(&mut self, r8: R8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.srl(r8_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_srl_addrr16(&mut self, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let address = self.registers.get16(r16);
        let res = self.srl(mmu.get(address));
        Ok(Some(Instruction::new(
            (Ld, AddrU16(address.into()), U8(res.into())), None,
        )))
    }

    fn srl(&mut self, u8: u8) -> u8 {
        let res = u8 >> 1;
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, u8 & 1 == 1);
        res
    }

    fn exe_decode_u8(&mut self, u8: u8, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let (immediate_execute, result) = Self::decode(u8);
        if let (true, Ok(Some(next_instruction))) = (immediate_execute, result.as_ref()) {
            assert!(next_instruction.next().is_none(), "Immediate should only have single instruction");
            self.execute(next_instruction.ins(), mmu)
        } else {
            result
        }
    }

    fn exe_decode_cb_u8(&mut self, u8: u8, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let (immediate_execute, result) = Self::decode_cb(u8);
        if let (true, Ok(Some(next_instruction))) = (immediate_execute, result.as_ref()) {
            assert!(next_instruction.next().is_none(), "Immediate should only have single instruction");
            self.execute(next_instruction.ins(), mmu)
        } else {
            result
        }
    }

    fn exe_xor_r8_r8(&mut self, r81: R8, r82: R8) -> Result<Option<Instruction>, String> {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        let res = self.exe_xor(r81_val, r82_val);
        self.registers.set8(r81, res);
        Ok(None)
    }

    fn exe_cp_r8_addrr16(&mut self, r8: R8, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let addrr16_val = mmu.get(self.registers.get16(r16));
        self.exe_cp(r8_val, addrr16_val);
        Ok(None)
    }

    fn exe_add_r8_addrr16(&mut self, r8: R8, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let addrr16_val = mmu.get(self.registers.get16(r16));
        let res = self.exe_add(r8_val, addrr16_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_add_r16_r16(&mut self, r161: R16, r162: R16) -> Result<Option<Instruction>, String> {
        let r161_val = self.registers.get16(r161);
        let r162_val = self.registers.get16(r162);
        let res = r161_val as u32 + r162_val as u32;
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, (r161_val & 0x0FFF) + (r162_val & 0x0FFF) > 0xFFF);
        self.registers.set_f(FFlag::C, res & 0xFF0000 != 0);
        self.registers.set16(r161, res as u16);
        Ok(None)
    }

    fn exe_xor_r8_addrr16(&mut self, r8: R8, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let addrr16_val = mmu.get(self.registers.get16(r16));
        let res = self.exe_xor(r8_val, addrr16_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_or_r8_addrr16(&mut self, r8: R8, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let addrr16_val = mmu.get(self.registers.get16(r16));
        let res = self.exe_or(r8_val, addrr16_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_cp_r8_r8(&mut self, r81: R8, r82: R8) -> Result<Option<Instruction>, String> {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        self.exe_cp(r81_val, r82_val);
        Ok(None)
    }

    fn exe_or_r8_r8(&mut self, r81: R8, r82: R8) -> Result<Option<Instruction>, String> {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        let res = self.exe_or(r81_val, r82_val);
        self.registers.set8(r81, res);
        Ok(None)
    }

    fn exe_or(&mut self, u81: u8, u82: u8) -> u8 {
        let res = u81 | u82;
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, false);
        res
    }

    fn exe_cp_r8_u8(&mut self, r8: R8, u8: u8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        self.exe_cp(r8_val, u8);
        Ok(None)
    }

    fn exe_and_r8_r8(&mut self, r81: R8, r82: R8) -> Result<Option<Instruction>, String> {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        let res = self.exe_and(r81_val, r82_val);
        self.registers.set8(r81, res);
        Ok(None)
    }

    fn exe_add_r8_u8(&mut self, r8: R8, u8: u8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.exe_add(r8_val, u8);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_add_r8_r8(&mut self, r81: R8, r82: R8) -> Result<Option<Instruction>, String> {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        let res = self.exe_add(r81_val, r82_val);
        self.registers.set8(r81, res);
        Ok(None)
    }

    fn exe_adc_r8_u8(&mut self, r8: R8, u8: u8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.exe_adc(r8_val, u8);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_add(&mut self, u81: u8, u82: u8) -> u8 {
        let res = u81 as u16 + u82 as u16;
        self.registers.set_f(FFlag::Z, res & 0xFF == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, (u81 & 0x0F) + (u82 & 0x0F) > 0x0F);
        self.registers.set_f(FFlag::C, res & 0xFF00 != 0);
        (res & 0xff) as u8
    }

    fn exe_cp(&mut self, u81: u8, u82: u8) {
        let (res, overflow) = u81.overflowing_sub(u82);
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, true);
        self.registers.set_f(FFlag::H, (u81 & 0x0F) < (u82 & 0x0F));
        self.registers.set_f(FFlag::C, overflow);
    }

    fn exe_adc_r8_r8(&mut self, r81: R8, r82: R8) -> Result<Option<Instruction>, String> {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        let res = self.exe_adc(r81_val, r82_val);
        self.registers.set8(r81, res);
        Ok(None)
    }

    fn exe_adc_r8_addrr16(&mut self, r8: R8, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let addrr16_val = mmu.get(self.registers.get16(r16));
        let res = self.exe_adc(r8_val, addrr16_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_adc(&mut self, u81: u8, u82: u8) -> u8 {
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

    fn exe_sub_r8_r8(&mut self, r81: R8, r82: R8) -> Result<Option<Instruction>, String> {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        let res = self.exe_sub(r81_val, r82_val);
        self.registers.set8(r81, res);
        Ok(None)
    }

    fn exe_sub_r8_addrr16(&mut self, r8: R8, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let addrr16_val = mmu.get(self.registers.get16(r16));
        let res = self.exe_sub(r8_val, addrr16_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_sub(&mut self, u81: u8, u82: u8) -> u8 {
        let (res, overflow) = u81.overflowing_sub(u82);
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, true);
        self.registers.set_f(FFlag::H, (u81 & 0x0F) < (u82 & 0x0F));
        self.registers.set_f(FFlag::C, overflow);
        (res & 0xff) as u8
    }

    fn exe_and_r8_addrr16(&mut self, r8: R8, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let addrr16_val = mmu.get(self.registers.get16(r16));
        let res = self.exe_and(r8_val, addrr16_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_and_r8_u8(&mut self, r8: R8, u8: u8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.exe_and(r8_val, u8);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_and(&mut self, u81: u8, u82: u8) -> u8 {
        let res = u81 & u82;
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, true);
        self.registers.set_f(FFlag::C, false);
        res
    }

    fn exe_or_r8_u8(&mut self, r8: R8, u8: u8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.exe_or(r8_val, u8);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_xor_r8_u8(&mut self, r8: R8, u8: u8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.exe_xor(r8_val, u8);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_sbc_r8_r8(&mut self, r81: R8, r82: R8) -> Result<Option<Instruction>, String> {
        let r81_val = self.registers.get8(r81);
        let r82_val = self.registers.get8(r82);
        let res = self.exe_sbc(r81_val, r82_val);
        self.registers.set8(r81, res);
        Ok(None)
    }

    fn exe_sbc_r8_addrr16(&mut self, r8: R8, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let addrr16_val = mmu.get(self.registers.get16(r16));
        let res = self.exe_sbc(r8_val, addrr16_val);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_sbc_r8_u8(&mut self, r8: R8, u8: u8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.exe_sbc(r8_val, u8);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_sbc(&mut self, u81: u8, u82: u8) -> u8 {
        let u161 = u81 as u16;
        let u162 = u82 as u16;
        let carry = self.registers.get_f(FFlag::C) as u16;
        let res = u161.wrapping_sub(u162).wrapping_sub(carry);

        self.registers.set_f(FFlag::Z, res & 0xFF == 0);
        self.registers.set_f(FFlag::N, true);
        self.registers.set_f(FFlag::H, (u161 & 0x0F).wrapping_sub(u162 & 0x0F).wrapping_sub(carry) > 0x0F);
        self.registers.set_f(FFlag::C, res & 0xFF00 != 0);
        (res & 0xFF) as u8
    }

    fn exe_sub_r8_u8(&mut self, r8: R8, u8: u8) -> Result<Option<Instruction>, String> {
        let r8_val = self.registers.get8(r8);
        let res = self.exe_sub(r8_val, u8);
        self.registers.set8(r8, res);
        Ok(None)
    }

    fn exe_xor(&mut self, u81: u8, u82: u8) -> u8 {
        let res = u81 ^ u82;
        self.registers.set_f(FFlag::Z, res == 0);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, false);
        self.registers.set_f(FFlag::C, false);
        res
    }

    fn exe_ld_r8_u8(&mut self, r8: R8, u8: u8) -> Result<Option<Instruction>, String> {
        self.registers.set8(r8, u8);
        Ok(None)
    }

    fn exe_ld_r8_r8(&mut self, r81: R8, r82: R8) -> Result<Option<Instruction>, String> {
        self.registers.set8(r81, self.registers.get8(r82));
        Ok(None)
    }

    fn exe_ld_r16_u16(&mut self, r16: R16, u16: u16) -> Result<Option<Instruction>, String> {
        self.registers.set16(r16, u16);
        Ok(None)
    }

    fn exe_ld_r16_r16(&mut self, r161: R16, r162: R16) -> Result<Option<Instruction>, String> {
        self.registers.set16(r161, self.registers.get16(r162));
        Ok(None)
    }

    fn exe_ld_r16_spi8(&mut self, r16: R16, i8: i8) -> Result<Option<Instruction>, String> {
        let sp = self.registers.get16(SP);
        let u16 = i8 as u16;
        let res = sp.wrapping_add(u16);

        self.registers.set_f(FFlag::Z, false);
        self.registers.set_f(FFlag::N, false);
        self.registers.set_f(FFlag::H, (sp & 0xf) + (u16 & 0xf) > 0xf);
        self.registers.set_f(FFlag::C, (sp & 0xff) + (u16 & 0xff) > 0xff);

        self.registers.set16(r16, res);
        Ok(Some(Instruction::new((Internal, Non, Non), None)))
    }

    fn exe_ld_addru16_u8(&mut self, u16: u16, u8: u8, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        mmu.set(u16, u8);
        Ok(None)
    }

    fn exe_ld_r8_addru16(&mut self, r8: R8, u16: u16) -> Result<Option<Instruction>, String> {
        Ok(Some(Instruction::new(
            (Read, Reg8(r8), AddrU16(u16.into())), None,
        )))
    }

    fn exe_read_r8_addru16(&mut self, r8: R8, u16: u16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        self.registers.set8(r8, mmu.get(u16));
        Ok(None)
    }

    fn exe_ld_r8_addrr16(&mut self, r8: R8, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let addr = self.registers.get16(r16);
        self.registers.set8(r8, mmu.get(addr));
        Ok(None)
    }

    fn exe_ld_r8_addrr16i(&mut self, r8: R8, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let addr = self.registers.get16i(r16, 1);
        self.registers.set8(r8, mmu.get(addr));
        Ok(None)
    }

    fn exe_ld_r8_addrr16d(&mut self, r8: R8, r16: R16, mmu: &MMU) -> Result<Option<Instruction>, String> {
        let addr = self.registers.get16d(r16, 1);
        self.registers.set8(r8, mmu.get(addr));
        Ok(None)
    }

    fn exe_ld_addru16_r8(&mut self, u16: u16, r8: R8, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        Ok(Some(Instruction::new(
            (Write, AddrU16(u16.into()), U8(self.registers.get8(r8).into())), None
        )))
    }

    fn exe_write_addru16_u8(&mut self, u16: u16, u8: u8, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        mmu.set(u16, u8);
        Ok(None)
    }

    fn exe_ld_addru16_r16(&mut self, u16: u16, r16: R16, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let val = self.registers.get16(r16);
        let val_1 = (val & 0xFF) as u8;
        let val_2 = ((val >> 8) & 0xFF) as u8;

        Ok(Some(Instruction::new(
            (Ld, AddrU16(u16.into()), U8(val_1.into())), Some(Box::new(Instruction::new(
                (Ld, AddrU16((u16 + 1).into()), U8(val_2.into())), None,
            ))),
        )))
    }

    fn exe_ld_addrr16_r8(&mut self, r16: R16, r8: R8, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let addr = self.registers.get16(r16);
        mmu.set(addr, self.registers.get8(r8));
        Ok(None)
    }

    fn exe_ld_addrr16i_r8(&mut self, r16: R16, r8: R8, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let addr = self.registers.get16i(r16, 1);
        mmu.set(addr, self.registers.get8(r8));
        Ok(None)
    }

    fn exe_ld_addrr16_u8(&mut self, r16: R16, u8: u8) -> Result<Option<Instruction>, String> {
        let addr = self.registers.get16(r16);
        Ok(Some(
            Instruction::new((Ld, AddrU16(addr.into()), U8(u8.into())), None)
        ))
    }
    fn exe_ld_addrdr16_u8(&mut self, r16: R16, u8: u8, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let addr = self.registers.getd16(r16, 1);
        mmu.set(addr, u8);
        Ok(None)
    }

    fn exe_ld_addrdr16_r8(&mut self, r16: R16, r8: R8, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let addr = self.registers.getd16(r16, 1);
        mmu.set(addr, self.registers.get8(r8));
        Ok(None)
    }

    fn exe_ld_inaddrr8_r8(&mut self, r81: R8, r82: R8, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let addr = 0xFF00 + (self.registers.get8(r81) as u16);
        mmu.set(addr, self.registers.get8(r82));
        Ok(None)
    }

    fn exe_ld_r8_inaddrr8(&mut self, r81: R8, r82: R8, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let addr = 0xFF00 + (self.registers.get8(r82) as u16);
        self.registers.set8(r81, mmu.get(addr));
        Ok(None)
    }

    fn exe_ld_r8_inaddru8(&mut self, r8: R8, u8: u8, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let addr = 0xFF00 + (u8 as u16);
        self.registers.set8(r8, mmu.get(addr));
        Ok(None)
    }
    fn exe_ld_inaddru8_r8(&mut self, u8: u8, r8: R8, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let addr = 0xFF00 + (u8 as u16);
        mmu.set(addr, self.registers.get8(r8));
        Ok(None)
    }

    fn exe_ld_addrr16d_r8(&mut self, r16: R16, r8: R8, mmu: &mut MMU) -> Result<Option<Instruction>, String> {
        let addr = self.registers.get16d(r16, 1);
        mmu.set(addr, self.registers.get8(r8));
        Ok(None)
    }
}