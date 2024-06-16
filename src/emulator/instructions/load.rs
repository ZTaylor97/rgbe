use num_traits::WrappingAdd;

use crate::emulator::{
    cpu::cpu_registers::CPURegisters,
    memory::{Memory, U16Wrapper},
};

use super::utils::{Args, BranchArgs, InstructionError, Operands, Ret, Word};

pub fn ld(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    if let Operands::Two(target, source, _) = operands {
        match (target, source) {
            (Word::U8Mut(target), Word::U8(source)) => {
                *target = source;
            }
            (Word::U16WrapperMut(target), Word::U16(source)) => target.from_u16(source),
            (Word::U16Mut(target), Word::U16(source)) => *target = source,
            (word1, word2) => {
                return Err(InstructionError::IncorrectOperandsError(format!(
                    "Incorrect words {:?} , {:?} passed to add function ld",
                    word1, word2
                )));
            }
        }
    } else {
        return Err(InstructionError::InvalidOperandsError(operands));
    }
    Ok(branch_args.cycles[0])
}

pub fn get_ld_operands<'a>(
    registers: &'a mut CPURegisters,
    mem: &'a mut Memory,
    opcode: u8,
    value: Option<Ret>,
) -> Result<Args<'a>, InstructionError<'a>> {
    let hi = (opcode & 0xF0) >> 4;
    let lo = opcode & 0x0F;

    let reg_copy = registers.clone();
    // When in that nice block of load instructions
    if let 0x40..=0x7F = opcode {
        let hl = registers.get_hl();
        let mem_hl_val = mem.read_u8(hl).clone();

        let dest = match hi {
            0x4 => {
                if lo <= 0x7 {
                    Word::U8Mut(&mut registers.b)
                } else {
                    Word::U8Mut(&mut registers.c)
                }
            }
            0x5 => {
                if lo <= 0x7 {
                    Word::U8Mut(&mut registers.d)
                } else {
                    Word::U8Mut(&mut registers.e)
                }
            }
            0x6 => {
                if lo <= 0x7 {
                    Word::U8Mut(&mut registers.h)
                } else {
                    Word::U8Mut(&mut registers.l)
                }
            }
            0x7 => {
                if lo <= 0x7 {
                    Word::U8Mut(mem.read_u8_mut(hl))
                } else {
                    Word::U8Mut(&mut registers.a)
                }
            }
            _ => return Err(InstructionError::UnimplementedError(opcode)),
        };

        let ops = match lo {
            0x0 => Operands::Two(dest, Word::U8(reg_copy.b), None),
            0x1 => Operands::Two(dest, Word::U8(reg_copy.c), None),
            0x2 => Operands::Two(dest, Word::U8(reg_copy.d), None),
            0x3 => Operands::Two(dest, Word::U8(reg_copy.e), None),
            0x4 => Operands::Two(dest, Word::U8(reg_copy.h), None),
            0x5 => Operands::Two(dest, Word::U8(reg_copy.l), None),
            0x6 => Operands::Two(dest, Word::U8(mem_hl_val), None),
            0x7 => Operands::Two(dest, Word::U8(reg_copy.a), None),
            0x8 => Operands::Two(dest, Word::U8(reg_copy.b), None),
            0x9 => Operands::Two(dest, Word::U8(reg_copy.c), None),
            0xA => Operands::Two(dest, Word::U8(reg_copy.d), None),
            0xB => Operands::Two(dest, Word::U8(reg_copy.e), None),
            0xC => Operands::Two(dest, Word::U8(reg_copy.h), None),
            0xD => Operands::Two(dest, Word::U8(reg_copy.l), None),
            0xE => Operands::Two(dest, Word::U8(mem_hl_val), None),
            0xF => Operands::Two(dest, Word::U8(reg_copy.a), None),
            _ => return Err(InstructionError::UnimplementedError(opcode)),
        };
        Ok((ops, None))
    } else {
        let ops = match lo {
            0x0 => {
                let value = if let Some(Ret::U8(x)) = value {
                    x
                } else {
                    return Err(InstructionError::InvalidLiteral(value.unwrap()));
                };

                match hi {
                    0xE => Operands::Two(
                        Word::U8Mut(mem.read_u8_mut(value as u16)),
                        Word::U8(registers.a),
                        None,
                    ),
                    0xF => Operands::Two(
                        Word::U8Mut(&mut registers.a),
                        Word::U8(mem.read_u8(value as u16)),
                        None,
                    ),
                    _ => return Err(InstructionError::UnimplementedError(opcode)),
                }
            }
            0x1 => {
                let value = if let Some(Ret::U16(x)) = value {
                    x
                } else {
                    return Err(InstructionError::InvalidLiteral(value.unwrap()));
                };

                match hi {
                    0x0 => Operands::Two(
                        Word::U16WrapperMut(U16Wrapper(&mut registers.b, &mut registers.c)),
                        Word::U16(value),
                        None,
                    ),
                    0x1 => Operands::Two(
                        Word::U16WrapperMut(U16Wrapper(&mut registers.d, &mut registers.e)),
                        Word::U16(value),
                        None,
                    ),
                    0x2 => Operands::Two(
                        Word::U16WrapperMut(U16Wrapper(&mut registers.h, &mut registers.l)),
                        Word::U16(value),
                        None,
                    ),
                    0x3 => Operands::Two(Word::U16Mut(&mut registers.sp), Word::U16(value), None),
                    _ => return Err(InstructionError::UnimplementedError(opcode)),
                }
            }
            0x2 => {
                let source = registers.a;

                match hi {
                    0x0 => Operands::Two(
                        Word::U8Mut(mem.read_u8_mut(registers.get_bc())),
                        Word::U8(source),
                        None,
                    ),
                    0x1 => Operands::Two(
                        Word::U8Mut(mem.read_u8_mut(registers.get_de())),
                        Word::U8(source),
                        None,
                    ),
                    0x2 => {
                        let hl = registers.get_hl();
                        let ops =
                            Operands::Two(Word::U8Mut(mem.read_u8_mut(hl)), Word::U8(source), None);
                        registers.set_hl(hl + 1);
                        ops
                    }
                    0x3 => {
                        let hl = registers.get_hl();
                        let ops =
                            Operands::Two(Word::U8Mut(mem.read_u8_mut(hl)), Word::U8(source), None);
                        registers.set_hl(hl - 1);
                        ops
                    }
                    0xE => Operands::Two(
                        Word::U8Mut(mem.read_u8_mut(registers.c as u16)),
                        Word::U8(source),
                        None,
                    ),
                    0xF => Operands::Two(
                        Word::U8Mut(mem.read_u8_mut(source as u16)),
                        Word::U8(registers.c),
                        None,
                    ),
                    _ => return Err(InstructionError::UnimplementedError(opcode)),
                }
            }
            0x8 => {
                let mut u16_value: u16 = 0;
                let mut u8_value: u8 = 0;

                match value {
                    Some(Ret::U16(x)) => {
                        u16_value = x;
                    }
                    Some(Ret::U8(x)) => u8_value = x,
                    _ => return Err(InstructionError::InvalidLiteral(value.unwrap())),
                }

                match hi {
                    0x0 => Operands::Two(
                        Word::U16WrapperMut(U16Wrapper(&mut registers.b, &mut registers.c)),
                        Word::U16(u16_value),
                        None,
                    ),
                    0xF => Operands::Two(
                        Word::U16WrapperMut(U16Wrapper(&mut registers.h, &mut registers.l)),
                        Word::U16(registers.sp.wrapping_add(u8_value as u16)),
                        None,
                    ),
                    _ => return Err(InstructionError::UnimplementedError(opcode)),
                }
            }
            0x9 => match hi {
                0xF => {
                    let hl = reg_copy.get_hl();
                    Operands::Two(Word::U16Mut(&mut registers.sp), Word::U16(hl), None)
                }
                _ => return Err(InstructionError::UnimplementedError(opcode)),
            },
            0xA => match hi {
                0x0 => Operands::Two(
                    Word::U8Mut(&mut registers.a),
                    Word::U8(mem.read_u8(reg_copy.get_bc())),
                    None,
                ),
                0x1 => Operands::Two(
                    Word::U8Mut(&mut registers.a),
                    Word::U8(mem.read_u8(reg_copy.get_de())),
                    None,
                ),
                0x2 => {
                    let hl = reg_copy.get_hl();
                    registers.set_hl(hl + 1);
                    let ops = Operands::Two(
                        Word::U8Mut(&mut registers.a),
                        Word::U8(mem.read_u8(hl)),
                        None,
                    );
                    ops
                }
                0x3 => {
                    let hl = registers.get_hl();
                    registers.set_hl(hl + 1);
                    let ops = Operands::Two(
                        Word::U8Mut(&mut registers.a),
                        Word::U8(mem.read_u8(hl)),
                        None,
                    );
                    ops
                }
                0xE => {
                    let value: u16 = if let Some(Ret::U16(x)) = value {
                        x
                    } else {
                        return Err(InstructionError::InvalidLiteral(value.unwrap()));
                    };

                    Operands::Two(
                        Word::U8Mut(mem.read_u8_mut(value)),
                        Word::U8(registers.a),
                        None,
                    )
                }
                0xF => {
                    let value: u16 = if let Some(Ret::U16(x)) = value {
                        x
                    } else {
                        return Err(InstructionError::InvalidLiteral(value.unwrap()));
                    };

                    Operands::Two(
                        Word::U8Mut(&mut registers.a),
                        Word::U8(mem.read_u8(value)),
                        None,
                    )
                }
                _ => return Err(InstructionError::UnimplementedError(opcode)),
            },

            0xE => match hi {
                0x0 => Operands::Two(
                    Word::U8Mut(&mut registers.c),
                    Word::U8(mem.read_u8(reg_copy.get_bc())),
                    None,
                ),
                0x1 => Operands::Two(
                    Word::U8Mut(&mut registers.e),
                    Word::U8(mem.read_u8(reg_copy.get_de())),
                    None,
                ),
                0x2 => {
                    let hl = reg_copy.get_hl();
                    registers.set_hl(hl + 1);
                    let ops = Operands::Two(
                        Word::U8Mut(&mut registers.l),
                        Word::U8(mem.read_u8(hl)),
                        None,
                    );
                    ops
                }
                0x3 => {
                    let hl = registers.get_hl();
                    registers.set_hl(hl + 1);
                    let ops = Operands::Two(
                        Word::U8Mut(&mut registers.a),
                        Word::U8(mem.read_u8(hl)),
                        None,
                    );
                    ops
                }
                _ => return Err(InstructionError::UnimplementedError(opcode)),
            },
            _ => return Err(InstructionError::UnimplementedError(opcode)),
        };
        Ok((ops, None))
    }
}

#[cfg(test)]
mod load_instruction_tests {
    use crate::emulator::{
        cpu::cpu_registers::convert_u16_to_two_u8s, instructions::*, memory::U16Wrapper,
    };
    use utils::Word;

    #[test]
    fn test_ld_r8_r8() {
        let source = 10;
        let mut target = 0;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: ld,
        };
        let branch_args = BranchArgs {
            cycles: vec![4],
            condition: None,
        };

        instruction.exec(
            Operands::Two(Word::U8Mut(&mut target), Word::U8(source), None),
            branch_args,
        );

        assert_eq!(target, source)
    }

    #[test]
    fn test_ld_r16_r16() {
        let source = 30000;
        let mut target = (100, 200);

        let instruction = Instruction {
            data: InstructionData::default(),
            func: ld,
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
