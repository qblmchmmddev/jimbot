use crate::cpu::condition::Condition;
use crate::cpu::CPU;
use crate::cpu::instruction::Instruction;
use crate::cpu::op::Op;
use crate::cpu::op::Op::{Adc, Add, And, Bit, Call, Ccf, Cp, Cpl, Daa, DcdCB, Dec, Di, Ei, Halt, Inc, Internal, Jp, Jr, Ld, Nop, Or, Res, Ret, EiImm, Rl, Rla, Rlc, Rlca, Rr, Rra, Rrc, Rrca, Rst, Sbc, Scf, Set, Sla, Sra, Srl, Sub, Swap, Xor};
use crate::cpu::op_arg::OpArg::{AddrReg16, AddrReg16d, AddrReg16i, AddrRegd16, CC, FetchAddrU16, FetchI8, FetchInAddrU8, FetchSPI8, FetchU16, FetchU8, InAddrReg8, InAddrU8, Non, Reg16, Reg8, U16, U8};
use crate::cpu::registers::R16::{BC, DE, HL, SP};
use crate::cpu::registers::{R16, R8};
use crate::cpu::registers::R8::{A, B, C, D, E, F, H, L, P, PCh, PCl, S};

impl CPU {
    pub(crate) fn decode(byte: u8) -> (bool, Result<Option<Instruction>, String>) {
        match byte {
            0x00 => Self::dcd_nop(),
            0xCB => Self::dcd_decode_cb(),

            0xF8 => Self::dcd_ld_r16_spi8(HL),

            0xE8 => Self::dcd_add_r16_i8(SP),

            0xFB => Self::dcd_ei(),

            0xD9 => Self::dcd_reti(),

            0xC7 => Self::dcd_rst(0x00),
            0xD7 => Self::dcd_rst(0x10),
            0xE7 => Self::dcd_rst(0x20),
            0xF7 => Self::dcd_rst(0x30),
            0xCF => Self::dcd_rst(0x08),
            0xDF => Self::dcd_rst(0x18),
            0xEF => Self::dcd_rst(0x28),
            0xFF => Self::dcd_rst(0x38),

            0x76 => Self::dcd_halt(),

            0xC3 => Self::dcd_jp_u16(),

            0x2F => Self::dcd_cpl(),

            0xC2 => Self::dcd_jp_c_u16(Condition::NZ),
            0xD2 => Self::dcd_jp_c_u16(Condition::NC),
            0xCA => Self::dcd_jp_c_u16(Condition::Z),
            0xDA => Self::dcd_jp_c_u16(Condition::C),

            0xE9 => Self::dcd_jp_r16(HL),

            0xC4 => Self::dcd_call_c_u16(Condition::NZ),
            0xD4 => Self::dcd_call_c_u16(Condition::NC),
            0xCC => Self::dcd_call_c_u16(Condition::Z),
            0xDC => Self::dcd_call_c_u16(Condition::C),

            0xF3 => Self::dcd_di(),

            0x27 => Self::dcd_daa(),

            0xFA => Self::dcd_ld_r8_addru16(A),

            0x2A => Self::dcd_ld_r8_addrr16i(A, HL),
            0x3A => Self::dcd_ld_r8_addrr16d(A, HL),

            0xCD => Self::dcd_call_u16(),

            0xC9 => Self::dcd_ret(),

            0xC0 => Self::dcd_ret_c(Condition::NZ),
            0xD0 => Self::dcd_ret_c(Condition::NC),
            0xC8 => Self::dcd_ret_c(Condition::Z),
            0xD8 => Self::dcd_ret_c(Condition::C),

            0x22 => Self::dcd_ld_addrr16i_r8(HL, A),

            0x80 => Self::dcd_add_r8_r8(A, B),
            0x81 => Self::dcd_add_r8_r8(A, C),
            0x82 => Self::dcd_add_r8_r8(A, D),
            0x83 => Self::dcd_add_r8_r8(A, E),
            0x84 => Self::dcd_add_r8_r8(A, H),
            0x85 => Self::dcd_add_r8_r8(A, L),
            0x86 => Self::dcd_add_r8_addrr16(A, HL),
            0x87 => Self::dcd_add_r8_r8(A, A),
            0xC6 => Self::dcd_add_r8_u8(A),

            0x09 => Self::dcd_add_r16_r16(HL, BC),
            0x19 => Self::dcd_add_r16_r16(HL, DE),
            0x29 => Self::dcd_add_r16_r16(HL, HL),
            0x39 => Self::dcd_add_r16_r16(HL, SP),

            0x88 => Self::dcd_adc_r8_r8(A, B),
            0x89 => Self::dcd_adc_r8_r8(A, C),
            0x8A => Self::dcd_adc_r8_r8(A, D),
            0x8B => Self::dcd_adc_r8_r8(A, E),
            0x8C => Self::dcd_adc_r8_r8(A, H),
            0x8D => Self::dcd_adc_r8_r8(A, L),
            0x8E => Self::dcd_adc_r8_addrr16(A, HL),
            0x8F => Self::dcd_adc_r8_r8(A, A),
            0xCE => Self::dcd_adc_r8_u8(A),

            0x90 => Self::dcd_sub_r8_r8(A, B),
            0x91 => Self::dcd_sub_r8_r8(A, C),
            0x92 => Self::dcd_sub_r8_r8(A, D),
            0x93 => Self::dcd_sub_r8_r8(A, E),
            0x94 => Self::dcd_sub_r8_r8(A, H),
            0x95 => Self::dcd_sub_r8_r8(A, L),
            0x96 => Self::dcd_sub_r8_addrr16(A, HL),
            0x97 => Self::dcd_sub_r8_r8(A, A),
            0xD6 => Self::dcd_sub_r8_u8(A),

            0x98 => Self::dcd_sbc_r8_r8(A, B),
            0x99 => Self::dcd_sbc_r8_r8(A, C),
            0x9A => Self::dcd_sbc_r8_r8(A, D),
            0x9B => Self::dcd_sbc_r8_r8(A, E),
            0x9C => Self::dcd_sbc_r8_r8(A, H),
            0x9D => Self::dcd_sbc_r8_r8(A, L),
            0x9E => Self::dcd_sbc_r8_addrr16(A, HL),
            0x9F => Self::dcd_sbc_r8_r8(A, A),
            0xDE => Self::dcd_sbc_r8_u8(A),

            0xA0 => Self::dcd_and_r8_r8(A, B),
            0xA1 => Self::dcd_and_r8_r8(A, C),
            0xA2 => Self::dcd_and_r8_r8(A, D),
            0xA3 => Self::dcd_and_r8_r8(A, E),
            0xA4 => Self::dcd_and_r8_r8(A, H),
            0xA5 => Self::dcd_and_r8_r8(A, L),
            0xA6 => Self::dcd_and_r8_addrr16(A, HL),
            0xA7 => Self::dcd_and_r8_r8(A, A),
            0xE6 => Self::dcd_and_r8_u8(A),

            0xA8 => Self::dcd_xor_r8_r8(A, B),
            0xA9 => Self::dcd_xor_r8_r8(A, C),
            0xAA => Self::dcd_xor_r8_r8(A, D),
            0xAB => Self::dcd_xor_r8_r8(A, E),
            0xAC => Self::dcd_xor_r8_r8(A, H),
            0xAD => Self::dcd_xor_r8_r8(A, L),
            0xAE => Self::dcd_xor_r8_addrr16(A, HL),
            0xAF => Self::dcd_xor_r8_r8(A, A),
            0xEE => Self::dcd_xor_r8_u8(A),

            0xB0 => Self::dcd_or_r8_r8(A, B),
            0xB1 => Self::dcd_or_r8_r8(A, C),
            0xB2 => Self::dcd_or_r8_r8(A, D),
            0xB3 => Self::dcd_or_r8_r8(A, E),
            0xB4 => Self::dcd_or_r8_r8(A, H),
            0xB5 => Self::dcd_or_r8_r8(A, L),
            0xB6 => Self::dcd_or_r8_addrr16(A, HL),
            0xB7 => Self::dcd_or_r8_r8(A, A),
            0xF6 => Self::dcd_or_r8_u8(A),

            0x0B => Self::dcd_dec_r16(BC),
            0x1B => Self::dcd_dec_r16(DE),
            0x2B => Self::dcd_dec_r16(HL),
            0x3B => Self::dcd_dec_r16(SP),

            0x05 => Self::dcd_dec_r8(B),
            0x15 => Self::dcd_dec_r8(D),
            0x25 => Self::dcd_dec_r8(H),
            0x35 => Self::dcd_dec_addrr16(HL),
            0x0D => Self::dcd_dec_r8(C),
            0x1D => Self::dcd_dec_r8(E),
            0x2D => Self::dcd_dec_r8(L),
            0x3D => Self::dcd_dec_r8(A),

            0xC1 => Self::dcd_pop_r16(B, C),
            0xD1 => Self::dcd_pop_r16(D, E),
            0xE1 => Self::dcd_pop_r16(H, L),
            0xF1 => Self::dcd_pop_r16(A, F),

            0xFE => Self::dcd_cp_r8_u8(A),

            0x37 => Self::dcd_scf(),

            0x17 => Self::dcd_rla(),

            0x0F => Self::dcd_rrca(),

            0x07 => Self::dcd_rlca(),

            0x1F => Self::dcd_rra(),

            0x3F => Self::dcd_ccf(),

            0xC5 => Self::dcd_push_r16(B, C),
            0xD5 => Self::dcd_push_r16(D, E),
            0xE5 => Self::dcd_push_r16(H, L),
            0xF5 => Self::dcd_push_r16(A, F),

            0xB8 => Self::dcd_cp_r8_r8(A, B),
            0xB9 => Self::dcd_cp_r8_r8(A, C),
            0xBA => Self::dcd_cp_r8_r8(A, D),
            0xBB => Self::dcd_cp_r8_r8(A, E),
            0xBC => Self::dcd_cp_r8_r8(A, H),
            0xBD => Self::dcd_cp_r8_r8(A, L),
            0xBE => Self::dcd_cp_r8_addrr16(A, HL),
            0xBF => Self::dcd_cp_r8_r8(A, A),

            0x04 => Self::dcd_inc_r8(B),
            0x14 => Self::dcd_inc_r8(D),
            0x24 => Self::dcd_inc_r8(H),
            0x34 => Self::dcd_inc_addrr16(HL),
            0x0C => Self::dcd_inc_r8(C),
            0x1C => Self::dcd_inc_r8(E),
            0x2C => Self::dcd_inc_r8(L),
            0x3C => Self::dcd_inc_r8(A),

            0x03 => Self::dcd_inc_r16(BC),
            0x13 => Self::dcd_inc_r16(DE),
            0x23 => Self::dcd_inc_r16(HL),
            0x33 => Self::dcd_inc_r16(SP),

            0xF0 => Self::dcd_ld_r8_inaddru8(A),

            0xEA => Self::dcd_ld_addru16_r8(A),

            0x08 => Self::dcd_ld_addru16_r16(SP),

            0x06 => Self::dcd_ld_r8_u8(B),
            0x16 => Self::dcd_ld_r8_u8(D),
            0x26 => Self::dcd_ld_r8_u8(H),
            0x36 => Self::dcd_ld_addrr16_u8(HL),
            0x0E => Self::dcd_ld_r8_u8(C),
            0x1E => Self::dcd_ld_r8_u8(E),
            0x2E => Self::dcd_ld_r8_u8(L),
            0x3E => Self::dcd_ld_r8_u8(A),

            0x40 => Self::dcd_ld_r8_r8(B, B),
            0x41 => Self::dcd_ld_r8_r8(B, C),
            0x42 => Self::dcd_ld_r8_r8(B, D),
            0x43 => Self::dcd_ld_r8_r8(B, E),
            0x44 => Self::dcd_ld_r8_r8(B, H),
            0x45 => Self::dcd_ld_r8_r8(B, L),
            0x46 => Self::dcd_ld_r8_addrr16(B, HL),
            0x47 => Self::dcd_ld_r8_r8(B, A),

            0x48 => Self::dcd_ld_r8_r8(C, B),
            0x49 => Self::dcd_ld_r8_r8(C, C),
            0x4A => Self::dcd_ld_r8_r8(C, D),
            0x4B => Self::dcd_ld_r8_r8(C, E),
            0x4C => Self::dcd_ld_r8_r8(C, H),
            0x4D => Self::dcd_ld_r8_r8(C, L),
            0x4E => Self::dcd_ld_r8_addrr16(C, HL),
            0x4F => Self::dcd_ld_r8_r8(C, A),

            0x50 => Self::dcd_ld_r8_r8(D, B),
            0x51 => Self::dcd_ld_r8_r8(D, C),
            0x52 => Self::dcd_ld_r8_r8(D, D),
            0x53 => Self::dcd_ld_r8_r8(D, E),
            0x54 => Self::dcd_ld_r8_r8(D, H),
            0x55 => Self::dcd_ld_r8_r8(D, L),
            0x56 => Self::dcd_ld_r8_addrr16(D, HL),
            0x57 => Self::dcd_ld_r8_r8(D, A),

            0x58 => Self::dcd_ld_r8_r8(E, B),
            0x59 => Self::dcd_ld_r8_r8(E, C),
            0x5A => Self::dcd_ld_r8_r8(E, D),
            0x5B => Self::dcd_ld_r8_r8(E, E),
            0x5C => Self::dcd_ld_r8_r8(E, H),
            0x5D => Self::dcd_ld_r8_r8(E, L),
            0x5E => Self::dcd_ld_r8_addrr16(E, HL),
            0x5F => Self::dcd_ld_r8_r8(E, A),

            0x60 => Self::dcd_ld_r8_r8(H, B),
            0x61 => Self::dcd_ld_r8_r8(H, C),
            0x62 => Self::dcd_ld_r8_r8(H, D),
            0x63 => Self::dcd_ld_r8_r8(H, E),
            0x64 => Self::dcd_ld_r8_r8(H, H),
            0x65 => Self::dcd_ld_r8_r8(H, L),
            0x66 => Self::dcd_ld_r8_addrr16(H, HL),
            0x67 => Self::dcd_ld_r8_r8(H, A),

            0x68 => Self::dcd_ld_r8_r8(L, B),
            0x69 => Self::dcd_ld_r8_r8(L, C),
            0x6A => Self::dcd_ld_r8_r8(L, D),
            0x6B => Self::dcd_ld_r8_r8(L, E),
            0x6C => Self::dcd_ld_r8_r8(L, H),
            0x6D => Self::dcd_ld_r8_r8(L, L),
            0x6E => Self::dcd_ld_r8_addrr16(L, HL),
            0x6F => Self::dcd_ld_r8_r8(L, A),

            0x0A => Self::dcd_ld_r8_addrr16(A, BC),
            0x1A => Self::dcd_ld_r8_addrr16(A, DE),

            0x02 => Self::dcd_ld_addrr16_r8(BC, A),
            0x12 => Self::dcd_ld_addrr16_r8(DE, A),

            0x70 => Self::dcd_ld_addrr16_r8(HL, B),
            0x71 => Self::dcd_ld_addrr16_r8(HL, C),
            0x72 => Self::dcd_ld_addrr16_r8(HL, D),
            0x73 => Self::dcd_ld_addrr16_r8(HL, E),
            0x74 => Self::dcd_ld_addrr16_r8(HL, H),
            0x75 => Self::dcd_ld_addrr16_r8(HL, L),
            0x77 => Self::dcd_ld_addrr16_r8(HL, A),

            0x78 => Self::dcd_ld_r8_r8(A, B),
            0x79 => Self::dcd_ld_r8_r8(A, C),
            0x7A => Self::dcd_ld_r8_r8(A, D),
            0x7B => Self::dcd_ld_r8_r8(A, E),
            0x7C => Self::dcd_ld_r8_r8(A, H),
            0x7D => Self::dcd_ld_r8_r8(A, L),
            0x7E => Self::dcd_ld_r8_addrr16(A, HL),
            0x7F => Self::dcd_ld_r8_r8(A, A),

            0xE0 => Self::dcd_ld_inaddru8_r8(A),

            0xE2 => Self::dcd_ld_inaddrr8_r8(C, A),
            0xF2 => Self::dcd_ld_r8_inaddrr8(A, C),

            0x01 => Self::dcd_ld_r16_u16(B, C),
            0x11 => Self::dcd_ld_r16_u16(D, E),
            0x21 => Self::dcd_ld_r16_u16(H, L),
            0x31 => Self::dcd_ld_r16_u16(S, P),

            0xF9 => Self::dcd_ld_r16_r16(SP, HL),

            0x32 => Self::dcd_ld_r16d_r8(HL, A),

            0x18 => Self::dcd_jr_i8(),
            0x20 => Self::dcd_jr_c_i8(Condition::NZ),
            0x30 => Self::dcd_jr_c_i8(Condition::NC),
            0x28 => Self::dcd_jr_c_i8(Condition::Z),
            0x38 => Self::dcd_jr_c_i8(Condition::C),

            _ => (false, Err(format!("[DCD] Unknown opcode {:#04X}", byte))),
        }
    }

