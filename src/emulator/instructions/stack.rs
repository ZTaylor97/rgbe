use num_traits::{
    ops::overflowing::{OverflowingAdd, OverflowingSub},
    WrappingAdd, WrappingSub,
};

use super::utils::{check_condition, Args, BranchArgs, InstructionError, Operands, Ret, Word};
use crate::emulator::{
    cpu::cpu_registers::{convert_u16_to_two_u8s, CPURegisters},
    memory::{Memory, U16Wrapper},
};

pub fn push_pop(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Two(target, source, flags) = operands {
        match (target, source) {
            (Word::U16WrapperMut(target), Word::U16WrapperMut(source)) => {
                *target.0 = *source.1;
                *target.1 = *source.0;
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
    Ok(branch_args.cycles[0])
}

pub fn ret(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Ret(pc, stack, sp, flags) = operands {
        match (pc, stack, sp) {
            (Word::U16Mut(pc), Word::U16WrapperMut(stack), Word::U16Mut(sp)) => {
                if let Some(condition) = branch_args.condition {
                    if check_condition(*flags.unwrap(), condition) {
                        let new_source = U16Wrapper(stack.1, stack.0);
                        *pc = new_source.into_u16();
                        *sp += 2;
                        Ok(branch_args.cycles[0])
                    } else {
                        Ok(branch_args.cycles[1])
                    }
                } else {
                    let new_source = U16Wrapper(stack.1, stack.0);
                    *pc = new_source.into_u16();
                    *sp += 2;
                    Ok(branch_args.cycles[0])
                }
            }
            (word1, word2, word3) => {
                return Err(InstructionError::IncorrectOperandsError(format!(
                    "Incorrect words {:?} , {:?}, {:?} passed to jump function",
                    word1, word2, word3
                )));
            }
        }
    } else {
        return Err(InstructionError::InvalidOperandsError(operands));
    }
}

pub fn call(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Call(stack, pc, sp, address, flags) = operands {
        match (stack, pc, sp, address) {
            (
                Word::U16WrapperMut(stack),
                Word::U16Mut(pc),
                Word::U16Mut(sp),
                Word::U16(address),
            ) => {
                if let Some(condition) = branch_args.condition {
                    if check_condition(*flags.unwrap(), condition) {
                        let split_pc = convert_u16_to_two_u8s(*pc);
                        *stack.0 = split_pc.1;
                        *stack.1 = split_pc.0;
                        *sp -= 2;
                        *pc = address;
                        Ok(branch_args.cycles[0])
                    } else {
                        Ok(branch_args.cycles[1])
                    }
                } else {
                    let split_pc = convert_u16_to_two_u8s(*pc);
                    *stack.0 = split_pc.1;
                    *stack.1 = split_pc.0;
                    *sp -= 2;
                    *pc = address;
                    Ok(branch_args.cycles[0])
                }
            }
            (word1, word2, word3, word4) => {
                return Err(InstructionError::IncorrectOperandsError(format!(
                    "Incorrect words {:?} , {:?}, {:?}, {:?} passed to jump function",
                    word1, word2, word3, word4
                )));
            }
        }
    } else {
        return Err(InstructionError::InvalidOperandsError(operands));
    }
}

pub fn get_stack_operands<'a>(
    registers: &'a mut CPURegisters,
    mem: &'a mut Memory,
    opcode: u8,
    value: Option<Ret>,
) -> Result<Args<'a>, InstructionError<'a>> {
    let hi = (opcode & 0xF0) >> 4;
    let lo = opcode & 0x0F;

    let source = match hi {
        0xC => U16Wrapper(&mut registers.b, &mut registers.c),
        0xD => U16Wrapper(&mut registers.d, &mut registers.e),
        0xE => U16Wrapper(&mut registers.h, &mut registers.l),
        0xF => U16Wrapper(&mut registers.a, &mut registers.f),
        _ => return Err(InstructionError::UnimplementedError(opcode)),
    };

    let ops = match lo {
        // push
        0x5 => {
            registers.sp -= 2;
            Operands::Two(
                Word::U16WrapperMut(mem.read_u16wrapper(registers.sp)),
                Word::U16WrapperMut(source),
                None,
            )
        }
        // pop
        0x1 => {
            let ops = Operands::Two(
                Word::U16WrapperMut(source),
                Word::U16WrapperMut(mem.read_u16wrapper(registers.sp)),
                None,
            );
            registers.sp += 2;
            ops
        }
        _ => return Err(InstructionError::UnimplementedError(opcode)),
    };
    Ok((ops, None))
}

pub fn get_ret_operands<'a>(
    registers: &'a mut CPURegisters,
    mem: &'a mut Memory,
    opcode: u8,
    value: Option<Ret>,
) -> Result<Args<'a>, InstructionError<'a>> {
    let ops = Operands::Ret(
        Word::U16Mut(&mut registers.pc),
        Word::U16WrapperMut(mem.read_u16wrapper(registers.sp)),
        Word::U16Mut(&mut registers.sp),
        Some(&mut registers.f),
    );
    let condition = match opcode {
        0xC0 => Some(0b1100_0000), // RET NZ
        0xC8 => Some(0b1000_0000), // RET Z
        0xC9 => None,              // RET
        0xD0 => Some(0b0101_0000), // RET NC
        0xD8 => Some(0b0001_0000), // RET C
        0xD9 => None,              // RETI
        _ => return Err(InstructionError::UnimplementedError(opcode)),
    };
    Ok((ops, condition))
}
pub fn get_call_operands<'a>(
    registers: &'a mut CPURegisters,
    mem: &'a mut Memory,
    opcode: u8,
    value: Option<Ret>,
) -> Result<Args<'a>, InstructionError<'a>> {
    let address = if let Some(Ret::U16(address)) = value {
        address
    } else {
        return Err(InstructionError::InvalidLiteral(value.unwrap()));
    };

    let ops = Operands::Call(
        Word::U16WrapperMut(mem.read_u16wrapper(registers.sp - 2)),
        Word::U16Mut(&mut registers.pc),
        Word::U16Mut(&mut registers.sp),
        Word::U16(address), // jump here
        Some(&mut registers.f),
    );
    let condition = match opcode {
        0xC4 => Some(0b1100_0000), // CALL NZ
        0xCC => Some(0b1000_0000), // CALL Z
        0xCD => None,              // CALL
        0xD4 => Some(0b0101_0000), // CALL NC
        0xDC => Some(0b0001_0000), // CALL C
        _ => return Err(InstructionError::UnimplementedError(opcode)),
    };
    Ok((ops, condition))
}

