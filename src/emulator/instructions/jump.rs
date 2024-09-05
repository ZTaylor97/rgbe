use num_traits::{
    ops::overflowing::{OverflowingAdd, OverflowingSub},
    WrappingAdd, WrappingSub,
};

use super::utils::{check_condition, Args, BranchArgs, InstructionError, Operands, Ret, Word};
use crate::emulator::{
    cpu::cpu_registers::CPURegisters,
    memory::{Memory, U16Wrapper},
};

pub fn jp(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Two(target, source, flags) = operands {
        match (target, source) {
            (Word::U16Mut(target), Word::U16(source)) => {
                if let Some(condition) = branch_args.condition {
                    if check_condition(*flags.unwrap(), condition) {
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
                    if check_condition(*flags.unwrap(), condition) {
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


pub fn get_jump_operands<'a>(
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
        // JP instructions
        0xC2 => {
            // jp NZ
            let address = if let Some(Ret::U16(address)) = value {
                address
            } else {
                return Err(InstructionError::InvalidLiteral(value.unwrap()));
            };
            Ok((
                Operands::Two(dest, Word::U16(address), Some(&mut registers.f)),
                Some(0b1100_0000),
            ))
        }
        0xD2 => {
            //jp NC
            let address = if let Some(Ret::U16(address)) = value {
                address
            } else {
                return Err(InstructionError::InvalidLiteral(value.unwrap()));
            };
            Ok((
                Operands::Two(dest, Word::U16(address), Some(&mut registers.f)),
                Some(0b0101_0000),
            ))
        }
        0xC3 => {
            // JP
            let address = if let Some(Ret::U16(address)) = value {
                address
            } else {
                return Err(InstructionError::InvalidLiteral(value.unwrap()));
            };
            Ok((Operands::Two(dest, Word::U16(address), None), None))
        }
        0xCA => {
            // jp Z
            let address = if let Some(Ret::U16(address)) = value {
                address
            } else {
                return Err(InstructionError::InvalidLiteral(value.unwrap()));
            };
            Ok((
                Operands::Two(dest, Word::U16(address), Some(&mut registers.f)),
                Some(0b1000_0000),
            ))
        }
        0xDA => {
            // jp C
            let address = if let Some(Ret::U16(address)) = value {
                address
            } else {
                return Err(InstructionError::InvalidLiteral(value.unwrap()));
            };
            Ok((
                Operands::Two(dest, Word::U16(address), Some(&mut registers.f)),
                Some(0b0001_0000),
            ))
        }
        0xE9 => {
            // jp C
            Ok((Operands::Two(dest, Word::U16(hl), None), None))
        }
        // JR instructions
        0x20 => {
            // jr NZ
            let address = if let Some(Ret::U8(address)) = value {
                address
            } else {
                return Err(InstructionError::InvalidLiteral(value.unwrap()));
            };
            Ok((
                Operands::Two(dest, Word::U8(address), Some(&mut registers.f)),
                Some(0b1100_0000),
            ))
        }
        0x30 => {
            //jr NC
            let address = if let Some(Ret::U8(address)) = value {
                address
            } else {
                return Err(InstructionError::InvalidLiteral(value.unwrap()));
            };
            Ok((
                Operands::Two(dest, Word::U8(address), Some(&mut registers.f)),
                Some(0b0101_0000),
            ))
        }
        0x18 => {
            // jr
            let address = if let Some(Ret::U8(address)) = value {
                address
            } else {
                return Err(InstructionError::InvalidLiteral(value.unwrap()));
            };
            Ok((Operands::Two(dest, Word::U8(address), Some(&mut registers.f)), None))
        }
        0x28 => {
            // jr Z
            let address = if let Some(Ret::U8(address)) = value {
                address
            } else {
                return Err(InstructionError::InvalidLiteral(value.unwrap()));
            };
            Ok((
                Operands::Two(dest, Word::U8(address), Some(&mut registers.f)),
                Some(0b1000_0000),
            ))
        }
        0x38 => {
            // jr C
            let address = if let Some(Ret::U8(address)) = value {
                address
            } else {
                return Err(InstructionError::InvalidLiteral(value.unwrap()));
            };
            Ok((
                Operands::Two(dest, Word::U8(address), Some(&mut registers.f)),
                Some(0b0001_0000),
            ))
        }

        _ => return Err(InstructionError::UnimplementedError(opcode)),
    }
}

#[cfg(test)]
mod jump_instruction_tests {
    use crate::emulator::{
        cpu::cpu_registers::convert_u16_to_two_u8s, instructions::*, memory::U16Wrapper,
    };
    use utils::Word;

    #[test]
    fn test_jp_a16() {
        let instruction = Instruction {
            data: InstructionData::default(),
            func: jp,
        };

        let branch_args = BranchArgs {
            cycles: vec![16],
            condition: None,
        };

        let mut flags = 0;
        let target_instruction = 0xAAAA;
        let mut fake_pc: u16 = 0x0000;

        instruction.exec(
            Operands::Two(
                Word::U16Mut(&mut fake_pc),
                Word::U16(target_instruction),
                Some(&mut flags),
            ),
            branch_args,
        );

        assert_eq!(fake_pc, target_instruction);
    }
    #[test]
    fn test_jp_nz_a16_true() {
        let instruction = Instruction {
            data: InstructionData::default(),
            func: jp,
        };

        let nz_val = 0b1100_0000;

        let branch_args = BranchArgs {
            cycles: vec![16, 12],
            condition: Some(nz_val),
        };

        let mut flags = nz_val;
        let target_instruction = 0xAAAA;
        let mut fake_pc: u16 = 0x0000;

        let cycles = instruction.exec(
            Operands::Two(
                Word::U16Mut(&mut fake_pc),
                Word::U16(target_instruction),
                Some(&mut flags),
            ),
            branch_args,
        );

        assert_eq!(fake_pc, target_instruction);
        assert_eq!(cycles, 16);
    }
    #[test]
    fn test_jp_nz_a16_false() {
        let instruction = Instruction {
            data: InstructionData::default(),
            func: jp,
        };

        let nz_val = 0b1100_0000;

        let branch_args = BranchArgs {
            cycles: vec![16, 12],
            condition: Some(nz_val),
        };

        let mut flags = 0;
        let target_instruction = 0xAAAA;
        let mut fake_pc: u16 = 0x0000;

        let cycles = instruction.exec(
            Operands::Two(
                Word::U16Mut(&mut fake_pc),
                Word::U16(target_instruction),
                Some(&mut flags),
            ),
            branch_args,
        );

        assert_eq!(fake_pc, 0x0000);
        assert_eq!(cycles, 12);
    }
    #[test]
    fn test_jr_e8() {
        let instruction = Instruction {
            data: InstructionData::default(),
            func: jr,
        };

        let branch_args = BranchArgs {
            cycles: vec![16, 12],
            condition: None,
        };

        let pc_value = 121;

        let mut flags = 0;
        let target_instruction: i8 = -120;
        let mut fake_pc: u16 = pc_value;

        let cycles = instruction.exec(
            Operands::Two(
                Word::U16Mut(&mut fake_pc),
                Word::U8(target_instruction as u8),
                Some(&mut flags),
            ),
            branch_args,
        );

        assert_eq!(
            fake_pc,
            u16::wrapping_add_signed(pc_value, target_instruction as i16)
        );
        assert_eq!(cycles, 16);
    }
}
