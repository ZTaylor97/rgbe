use num_traits::{
    ops::overflowing::{OverflowingAdd, OverflowingSub},
    WrappingAdd, WrappingSub,
};

use super::utils::{Args, BranchArgs, InstructionError, Operands, Ret, Word};
use crate::emulator::{
    cpu::cpu_registers::CPURegisters,
    memory::{Memory, U16Wrapper},
};

pub fn push(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Two(target, source, flags) = operands {
        match (target, source) {
            (Word::U16WrapperMut(target), Word::U16(source)) => target.from_u16(source),
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

    let reg_copy = registers.clone();

    let hl = registers.get_hl();
    let mem_hl_val = mem.read_u8(hl).clone();

    let dest = Word::U16Mut(&mut registers.pc);

    match opcode {
        _ => return Err(InstructionError::UnimplementedError(opcode)),
    }
}

#[cfg(test)]
mod stack_instruction_tests {
    use crate::emulator::{
        cpu::cpu_registers::convert_u16_to_two_u8s, instructions::*, memory::U16Wrapper,
    };
    use utils::Word;

    #[test]
    fn test_push_af() {
        let source = 30000;
        let mut target = (100, 200);

        let instruction = Instruction {
            data: InstructionData::default(),
            func: push,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };

        instruction.exec(
            Operands::Two(
                Word::U16WrapperMut(U16Wrapper(&mut target.0, &mut target.1)),
                Word::U16(source),
                None,
            ),
            branch_args,
        );

        let expected_values = convert_u16_to_two_u8s(source);
        assert_eq!(target, expected_values)

    }
}
