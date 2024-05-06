mod cpu_registers;

use std::default;

use self::cpu_registers::CPURegisters;
use super::instructions::Instruction;

pub struct CPU {
    instructions: [Instruction; 255],
    registers: CPURegisters,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            instructions: todo!(),
            registers: CPURegisters::default(),
        }
    }
}