    pub(crate) fn decode_cb(byte: u8) -> (bool, Result<Option<Instruction>, String>) {
        match byte {
            0x30 => Self::dcd_swap_r8(B),
            0x31 => Self::dcd_swap_r8(C),
            0x32 => Self::dcd_swap_r8(D),
            0x33 => Self::dcd_swap_r8(E),
            0x34 => Self::dcd_swap_r8(H),
            0x35 => Self::dcd_swap_r8(L),
            0x36 => Self::dcd_swap_addrr16(HL),
            0x37 => Self::dcd_swap_r8(A),

            0x18 => Self::dcd_rr_r8(B),
            0x19 => Self::dcd_rr_r8(C),
            0x1A => Self::dcd_rr_r8(D),
            0x1B => Self::dcd_rr_r8(E),
            0x1C => Self::dcd_rr_r8(H),
            0x1D => Self::dcd_rr_r8(L),
            0x1E => Self::dcd_rr_addrr16(HL),
            0x1F => Self::dcd_rr_r8(A),

            0x38 => Self::dcd_srl_r8(B),
            0x39 => Self::dcd_srl_r8(C),
            0x3A => Self::dcd_srl_r8(D),
            0x3B => Self::dcd_srl_r8(E),
            0x3C => Self::dcd_srl_r8(H),
            0x3D => Self::dcd_srl_r8(L),
            0x3E => Self::dcd_srl_addrr16(HL),
            0x3F => Self::dcd_srl_r8(A),

            0x00 => Self::dcd_rlc_r8(B),
            0x01 => Self::dcd_rlc_r8(C),
            0x02 => Self::dcd_rlc_r8(D),
            0x03 => Self::dcd_rlc_r8(E),
            0x04 => Self::dcd_rlc_r8(H),
            0x05 => Self::dcd_rlc_r8(L),
            0x06 => Self::dcd_rlc_addrr16(HL),
            0x07 => Self::dcd_rlc_r8(A),

            0x20 => Self::dcd_sla_r8(B),
            0x21 => Self::dcd_sla_r8(C),
            0x22 => Self::dcd_sla_r8(D),
            0x23 => Self::dcd_sla_r8(E),
            0x24 => Self::dcd_sla_r8(H),
            0x25 => Self::dcd_sla_r8(L),
            0x26 => Self::dcd_sla_addrr16(HL),
            0x27 => Self::dcd_sla_r8(A),

            0x28 => Self::dcd_sra_r8(B),
            0x29 => Self::dcd_sra_r8(C),
            0x2A => Self::dcd_sra_r8(D),
            0x2B => Self::dcd_sra_r8(E),
            0x2C => Self::dcd_sra_r8(H),
            0x2D => Self::dcd_sra_r8(L),
            0x2E => Self::dcd_sra_addrr16(HL),
            0x2F => Self::dcd_sra_r8(A),

            0x08 => Self::dcd_rrc_r8(B),
            0x09 => Self::dcd_rrc_r8(C),
            0x0A => Self::dcd_rrc_r8(D),
            0x0B => Self::dcd_rrc_r8(E),
            0x0C => Self::dcd_rrc_r8(H),
            0x0D => Self::dcd_rrc_r8(L),
            0x0E => Self::dcd_rrc_addrr16(HL),
            0x0F => Self::dcd_rrc_r8(A),

            0x10 => Self::dcd_rl_r8(B),
            0x11 => Self::dcd_rl_r8(C),
            0x12 => Self::dcd_rl_r8(D),
            0x13 => Self::dcd_rl_r8(E),
            0x14 => Self::dcd_rl_r8(H),
            0x15 => Self::dcd_rl_r8(L),
            0x16 => Self::dcd_rl_addrr16(HL),
            0x17 => Self::dcd_rl_r8(A),

            0x40 => Self::dcd_bit_u8_r8(0, B),
            0x41 => Self::dcd_bit_u8_r8(0, C),
            0x42 => Self::dcd_bit_u8_r8(0, D),
            0x43 => Self::dcd_bit_u8_r8(0, E),
            0x44 => Self::dcd_bit_u8_r8(0, H),
            0x45 => Self::dcd_bit_u8_r8(0, L),
            0x46 => Self::dcd_bit_u8_addrr16(0, HL),
            0x47 => Self::dcd_bit_u8_r8(0, A),

            0x48 => Self::dcd_bit_u8_r8(1, B),
            0x49 => Self::dcd_bit_u8_r8(1, C),
            0x4A => Self::dcd_bit_u8_r8(1, D),
            0x4B => Self::dcd_bit_u8_r8(1, E),
            0x4C => Self::dcd_bit_u8_r8(1, H),
            0x4D => Self::dcd_bit_u8_r8(1, L),
            0x4E => Self::dcd_bit_u8_addrr16(1, HL),
            0x4F => Self::dcd_bit_u8_r8(1, A),

            0x50 => Self::dcd_bit_u8_r8(2, B),
            0x51 => Self::dcd_bit_u8_r8(2, C),
            0x52 => Self::dcd_bit_u8_r8(2, D),
            0x53 => Self::dcd_bit_u8_r8(2, E),
            0x54 => Self::dcd_bit_u8_r8(2, H),
            0x55 => Self::dcd_bit_u8_r8(2, L),
            0x56 => Self::dcd_bit_u8_addrr16(2, HL),
            0x57 => Self::dcd_bit_u8_r8(2, A),

            0x58 => Self::dcd_bit_u8_r8(3, B),
            0x59 => Self::dcd_bit_u8_r8(3, C),
            0x5A => Self::dcd_bit_u8_r8(3, D),
            0x5B => Self::dcd_bit_u8_r8(3, E),
            0x5C => Self::dcd_bit_u8_r8(3, H),
            0x5D => Self::dcd_bit_u8_r8(3, L),
            0x5E => Self::dcd_bit_u8_addrr16(3, HL),
            0x5F => Self::dcd_bit_u8_r8(3, A),

            0x60 => Self::dcd_bit_u8_r8(4, B),
            0x61 => Self::dcd_bit_u8_r8(4, C),
            0x62 => Self::dcd_bit_u8_r8(4, D),
            0x63 => Self::dcd_bit_u8_r8(4, E),
            0x64 => Self::dcd_bit_u8_r8(4, H),
            0x65 => Self::dcd_bit_u8_r8(4, L),
            0x66 => Self::dcd_bit_u8_addrr16(4, HL),
            0x67 => Self::dcd_bit_u8_r8(4, A),

            0x68 => Self::dcd_bit_u8_r8(5, B),
            0x69 => Self::dcd_bit_u8_r8(5, C),
            0x6A => Self::dcd_bit_u8_r8(5, D),
            0x6B => Self::dcd_bit_u8_r8(5, E),
            0x6C => Self::dcd_bit_u8_r8(5, H),
            0x6D => Self::dcd_bit_u8_r8(5, L),
            0x6E => Self::dcd_bit_u8_addrr16(5, HL),
            0x6F => Self::dcd_bit_u8_r8(5, A),

            0x70 => Self::dcd_bit_u8_r8(6, B),
            0x71 => Self::dcd_bit_u8_r8(6, C),
            0x72 => Self::dcd_bit_u8_r8(6, D),
            0x73 => Self::dcd_bit_u8_r8(6, E),
            0x74 => Self::dcd_bit_u8_r8(6, H),
            0x75 => Self::dcd_bit_u8_r8(6, L),
            0x76 => Self::dcd_bit_u8_addrr16(6, HL),
            0x77 => Self::dcd_bit_u8_r8(6, A),

            0x78 => Self::dcd_bit_u8_r8(7, B),
            0x79 => Self::dcd_bit_u8_r8(7, C),
            0x7A => Self::dcd_bit_u8_r8(7, D),
            0x7B => Self::dcd_bit_u8_r8(7, E),
            0x7C => Self::dcd_bit_u8_r8(7, H),
            0x7D => Self::dcd_bit_u8_r8(7, L),
            0x7E => Self::dcd_bit_u8_addrr16(7, HL),
            0x7F => Self::dcd_bit_u8_r8(7, A),

            0x80 => Self::dcd_res_u8_r8(0, B),
            0x81 => Self::dcd_res_u8_r8(0, C),
            0x82 => Self::dcd_res_u8_r8(0, D),
            0x83 => Self::dcd_res_u8_r8(0, E),
            0x84 => Self::dcd_res_u8_r8(0, H),
            0x85 => Self::dcd_res_u8_r8(0, L),
            0x86 => Self::dcd_res_u8_addrr16(0, HL),
            0x87 => Self::dcd_res_u8_r8(0, A),

            0x88 => Self::dcd_res_u8_r8(1, B),
            0x89 => Self::dcd_res_u8_r8(1, C),
            0x8A => Self::dcd_res_u8_r8(1, D),
            0x8B => Self::dcd_res_u8_r8(1, E),
            0x8C => Self::dcd_res_u8_r8(1, H),
            0x8D => Self::dcd_res_u8_r8(1, L),
            0x8E => Self::dcd_res_u8_addrr16(1, HL),
            0x8F => Self::dcd_res_u8_r8(1, A),

            0x90 => Self::dcd_res_u8_r8(2, B),
            0x91 => Self::dcd_res_u8_r8(2, C),
            0x92 => Self::dcd_res_u8_r8(2, D),
            0x93 => Self::dcd_res_u8_r8(2, E),
            0x94 => Self::dcd_res_u8_r8(2, H),
            0x95 => Self::dcd_res_u8_r8(2, L),
            0x96 => Self::dcd_res_u8_addrr16(2, HL),
            0x97 => Self::dcd_res_u8_r8(2, A),

            0x98 => Self::dcd_res_u8_r8(3, B),
            0x99 => Self::dcd_res_u8_r8(3, C),
            0x9A => Self::dcd_res_u8_r8(3, D),
            0x9B => Self::dcd_res_u8_r8(3, E),
            0x9C => Self::dcd_res_u8_r8(3, H),
            0x9D => Self::dcd_res_u8_r8(3, L),
            0x9E => Self::dcd_res_u8_addrr16(3, HL),
            0x9F => Self::dcd_res_u8_r8(3, A),

            0xA0 => Self::dcd_res_u8_r8(4, B),
            0xA1 => Self::dcd_res_u8_r8(4, C),
            0xA2 => Self::dcd_res_u8_r8(4, D),
            0xA3 => Self::dcd_res_u8_r8(4, E),
            0xA4 => Self::dcd_res_u8_r8(4, H),
            0xA5 => Self::dcd_res_u8_r8(4, L),
            0xA6 => Self::dcd_res_u8_addrr16(4, HL),
            0xA7 => Self::dcd_res_u8_r8(4, A),

            0xA8 => Self::dcd_res_u8_r8(5, B),
            0xA9 => Self::dcd_res_u8_r8(5, C),
            0xAA => Self::dcd_res_u8_r8(5, D),
            0xAB => Self::dcd_res_u8_r8(5, E),
            0xAC => Self::dcd_res_u8_r8(5, H),
            0xAD => Self::dcd_res_u8_r8(5, L),
            0xAE => Self::dcd_res_u8_addrr16(5, HL),
            0xAF => Self::dcd_res_u8_r8(5, A),

            0xB0 => Self::dcd_res_u8_r8(6, B),
            0xB1 => Self::dcd_res_u8_r8(6, C),
            0xB2 => Self::dcd_res_u8_r8(6, D),
            0xB3 => Self::dcd_res_u8_r8(6, E),
            0xB4 => Self::dcd_res_u8_r8(6, H),
            0xB5 => Self::dcd_res_u8_r8(6, L),
            0xB6 => Self::dcd_res_u8_addrr16(6, HL),
            0xB7 => Self::dcd_res_u8_r8(6, A),

            0xB8 => Self::dcd_res_u8_r8(7, B),
            0xB9 => Self::dcd_res_u8_r8(7, C),
            0xBA => Self::dcd_res_u8_r8(7, D),
            0xBB => Self::dcd_res_u8_r8(7, E),
            0xBC => Self::dcd_res_u8_r8(7, H),
            0xBD => Self::dcd_res_u8_r8(7, L),
            0xBE => Self::dcd_res_u8_addrr16(7, HL),
            0xBF => Self::dcd_res_u8_r8(7, A),

            0xC0 => Self::dcd_set_u8_r8(0, B),
            0xC1 => Self::dcd_set_u8_r8(0, C),
            0xC2 => Self::dcd_set_u8_r8(0, D),
            0xC3 => Self::dcd_set_u8_r8(0, E),
            0xC4 => Self::dcd_set_u8_r8(0, H),
            0xC5 => Self::dcd_set_u8_r8(0, L),
            0xC6 => Self::dcd_set_u8_addrr16(0, HL),
            0xC7 => Self::dcd_set_u8_r8(0, A),

            0xC8 => Self::dcd_set_u8_r8(1, B),
            0xC9 => Self::dcd_set_u8_r8(1, C),
            0xCA => Self::dcd_set_u8_r8(1, D),
            0xCB => Self::dcd_set_u8_r8(1, E),
            0xCC => Self::dcd_set_u8_r8(1, H),
            0xCD => Self::dcd_set_u8_r8(1, L),
            0xCE => Self::dcd_set_u8_addrr16(1, HL),
            0xCF => Self::dcd_set_u8_r8(1, A),

            0xD0 => Self::dcd_set_u8_r8(2, B),
            0xD1 => Self::dcd_set_u8_r8(2, C),
            0xD2 => Self::dcd_set_u8_r8(2, D),
            0xD3 => Self::dcd_set_u8_r8(2, E),
            0xD4 => Self::dcd_set_u8_r8(2, H),
            0xD5 => Self::dcd_set_u8_r8(2, L),
            0xD6 => Self::dcd_set_u8_addrr16(2, HL),
            0xD7 => Self::dcd_set_u8_r8(2, A),

            0xD8 => Self::dcd_set_u8_r8(3, B),
            0xD9 => Self::dcd_set_u8_r8(3, C),
            0xDA => Self::dcd_set_u8_r8(3, D),
            0xDB => Self::dcd_set_u8_r8(3, E),
            0xDC => Self::dcd_set_u8_r8(3, H),
            0xDD => Self::dcd_set_u8_r8(3, L),
            0xDE => Self::dcd_set_u8_addrr16(3, HL),
            0xDF => Self::dcd_set_u8_r8(3, A),

            0xE0 => Self::dcd_set_u8_r8(4, B),
            0xE1 => Self::dcd_set_u8_r8(4, C),
            0xE2 => Self::dcd_set_u8_r8(4, D),
            0xE3 => Self::dcd_set_u8_r8(4, E),
            0xE4 => Self::dcd_set_u8_r8(4, H),
            0xE5 => Self::dcd_set_u8_r8(4, L),
            0xE6 => Self::dcd_set_u8_addrr16(4, HL),
            0xE7 => Self::dcd_set_u8_r8(4, A),

            0xE8 => Self::dcd_set_u8_r8(5, B),
            0xE9 => Self::dcd_set_u8_r8(5, C),
            0xEA => Self::dcd_set_u8_r8(5, D),
            0xEB => Self::dcd_set_u8_r8(5, E),
            0xEC => Self::dcd_set_u8_r8(5, H),
            0xED => Self::dcd_set_u8_r8(5, L),
            0xEE => Self::dcd_set_u8_addrr16(5, HL),
            0xEF => Self::dcd_set_u8_r8(5, A),

            0xF0 => Self::dcd_set_u8_r8(6, B),
            0xF1 => Self::dcd_set_u8_r8(6, C),
            0xF2 => Self::dcd_set_u8_r8(6, D),
            0xF3 => Self::dcd_set_u8_r8(6, E),
            0xF4 => Self::dcd_set_u8_r8(6, H),
            0xF5 => Self::dcd_set_u8_r8(6, L),
            0xF6 => Self::dcd_set_u8_addrr16(6, HL),
            0xF7 => Self::dcd_set_u8_r8(6, A),

            0xF8 => Self::dcd_set_u8_r8(7, B),
            0xF9 => Self::dcd_set_u8_r8(7, C),
            0xFA => Self::dcd_set_u8_r8(7, D),
            0xFB => Self::dcd_set_u8_r8(7, E),
            0xFC => Self::dcd_set_u8_r8(7, H),
            0xFD => Self::dcd_set_u8_r8(7, L),
            0xFE => Self::dcd_set_u8_addrr16(7, HL),
            0xFF => Self::dcd_set_u8_r8(7, A),

            _ => (false, Err(format!("[DCD CB] Unknown opcode {:#04X}", byte))),
        }
    }