#[cfg(test)]
mod stack_instruction_tests {
    use crate::emulator::{
        cpu::cpu_registers::{convert_two_u8s_to_u16, convert_u16_to_two_u8s},
        instructions::*,
        memory::U16Wrapper,
    };
    use utils::Word;

    #[test]
    fn test_push_pop() {
        let source = 30000;
        let mut expected_values = convert_u16_to_two_u8s(source);
        let mut target = (100, 200);

        let instruction = Instruction {
            data: InstructionData::default(),
            func: push_pop,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };

        instruction.exec(
            Operands::Two(
                Word::U16WrapperMut(U16Wrapper(&mut target.0, &mut target.1)),
                Word::U16WrapperMut(U16Wrapper(&mut expected_values.0, &mut expected_values.1)),
                None,
            ),
            branch_args,
        );

        assert_eq!(target, (expected_values.1, expected_values.0))
    }
    #[test]
    fn test_ret() {
        let source = 30000;
        let mut expected_values = convert_u16_to_two_u8s(source);
        let mut target = 100;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: ret,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };

        let mut stack_pointer = 0;

        instruction.exec(
            Operands::Ret(
                Word::U16Mut(&mut target),
                Word::U16WrapperMut(U16Wrapper(&mut expected_values.0, &mut expected_values.1)),
                Word::U16Mut(&mut stack_pointer),
                None,
            ),
            branch_args,
        );

        assert_eq!(
            target,
            convert_two_u8s_to_u16(expected_values.1, expected_values.0)
        );

