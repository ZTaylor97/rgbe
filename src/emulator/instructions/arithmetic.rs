use crate::emulator::{
    cpu::cpu_registers::CPURegisters,
    memory::{Memory, U16Wrapper},
};

use super::utils::{Args, BranchArgs, InstructionError, Operands, Ret, Word};

pub fn add(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Two(target, source, flags) = operands {
        match (target, source) {
            (Word::U8Mut(target), Word::U8(source)) => {
                *target = add_with_flags(target.clone(), source, flags.unwrap());
                Ok(branch_args.cycles[0])
            }
            (Word::U16WrapperMut(target), Word::U16(source)) => {
                let mut val: u16 = target.into_u16();
                val = add_u16_with_flags(val, source, flags.unwrap());
                target.from_u16(val);
                Ok(branch_args.cycles[0])
            }
            (Word::U16Mut(target), Word::U16(source)) => {
                *target = add_u16_with_flags(target.clone(), source, flags.unwrap());
                Ok(branch_args.cycles[0])
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

pub fn adc(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Two(target, source, flags) = operands {
        match (target, source) {
            (Word::U8Mut(target), Word::U8(source)) => {
                let mut flags: &mut u8 = flags.unwrap();

                let source = if ((*flags & 0b00010000) >> 4) == 1 {
                    source + 1
                } else {
                    source
                };

                *target = add_with_flags(target.clone(), source, flags);

                Ok(branch_args.cycles[0])
            }
            (word1, word2) => {
                return Err(InstructionError::IncorrectOperandsError(format!(
                    "Incorrect words {:?} , {:?} passed to function adc",
                    word1, word2
                )));
            }
        }
    } else {
        return Err(InstructionError::InvalidOperandsError(operands));
    }
}
pub fn sub(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Two(target, source, flags) = operands {
        match (target, source) {
            (Word::U8Mut(target), Word::U8(source)) => {
                *target = sub_with_flags(target.clone(), source, flags.unwrap());
                Ok(branch_args.cycles[0])
            }
            (word1, word2) => {
                return Err(InstructionError::IncorrectOperandsError(format!(
                    "Incorrect words {:?} , {:?} passed to add function sub",
                    word1, word2
                )));
            }
        }
    } else {
        return Err(InstructionError::InvalidOperandsError(operands));
    }
}

pub fn sbc(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Two(target, source, flags) = operands {
        match (target, source) {
            (Word::U8Mut(target), Word::U8(source)) => {
                let mut flags: &mut u8 = flags.unwrap();

                let source = if ((*flags & 0b00010000) >> 4) == 1 {
                    source + 1
                } else {
                    source
                };

                *target = sub_with_flags(target.clone(), source, flags);
                Ok(branch_args.cycles[0])
            }
            (word1, word2) => {
                return Err(InstructionError::IncorrectOperandsError(format!(
                    "Incorrect words {:?} , {:?} passed to add function sbc",
                    word1, word2
                )));
            }
        }
    } else {
        return Err(InstructionError::InvalidOperandsError(operands));
    }
}
pub fn xor(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Two(target, source, flags) = operands {
        match (target, source) {
            (Word::U8Mut(target), Word::U8(source)) => {
                *target ^= source;

                let zero: u8 = (*target == 0) as u8;

                *flags.unwrap() = zero << 7;
                Ok(branch_args.cycles[0])
            }
            (word1, word2) => {
                return Err(InstructionError::IncorrectOperandsError(format!(
                    "Incorrect words {:?} , {:?} passed to add function xor",
                    word1, word2
                )));
            }
        }
    } else {
        return Err(InstructionError::InvalidOperandsError(operands));
    }
}

pub fn and(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Two(target, source, flags) = operands {
        match (target, source) {
            (Word::U8Mut(target), Word::U8(source)) => {
                *target &= source;

                let zero: u8 = (*target == 0) as u8;

                *flags.unwrap() = zero << 7 | 1 << 5; // half carry always set
                Ok(branch_args.cycles[0])
            }
            (word1, word2) => {
                return Err(InstructionError::IncorrectOperandsError(format!(
                    "Incorrect words {:?} , {:?} passed to add function and",
                    word1, word2
                )));
            }
        }
    } else {
        return Err(InstructionError::InvalidOperandsError(operands));
    }
}
pub fn or(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Two(target, source, flags) = operands {
        match (target, source) {
            (Word::U8Mut(target), Word::U8(source)) => {
                *target |= source;

                let zero: u8 = (*target == 0) as u8;

                *flags.unwrap() = zero << 7; // half carry always set
                Ok(branch_args.cycles[0])
            }
            (word1, word2) => {
                return Err(InstructionError::IncorrectOperandsError(format!(
                    "Incorrect words {:?} , {:?} passed to add function or",
                    word1, word2
                )));
            }
        }
    } else {
        return Err(InstructionError::InvalidOperandsError(operands));
    }
}

pub fn cp(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Two(target, source, flags) = operands {
        match (target, source) {
            (Word::U8Mut(target), Word::U8(source)) => {
                sub_with_flags(target.clone(), source, flags.unwrap());
                Ok(branch_args.cycles[0])
            }
            (word1, word2) => {
                return Err(InstructionError::IncorrectOperandsError(format!(
                    "Incorrect words {:?} , {:?} passed to add function cp",
                    word1, word2
                )));
            }
        }
    } else {
        return Err(InstructionError::InvalidOperandsError(operands));
    }
}
fn sub_with_flags(target: u8, source: u8, flags: &mut u8) -> u8 {
    let result: (u8, bool) = target.overflowing_sub(source);
    let carry = result.1 as u8;

    let half_carry = if (source & 0x0f) > (target & 0x0f) {
        1
    } else {
        0
    };

    let zero = if result.0 == 0 { 1 } else { 0 };

    let negative = 1;
    *flags = (zero << 7) | (negative << 6) | (half_carry << 5) | (carry << 4);

    result.0
}
fn add_with_flags(target: u8, source: u8, flags: &mut u8) -> u8 {
    let result: (u8, bool) = target.overflowing_add(source);
    let carry = result.1 as u8;

    let zero = if result.0 == 0 { 1 } else { 0 };

    let half_carry = if ((result.0 & 0x0f) + (source & 0x0f)) > 0x0f {
        1
    } else {
        0
    };

    let negative = 0;
    *flags = (zero << 7) | (negative << 6) | (half_carry << 5) | (carry << 4);

    result.0
}
fn add_u16_with_flags(target: u16, source: u16, flags: &mut u8) -> u16 {
    let result: (u16, bool) = target.overflowing_add(source);
    let carry = result.1 as u8;

    let zero = if result.0 == 0 { 1 } else { 0 };

    let half_carry = if ((result.0 & 0x0f) + (source & 0x0f)) > 0x0f {
        1
    } else {
        0
    };

    let negative = 0;
    *flags = (zero << 7) | (negative << 6) | (half_carry << 5) | (carry << 4);

    result.0
}

pub fn get_arithmetic_operands<'a>(
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

    // When in that nice block of load instructions
    let ops = if let 0x80..=0xBF = opcode {
        let source = match lo {
            0x0 | 0x8 => Word::U8(registers.b),
            0x1 | 0x9 => Word::U8(registers.c),
            0x2 | 0xA => Word::U8(registers.d),
            0x3 | 0xB => Word::U8(registers.e),
            0x4 | 0xC => Word::U8(registers.h),
            0x5 | 0xD => Word::U8(registers.l),
            0x6 | 0xE => Word::U8(mem_hl_val),
            0x7 | 0xF => Word::U8(reg_copy.a),
            _ => return Err(InstructionError::UnimplementedError(opcode)),
        };

        Operands::Two(
            Word::U8Mut(&mut registers.a),
            source,
            Some(&mut registers.f),
        )
    } else {
        match lo {
            0x6 | 0xE => {
                let value: u8 = if let Some(Ret::U8(x)) = value {
                    x
                } else {
                    return Err(InstructionError::InvalidLiteral(value.unwrap()));
                };

                Operands::Two(
                    Word::U8Mut(&mut registers.a),
                    Word::U8(value),
                    Some(&mut registers.f),
                )
            }
            0x9 => {
                let reg_copy = registers.clone();
                let dest = Word::U16WrapperMut(U16Wrapper(&mut registers.h, &mut registers.l));

                match hi {
                    0x0 => {
                        Operands::Two(dest, Word::U16(reg_copy.get_bc()), Some(&mut registers.f))
                    }
                    0x1 => {
                        Operands::Two(dest, Word::U16(reg_copy.get_de()), Some(&mut registers.f))
                    }
                    0x2 => {
                        Operands::Two(dest, Word::U16(reg_copy.get_hl()), Some(&mut registers.f))
                    }
                    0x3 => Operands::Two(dest, Word::U16(reg_copy.sp), Some(&mut registers.f)),
                    _ => return Err(InstructionError::UnimplementedError(opcode)),
                }
            }
            _ => return Err(InstructionError::UnimplementedError(opcode)),
        }
    };

    Ok((ops, None))
}

#[cfg(test)]
mod arithmetic_instruction_tests {
    use crate::emulator::instructions::*;
    use arithmetic::{adc, add, sbc, sub, xor};
    use utils::Word;

    #[test]
    fn test_add_r8_r8() {
        let source = 10;
        let mut target = 5;
        let desired_result = source + target;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: add,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };

        let mut flags = 0;

        instruction.exec(
            Operands::Two(Word::U8Mut(&mut target), Word::U8(source), Some(&mut flags)),
            branch_args,
        );

        assert_eq!(target, desired_result);
    }
    #[test]
    fn test_add_r8_r8_zero_flag() {
        let source = 0;
        let mut target = 0;
        let desired_result = source + target;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: add,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };
        let mut flags = 0;

        instruction.exec(
            Operands::Two(Word::U8Mut(&mut target), Word::U8(source), Some(&mut flags)),
            branch_args,
        );

        assert_eq!(flags, 0b10000000);
        assert_eq!(target, desired_result);
    }
    #[test]
    fn test_add_r8_r8_carry_flag() {
        let source: u8 = 200;
        let mut target: u8 = 200;
        let desired_result = source.wrapping_add(target);

        let instruction = Instruction {
            data: InstructionData::default(),
            func: add,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };

        let mut flags = 0;

        instruction.exec(
            Operands::Two(Word::U8Mut(&mut target), Word::U8(source), Some(&mut flags)),
            branch_args,
        );
        assert_eq!(flags, 0b00010000);
        assert_eq!(target, desired_result);
    }
    #[test]
    fn test_adc_r8_r8() {
        let source = 10;
        let mut target = 5;
        let desired_result = source + target + 1;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: adc,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };

        let mut flags = 0b00010000;

        instruction.exec(
            Operands::Two(Word::U8Mut(&mut target), Word::U8(source), Some(&mut flags)),
            branch_args,
        );

        assert_eq!(target, desired_result);
    }
    #[test]
    fn test_adc_r8_r8_zero_flag() {
        let source = 0;
        let mut target = 0;
        let desired_result = source + target + 1;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: adc,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };
        let mut flags = 0b0001_0000;

        instruction.exec(
            Operands::Two(Word::U8Mut(&mut target), Word::U8(source), Some(&mut flags)),
            branch_args,
        );

        assert_eq!(flags, 0b00000000); // zero flags should NOT be set because of carry
        assert_eq!(target, desired_result);
    }
    #[test]
    fn test_adc_r8_r8_carry_flag() {
        let source: u8 = 200;
        let mut target: u8 = 200;
        let desired_result = source.wrapping_add(target + 1);

        let instruction = Instruction {
            data: InstructionData::default(),
            func: adc,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };

        let mut flags = 0b00010000;

        instruction.exec(
            Operands::Two(Word::U8Mut(&mut target), Word::U8(source), Some(&mut flags)),
            branch_args,
        );
        assert_eq!(flags, 0b00010000);
        assert_eq!(target, desired_result);
    }

    #[test]
    fn test_sub_r8_r8() {
        let source = 5;
        let mut target = 10;
        let desired_result = target - source;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: sub,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };

        let mut flags = 0b00000000;

        instruction.exec(
            Operands::Two(Word::U8Mut(&mut target), Word::U8(source), Some(&mut flags)),
            branch_args,
        );

        assert_eq!(target, desired_result);
        assert_eq!(flags, 0b01000000);
    }
    #[test]
    fn test_sub_r8_r8_zero_flag() {
        let source = 10;
        let mut target = 10;
        let desired_result = source - target;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: sub,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };
        let mut flags = 0;

        instruction.exec(
            Operands::Two(Word::U8Mut(&mut target), Word::U8(source), Some(&mut flags)),
            branch_args,
        );

        assert_eq!(flags, 0b11000000);
        assert_eq!(target, desired_result);
    }
    #[test]
    fn test_sub_r8_r8_carry_flags() {
        let source: u8 = 0b1100_1000; //200
        let mut target: u8 = 0b0110_0100; // 100
        let desired_result = target.wrapping_sub(source);

        let instruction = Instruction {
            data: InstructionData::default(),
            func: sub,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };

        let mut flags = 0;

        instruction.exec(
            Operands::Two(Word::U8Mut(&mut target), Word::U8(source), Some(&mut flags)),
            branch_args,
        );
        assert_eq!(flags, 0b0111_0000);
        assert_eq!(target, desired_result);
    }

    #[test]
    fn test_sbc_r8_r8() {
        let source = 5;
        let mut target = 10;
        let desired_result = target - (source + 1);

        let instruction = Instruction {
            data: InstructionData::default(),
            func: sbc,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };

        let mut flags = 0b00010000;

        instruction.exec(
            Operands::Two(Word::U8Mut(&mut target), Word::U8(source), Some(&mut flags)),
            branch_args,
        );

        assert_eq!(flags, 0b0100_0000);
        assert_eq!(target, desired_result);
    }
    #[test]
    fn test_sbc_r8_r8_zero_flag() {
        let source: u8 = 0;
        let mut target: u8 = 1;
        let desired_result: u8 = target - (source + 1);

        let instruction = Instruction {
            data: InstructionData::default(),
            func: sbc,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };
        let mut flags = 0b00010000;

        instruction.exec(
            Operands::Two(Word::U8Mut(&mut target), Word::U8(source), Some(&mut flags)),
            branch_args,
        );

        assert_eq!(flags, 0b1100_0000);
        assert_eq!(target, desired_result);
    }

    #[test]
    fn test_sbc_r8_r8_carry_flag() {
        let source: u8 = 200;
        let mut target: u8 = 100;
        let desired_result = target.wrapping_sub(source + 1);

        let instruction = Instruction {
            data: InstructionData::default(),
            func: sbc,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };

        let mut flags = 0b00010000;

        instruction.exec(
            Operands::Two(Word::U8Mut(&mut target), Word::U8(source), Some(&mut flags)),
            branch_args,
        );
        assert_eq!(flags, 0b0111_0000); // both carry and half carry should be set
        assert_eq!(target, desired_result);
    }

    #[test]
    fn test_xor() {
        let source: u8 = 0b1111_0000;
        let mut target: u8 = 0b0000_1111;
        let desired_result = 0b1111_1111;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: xor,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };

        let mut flags = 0b00010000;

        instruction.exec(
            Operands::Two(Word::U8Mut(&mut target), Word::U8(source), Some(&mut flags)),
            branch_args,
        );
        assert_eq!(target, desired_result);
    }
}