    fn dcd_ld_r16_spi8(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Ld, Reg16(r16), FetchSPI8), None)
        )))
    }

    fn dcd_add_r16_i8(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Add, Reg16(r16), FetchI8), None)
        )))
    }

    fn dcd_ei() -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(
            Instruction::new((Ei, Non, Non), None)
        )))
    }

    fn dcd_halt() -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Halt, Non, Non), None)
        )))
    }

    fn dcd_swap_r8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Swap, Reg8(r8), Non), None,
        ))))
    }

    fn dcd_swap_addrr16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Swap, AddrReg16(r16), Non), None,
        ))))
    }

    fn dcd_rr_r8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Rr, Reg8(r8), Non), None,
        ))))
    }

    fn dcd_rr_addrr16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Rr, AddrReg16(r16), Non), None,
        ))))
    }

    fn dcd_nop() -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Nop, Non, Non), None,
        ))))
    }
    fn dcd_di() -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Di, Non, Non), None,
        ))))
    }

    fn dcd_cpl() -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Cpl, Non, Non), None,
        ))))
    }

    fn dcd_daa() -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Daa, Non, Non), None,
        ))))
    }

    fn dcd_dec_r8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Dec, Reg8(r8), Non), None,
        ))))
    }

    fn dcd_dec_r16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Dec, Reg16(r16), Non), None,
        ))))
    }

    fn dcd_dec_addrr16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Dec, AddrReg16(r16), Non), None)
        )))
    }

    fn dcd_inc_addrr16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Inc, AddrReg16(r16), Non), None)
        )))
    }

    fn dcd_scf() -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Scf, Non, Non), None,
        ))))
    }

    fn dcd_rla() -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Rla, Non, Non), None,
        ))))
    }

    fn dcd_rrca() -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Rrca, Non, Non), None,
        ))))
    }

    fn dcd_rlca() -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Rlca, Non, Non), None,
        ))))
    }

    fn dcd_rra() -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Rra, Non, Non), None,
        ))))
    }

    fn dcd_ccf() -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Ccf, Non, Non), None,
        ))))
    }

    fn dcd_rl_addrr16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Rl, AddrReg16(r16), Non), None,
        ))))
    }

    fn dcd_rlc_addr16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Rlc, AddrReg16(r16), Non), None,
        ))))
    }

    fn dcd_rlc_addrr16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Rlc, AddrReg16(r16), Non), None,
        ))))
    }

    fn dcd_rlc_r8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Rlc, Reg8(r8), Non), None,
        ))))
    }

    fn dcd_sla_addrr16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Sla, AddrReg16(r16), Non), None,
        ))))
    }

    fn dcd_sla_r8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Sla, Reg8(r8), Non), None,
        ))))
    }

    fn dcd_sra_addrr16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Sra, AddrReg16(r16), Non), None,
        ))))
    }

    fn dcd_sra_r8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Sra, Reg8(r8), Non), None,
        ))))
    }

    fn dcd_rrc_addrr16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Rrc, AddrReg16(r16), Non), None,
        ))))
    }

    fn dcd_rrc_r8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Rrc, Reg8(r8), Non), None,
        ))))
    }

    fn dcd_srl_addrr16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Srl, AddrReg16(r16), Non), None,
        ))))
    }

    fn dcd_srl_r8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Srl, Reg8(r8), Non), None,
        ))))
    }

    fn dcd_rl_r8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Rl, Reg8(r8), Non), None,
        ))))
    }

    fn dcd_pop_r16(r81: R8, r82: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(
            Some(Instruction::new(
                (Ld, Reg8(r82), AddrReg16i(SP)),
                Some(Box::new(Instruction::new(
                    (Ld, Reg8(r81), AddrReg16i(SP)),
                    None,
                ))),
            ))))
    }

    fn dcd_push_r16(r81: R8, r82: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Internal, Non, Non),
            Some(Box::new(Instruction::new(
                (Ld, AddrRegd16(SP), Reg8(r81)),
                Some(Box::new(Instruction::new(
                    (Ld, AddrRegd16(SP), Reg8(r82)),
                    None,
                ))),
            )))))))
    }

    fn dcd_cp_r8_r8(r81: R8, r82: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Cp, Reg8(r81), Reg8(r82)), None,
        ))))
    }

    fn dcd_sub_r8_r8(r81: R8, r82: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Sub, Reg8(r81), Reg8(r82)), None,
        ))))
    }

    fn dcd_and_r8_r8(r81: R8, r82: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (And, Reg8(r81), Reg8(r82)), None,
        ))))
    }

    fn dcd_sbc_r8_r8(r81: R8, r82: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Sbc, Reg8(r81), Reg8(r82)), None,
        ))))
    }

    fn dcd_adc_r8_r8(r81: R8, r82: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Adc, Reg8(r81), Reg8(r82)), None,
        ))))
    }

    fn dcd_add_r8_r8(r81: R8, r82: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Add, Reg8(r81), Reg8(r82)), None,
        ))))
    }

    fn dcd_add_r16_r16(r161: R16, r162: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Add, Reg16(r161), Reg16(r162)), None,
        ))))
    }

    fn dcd_cp_r8_addrr16(r8: R8, r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Cp, Reg8(r8), AddrReg16(r16)), None,
        ))))
    }

    fn dcd_cp_r8_u8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Cp, Reg8(r8), FetchU8), None,
        ))))
    }

    fn dcd_call_c_u16(c: Condition) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Call, CC(c), FetchU16(None, None)), None,
        ))))
    }

    fn dcd_call_u16() -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Call, FetchU16(None, None), Non), None,
        ))))
    }

    fn dcd_ret() -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(
            Some(Instruction::new(
                (Ld, Reg8(PCl), AddrReg16i(SP)),
                Some(Box::new(Instruction::new(
                    (Ld, Reg8(PCh), AddrReg16i(SP)),
                    Some(Box::new(Instruction::new((Internal, Non, Non), None))),
                ))),
            ))))
    }

    fn dcd_rst(u16: u16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new((Rst, U16(u16.into()), Non, ), None))))
    }

    fn dcd_reti() -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(
            Some(Instruction::new(
                (Ld, Reg8(PCl), AddrReg16i(SP)),
                Some(Box::new(Instruction::new(
                    (Ld, Reg8(PCh), AddrReg16i(SP)),
                    Some(Box::new(Instruction::new(
                        (EiImm, Non, Non), None,
                    ))),
                ))),
            ))))
    }

    fn dcd_ret_c(c: Condition) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ret, CC(c), Non), None,
        ))))
    }

    fn dcd_inc_r8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Inc, Reg8(r8), Non), None,
        ))))
    }

    fn dcd_inc_r16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Inc, Reg16(r16), Non), None,
        ))))
    }

    fn dcd_jp_u16() -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Jp, FetchU16(None, None), Non), None)
        )))
    }

    fn dcd_jp_c_u16(c: Condition) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Jp, CC(c), FetchU16(None, None)), None)
        )))
    }

    fn dcd_jp_r16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(
            Instruction::new((Jp, Reg16(r16), Non), None)
        )))
    }

    fn dcd_jr_i8() -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Jr, FetchI8, Non), None)
        )))
    }

    fn dcd_jr_c_i8(c: Condition) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Jr, CC(c), FetchI8), None)
        )))
    }
    fn dcd_res_u8_addrr16(u8: u8, r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Res, U8(u8.into()), AddrReg16(r16)), None)
        )))
    }

    fn dcd_res_u8_r8(u8: u8, r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(
            Instruction::new((Res, U8(u8.into()), Reg8(r8)), None)
        )))
    }
    fn dcd_set_u8_addrr16(u8: u8, r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Set, U8(u8.into()), AddrReg16(r16)), None)
        )))
    }

    fn dcd_set_u8_r8(u8: u8, r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(
            Instruction::new((Set, U8(u8.into()), Reg8(r8)), None)
        )))
    }

    fn dcd_bit_u8_addrr16(u8: u8, r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Bit, U8(u8.into()), AddrReg16(r16)), None)
        )))
    }

    fn dcd_bit_u8_r8(u8: u8, r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(
            Instruction::new((Bit, U8(u8.into()), Reg8(r8)), None)
        )))
    }

    fn dcd_decode_cb() -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((DcdCB, FetchU8, Non), None)
        )))
    }
    fn dcd_ld_r16d_r8(r16: R16, r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Ld, AddrReg16d(r16), Reg8(r8)), None)
        )))
    }

    fn dcd_and_r8_u8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((And, Reg8(r8), FetchU8), None)
        )))
    }

    fn dcd_xor_r8_u8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Xor, Reg8(r8), FetchU8), None)
        )))
    }

    fn dcd_add_r8_u8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Add, Reg8(r8), FetchU8), None)
        )))
    }

    fn dcd_adc_r8_u8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Adc, Reg8(r8), FetchU8), None)
        )))
    }

    fn dcd_sub_r8_u8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Sub, Reg8(r8), FetchU8), None)
        )))
    }

    fn dcd_sbc_r8_u8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Sbc, Reg8(r8), FetchU8), None)
        )))
    }

    fn dcd_or_r8_u8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Or, Reg8(r8), FetchU8), None)
        )))
    }

    fn dcd_or_r8_r8(r81: R8, r82: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(
            Instruction::new((Or, Reg8(r81), Reg8(r82)), None)
        )))
    }

    fn dcd_xor_r8_r8(r81: R8, r82: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(
            Instruction::new((Xor, Reg8(r81), Reg8(r82)), None)
        )))
    }

    fn dcd_xor_r8_addrr16(r8: R8, r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Xor, Reg8(r8), AddrReg16(r16)), None)
        )))
    }

    fn dcd_or_r8_addrr16(r8: R8, r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Or, Reg8(r8), AddrReg16(r16)), None)
        )))
    }

    fn dcd_add_r8_addrr16(r8: R8, r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Add, Reg8(r8), AddrReg16(r16)), None)
        )))
    }

    fn dcd_adc_r8_addrr16(r8: R8, r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Adc, Reg8(r8), AddrReg16(r16)), None)
        )))
    }

    fn dcd_sbc_r8_addrr16(r8: R8, r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Sbc, Reg8(r8), AddrReg16(r16)), None)
        )))
    }

    fn dcd_and_r8_addrr16(r8: R8, r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((And, Reg8(r8), AddrReg16(r16)), None)
        )))
    }

    fn dcd_sub_r8_addrr16(r8: R8, r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(
            Instruction::new((Sub, Reg8(r8), AddrReg16(r16)), None)
        )))
    }

    fn dcd_ld_addrr16_r8(r16: R16, r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ld, AddrReg16(r16), Reg8(r8)), None,
        ))))
    }

    fn dcd_ld_addrr16i_r8(r16: R16, r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ld, AddrReg16i(r16), Reg8(r8)), None,
        ))))
    }

    fn dcd_ld_r8_addrr16i(r8: R8, r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ld, Reg8(r8), AddrReg16i(r16)), None,
        ))))
    }

    fn dcd_ld_r8_addrr16d(r8: R8, r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ld, Reg8(r8), AddrReg16d(r16)), None,
        ))))
    }

    fn dcd_ld_r8_addru16(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ld, Reg8(r8), FetchAddrU16(None, None)), None,
        ))))
    }

    fn dcd_ld_r8_addrr16(r8: R8, r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ld, Reg8(r8), AddrReg16(r16)), None,
        ))))
    }

    fn dcd_ld_r8_inaddru8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ld, Reg8(r8), FetchInAddrU8), None,
        ))))
    }

    fn dcd_ld_addru16_r8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ld, FetchAddrU16(None, None), Reg8(r8)), None,
        ))))
    }

    fn dcd_ld_addru16_r16(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ld, FetchAddrU16(None, None), Reg16(r16)), None,
        ))))
    }

    fn dcd_ld_addrr16_u8(r16: R16) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ld, AddrReg16(r16), FetchU8), None,
        ))))
    }

    fn dcd_ld_r8_inaddrr8(r81: R8, r82: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ld, Reg8(r81), InAddrReg8(r82)), None,
        ))))
    }

    fn dcd_ld_inaddrr8_r8(r81: R8, r82: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ld, InAddrReg8(r81), Reg8(r82)), None,
        ))))
    }

    fn dcd_ld_inaddru8_r8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ld, FetchInAddrU8, Reg8(r8)), None,
        ))))
    }

    fn dcd_ld_r8_r8(r81: R8, r82: R8) -> (bool, Result<Option<Instruction>, String>) {
        (true, Ok(Some(Instruction::new(
            (Ld, Reg8(r81), Reg8(r82)), None,
        ))))
    }

    fn dcd_ld_r8_u8(r8: R8) -> (bool, Result<Option<Instruction>, String>) {
        (false, Ok(Some(Instruction::new(
            (Ld, Reg8(r8), FetchU8), None,
        ))))
    }

    fn dcd_ld_r16_r16(r161: R16, r162: R16) -> (bool, Result<Option<Instruction>, String>) {
        let ins = Instruction::new(
            (Ld, Reg16(r161), Reg16(r162)), None,
        );
        (false, Ok(Some(ins)))
    }

    fn dcd_ld_r16_u16(r81: R8, r82: R8) -> (bool, Result<Option<Instruction>, String>) {
        let ins = Instruction::new(
            (Ld, Reg8(r82), FetchU8),
            Some(Box::new(Instruction::new(
                (Ld, Reg8(r81), FetchU8),
                None,
            ))),
        );
        (false, Ok(Some(ins)))
    }
}