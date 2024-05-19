pub mod cpu_registers;

use self::cpu_registers::CPURegisters;
use super::{
    instructions::{self, Instruction, Operands, Ret, Word},
    memory::Memory,
};

pub struct CPU {
    registers: CPURegisters,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            registers: CPURegisters::default(),
        }
    }
}

impl CPU {
    pub fn execute(&mut self, mem: &mut Memory) {
        instructions::execute_instruction(&mut self.registers, mem);
    }
}
