use num_traits::{ops::overflowing::OverflowingSub, WrappingAdd, WrappingSub};

use super::utils::{BranchArgs, InstructionError, Operands, Ret, Word};
use crate::emulator::{
    cpu::cpu_registers::CPURegisters,
    memory::{Memory, U16Wrapper},
};

pub fn jp(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Two(target, source, flags) = operands {
        match (target, source) {
            (Word::U16Mut(target), Word::U16(source)) => {
                if let Some(condition) = branch_args.condition {
                    if jp_check_condition(*flags.unwrap(), condition) {
                        *target = source;

                        Ok(branch_args.cycles[0])
                    } else {
                        Ok(branch_args.cycles[1])
                    }
                } else {
                    *target = source;
                    Ok(branch_args.cycles[0])
                }
            }
            (word1, word2) => {
                return Err(InstructionError::IncorrectOperandsError(format!(
                    "Incorrect words {:?} , {:?} passed to add function add",
                    word1, word2
                )));
            }
        }
    } else {
        return Err(InstructionError::InvalidOperandsError(operands));
    }
}

pub fn jp_check_condition(flags: u8, condition: u8) -> bool {
    (flags & condition) == condition
}
