use crate::cpu::op::Op;
use crate::cpu::op::Op::Nop;
use crate::cpu::op_arg::OpArg;
use crate::cpu::op_arg::OpArg::Non;
use crate::cpu::registers::R8;

#[derive(Debug)]
pub struct Instruction {
    ins: (Op, OpArg, OpArg),
    next: Option<Box<Instruction>>,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            ins: (Nop, Non, Non),
            next: None,
        }
    }
}

impl Instruction {
    pub fn new(ins: (Op, OpArg, OpArg), next: Option<Box<Instruction>>) -> Self {
        Self { ins, next }
    }
    pub fn ins(&self) -> (Op, OpArg, OpArg) {
        self.ins
    }
    pub fn set_ins(&mut self, ins: (Op, OpArg, OpArg)) {
        self.ins = ins;
    }
    pub fn next(&self) -> &Option<Box<Instruction>> {
        &self.next
    }

    pub fn next_mut(&mut self) -> &mut Option<Box<Instruction>> {
        &mut self.next
    }
}