#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use valida_derive::Machine;
use valida_machine::instruction::Instruction;
use valida_machine::Machine;

pub struct ALU32Chip {
    pub operations: Vec<()>,
}

pub trait MachineWithALU32: Machine {
    fn alu_32(&self) -> &ALU32Chip;
    fn alu_32_mut(&mut self) -> &mut ALU32Chip;
}

pub struct Add32Instruction;

impl<M: MachineWithALU32> Instruction<M> for Add32Instruction {
    const OPCODE: u32 = 123;

    fn execute(state: &mut M) {
        state.alu_32_mut().operations.push(());
    }
}

#[derive(Machine)]
pub struct BasicMachine {
    #[instruction]
    add32: Add32Instruction,

    #[chip]
    alu_32: ALU32Chip,
}

impl MachineWithALU32 for BasicMachine {
    fn alu_32(&self) -> &ALU32Chip {
        &self.alu_32
    }
    fn alu_32_mut(&mut self) -> &mut ALU32Chip {
        &mut self.alu_32
    }
}
