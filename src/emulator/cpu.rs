mod cpu_registers;

use num_traits::NumAssignRef;
use std::default;

use self::cpu_registers::CPURegisters;
use super::instructions::Instruction;

pub struct CPU<T: NumAssignRef> {
    instructions: [Instruction<T>; 255],
    registers: CPURegisters,
}

impl<T: NumAssignRef> Default for CPU<T> {
    fn default() -> Self {
        Self {
            instructions: todo!(),
            registers: CPURegisters::default(),
        }
    }
}
