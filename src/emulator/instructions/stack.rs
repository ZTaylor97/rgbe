use num_traits::{
    ops::overflowing::{OverflowingAdd, OverflowingSub},
    WrappingAdd, WrappingSub,
};

use super::utils::{Args, BranchArgs, InstructionError, Operands, Ret, Word};
use crate::emulator::{
    cpu::cpu_registers::CPURegisters,
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

pub fn get_stack_operands<'a>(
    registers: &'a mut CPURegisters,
    mem: &'a mut Memory,
    opcode: u8,
    value: Option<Ret>,
) -> Result<Args<'a>, InstructionError<'a>> {
    let hi = (opcode & 0xF0) >> 4;
    let lo = opcode & 0x0F;

    let source = match lo {
        0xC => U16Wrapper(&mut registers.b, &mut registers.c),
        0xD => U16Wrapper(&mut registers.d, &mut registers.e),
        0xE => U16Wrapper(&mut registers.h, &mut registers.l),
        0xF => U16Wrapper(&mut registers.a, &mut registers.f),
        _ => return Err(InstructionError::UnimplementedError(opcode)),
    };

    let ops = match hi {
        0x5 => {
            registers.sp -= 2;
            Operands::Two(
                Word::U16WrapperMut(mem.read_u16wrapper(registers.sp)),
                Word::U16WrapperMut(source),
                None,
            )
        }
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

#[cfg(test)]
mod stack_instruction_tests {
    use crate::emulator::{
        cpu::cpu_registers::convert_u16_to_two_u8s, instructions::*, memory::U16Wrapper,
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

        assert_eq!( target, (expected_values.1,expected_values.0))
    }
}
