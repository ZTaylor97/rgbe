use num_traits::{
    ops::overflowing::{OverflowingAdd, OverflowingSub},
    WrappingAdd, WrappingSub,
};

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
                    "Incorrect words {:?} , {:?} passed to jump function",
                    word1, word2
                )));
            }
        }
    } else {
        return Err(InstructionError::InvalidOperandsError(operands));
    }
}

pub fn jr(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Two(target, source, flags) = operands {
        match (target, source) {
            (Word::U16Mut(target), Word::U8(source)) => {
                if let Some(condition) = branch_args.condition {
                    if jp_check_condition(*flags.unwrap(), condition) {
                        let val = target.wrapping_add_signed((source as i8) as i16);
                        *target = val;

                        Ok(branch_args.cycles[0])
                    } else {
                        Ok(branch_args.cycles[1])
                    }
                } else {
                    let val = target.wrapping_add_signed((source as i8) as i16);
                    *target = val;
                    Ok(branch_args.cycles[0])
                }
            }
            (word1, word2) => {
                return Err(InstructionError::IncorrectOperandsError(format!(
                    "Incorrect words {:?} , {:?} passed to jump function",
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

pub fn get_jump_operands<'a>(
    registers: &'a mut CPURegisters,
    mem: &'a mut Memory,
    opcode: u8,
    value: Option<Ret>,
) -> Result<(Operands<'a>, Option<u8>), InstructionError<'a>> {
    let hi = (opcode & 0xF0) >> 4;
    let lo = opcode & 0x0F;

    let reg_copy = registers.clone();

    let hl = registers.get_hl();
    let mem_hl_val = mem.read_u8(hl).clone();

    let dest = Word::U16Mut(&mut registers.pc);

    match opcode {
        0xC2 => {
            let address = if let Some(Ret::U16(address)) = value {
                address
            } else {
                return Err(InstructionError::InvalidLiteral(value.unwrap()));
            };
            Ok((
                Operands::Two(dest, Word::U16(address), None),
                Some(0b1100_0000),
            ))
        }

        _ => return Err(InstructionError::UnimplementedError(opcode)),
    }
}
