pub mod cpu_registers;

use std::fs;

use self::cpu_registers::CPURegisters;
use super::{
    instructions::{self, Instruction, InstructionData, Operands, Ret, Word},
    memory::Memory,
};

pub struct CPU {
    registers: CPURegisters,
    instructions: Vec<Instruction>,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            registers: CPURegisters::default(),
            instructions: vec![],
        }
    }
}

impl CPU {
    pub fn load_instructions(&mut self) {
        self.instructions = instructions::fetch_instructions();
    }
    pub fn execute(&mut self, memory: &mut Memory) {
        let opcode = memory.read_u8(self.registers.pc);
        instructions::execute_instruction(
            &self.instructions[opcode as usize],
            &mut self.registers,
            memory,
        );
    }
}