        assert_eq!(stack_pointer, 2);
    }
    #[test]
    fn test_ret_condition_true() {
        let source = 30000;
        let mut expected_values = convert_u16_to_two_u8s(source);
        let mut target = 100;

        let mut flags = 0b1100_0000;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: ret,
        };
        let cycles = vec![16, 4];
        let branch_args = BranchArgs {
            cycles: cycles.clone(),
            condition: Some(flags), // condition is same as flags
        };

        let mut stack_pointer = 0;

        let result_cycles = instruction.exec(
            Operands::Ret(
                Word::U16Mut(&mut target),
                Word::U16WrapperMut(U16Wrapper(&mut expected_values.0, &mut expected_values.1)),
                Word::U16Mut(&mut stack_pointer),
                Some(&mut flags), // condition is same as flags
            ),
            branch_args,
        );

        assert_eq!(
            target,
            convert_two_u8s_to_u16(expected_values.1, expected_values.0)
        );

        assert_eq!(stack_pointer, 2);
        assert_eq!(result_cycles, cycles[0]);
    }
    #[test]
    fn test_ret_condition_false() {
        let source = 30000;
        let mut expected_values = convert_u16_to_two_u8s(source);
        let answer = 100;
        let mut target = answer;

        let mut flags = 0b1100_0000;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: ret,
        };
        let cycles = vec![16, 4];
        let branch_args = BranchArgs {
            cycles: cycles.clone(),
            condition: Some(0b1111_0000), // condition is same as flags
        };

        let mut stack_pointer = 0;

        let result_cycles = instruction.exec(
            Operands::Ret(
                Word::U16Mut(&mut target),
                Word::U16WrapperMut(U16Wrapper(&mut expected_values.0, &mut expected_values.1)),
                Word::U16Mut(&mut stack_pointer),
                Some(&mut flags), // condition is same as flags
            ),
            branch_args,
        );

        assert_eq!(target, answer);

        assert_eq!(stack_pointer, 0);
        assert_eq!(result_cycles, cycles[1]);
    }
    #[test]
    fn test_call() {
        let source = 30000;
        let mut target = convert_u16_to_two_u8s(source);
        let mut pc = 0x6996;
        let address = 0xFFFF;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: call,
        };
        let branch_args = BranchArgs {
            cycles: vec![16,4],
            condition: None,
        };

        let mut stack_pointer = 10;

        instruction.exec(
            Operands::Call(
                Word::U16WrapperMut(U16Wrapper(&mut target.0, &mut target.1)),
                Word::U16Mut(&mut pc),
                Word::U16Mut(&mut stack_pointer),
                Word::U16(address),
                None,
            ),
            branch_args,
        );

        assert_eq!(pc, address);
        assert_eq!(target, (0x96, 0x69));
        assert_eq!(stack_pointer, 8);
    }
    #[test]
    fn test_call_condition_true() {
        let source = 30000;
        let mut target = convert_u16_to_two_u8s(source);
        let mut pc = 0x6996;
        let address = 0xFFFF;
        let mut flags = 0b1100_0000;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: call,
        };
        let cycles = vec![16,4];
        let branch_args = BranchArgs {
            cycles: cycles.clone(),
            condition: Some(flags),
        };

        let mut stack_pointer = 10;

        let result_cycles = instruction.exec(
            Operands::Call(
                Word::U16WrapperMut(U16Wrapper(&mut target.0, &mut target.1)),
                Word::U16Mut(&mut pc),
                Word::U16Mut(&mut stack_pointer),
                Word::U16(address),
                Some(&mut flags),
            ),
            branch_args,
        );

        assert_eq!(pc, address);
        assert_eq!(target, (0x96, 0x69));
        assert_eq!(stack_pointer, 8);
        assert_eq!(result_cycles, cycles[0]);
    }
    #[test]
    fn test_call_condition_false() {
        let source = 30000;
        let mut target = convert_u16_to_two_u8s(source);
        let start_pc = 0x6996;
        let mut pc = start_pc;
        let address = 0xFFFF;
        let mut flags = 0b1100_0000;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: call,
        };
        let cycles = vec![16,4];
        let branch_args = BranchArgs {
            cycles: cycles.clone(),
            condition: Some(0b1111_0000),
        };

        let mut stack_pointer = 10;

        let result_cycles = instruction.exec(
            Operands::Call(
                Word::U16WrapperMut(U16Wrapper(&mut target.0, &mut target.1)),
                Word::U16Mut(&mut pc),
                Word::U16Mut(&mut stack_pointer),
                Word::U16(address),
                Some(&mut flags),
            ),
            branch_args,
        );

        assert_eq!(pc, start_pc);
        assert_eq!(target, convert_u16_to_two_u8s(source));
        assert_eq!(stack_pointer, 10);
        assert_eq!(result_cycles, cycles[1]);
    }
}
