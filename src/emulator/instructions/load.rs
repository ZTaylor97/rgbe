use crate::emulator::{cpu::cpu_registers::CPURegisters, memory::Memory};

use super::utils::{Operands, Ret, Word};

pub fn ld(operands: Operands<'_>) -> Option<Ret> {
    if let Operands::Two(target, source) = operands {
        match (target, source) {
            (Word::U8Mut(target), Word::U8(source)) => {
                *target = source;
            }
            (Word::U16Mut(target), Word::U16(source)) => {
                *target = source;
            }
            _ => panic!("Invalid operands"),
        }
    } else {
        panic!("Incorrect number of operands")
    }
    None
}

pub fn get_ld_operands<'a>(
    registers: &'a mut CPURegisters,
    mem: &'a mut Memory,
    opcode: u8,
    value: Option<Ret>,
) -> Operands<'a> {
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
            _ => panic!("Opcode {opcode:#04x} not implemented! No match found for hi nibble"),
        };

        match lo {
            0x0 => Operands::Two(dest, Word::U8(reg_copy.b)),
            0x1 => Operands::Two(dest, Word::U8(reg_copy.c)),
            0x2 => Operands::Two(dest, Word::U8(reg_copy.d)),
            0x3 => Operands::Two(dest, Word::U8(reg_copy.e)),
            0x4 => Operands::Two(dest, Word::U8(reg_copy.h)),
            0x5 => Operands::Two(dest, Word::U8(reg_copy.l)),
            0x6 => Operands::Two(dest, Word::U8(mem_hl_val)),
            0x7 => Operands::Two(dest, Word::U8(reg_copy.a)),
            0x8 => Operands::Two(dest, Word::U8(reg_copy.b)),
            0x9 => Operands::Two(dest, Word::U8(reg_copy.c)),
            0xA => Operands::Two(dest, Word::U8(reg_copy.d)),
            0xB => Operands::Two(dest, Word::U8(reg_copy.e)),
            0xC => Operands::Two(dest, Word::U8(reg_copy.h)),
            0xD => Operands::Two(dest, Word::U8(reg_copy.l)),
            0xE => Operands::Two(dest, Word::U8(mem_hl_val)),
            0xF => Operands::Two(dest, Word::U8(reg_copy.a)),
            _ => panic!("Not implemented! No match found for lo nibble"),
        }
    } else {
        match lo {
            0x0 => {
                let value = if let Some(Ret::U8(x)) = value {
                    x
                } else {
                    panic!("Numeric Value not passed for opcode: {opcode}");
                };

                match hi {
                    0xE => Operands::Two(
                        Word::U8Mut(mem.read_u8_mut(value as u16)),
                        Word::U8(registers.a),
                    ),
                    0xF => Operands::Two(
                        Word::U8Mut(&mut registers.a),
                        Word::U8(mem.read_u8(value as u16)),
                    ),
                    _ => panic!("Not implemented!"),
                }
            }
            0x2 => {
                let source = registers.a;

                match hi {
                    0x0 => Operands::Two(
                        Word::U8Mut(mem.read_u8_mut(registers.get_bc())),
                        Word::U8(source),
                    ),
                    0x1 => Operands::Two(
                        Word::U8Mut(mem.read_u8_mut(registers.get_de())),
                        Word::U8(source),
                    ),
                    0x2 => {
                        let hl = registers.get_hl();
                        let ops = Operands::Two(Word::U8Mut(mem.read_u8_mut(hl)), Word::U8(source));
                        registers.set_hl(hl + 1);
                        ops
                    }
                    0x3 => {
                        let hl = registers.get_hl();
                        let ops = Operands::Two(Word::U8Mut(mem.read_u8_mut(hl)), Word::U8(source));
                        registers.set_hl(hl - 1);
                        ops
                    }
                    0xE => Operands::Two(
                        Word::U8Mut(mem.read_u8_mut(registers.c as u16)),
                        Word::U8(source),
                    ),
                    0xF => Operands::Two(
                        Word::U8Mut(mem.read_u8_mut(source as u16)),
                        Word::U8(registers.c),
                    ),
                    _ => panic!("Not Implemented!"),
                }
            }
            0xA => match hi {
                0x0 => Operands::Two(
                    Word::U8Mut(&mut registers.a),
                    Word::U8(mem.read_u8(reg_copy.get_bc())),
                ),
                0x1 => Operands::Two(
                    Word::U8Mut(&mut registers.a),
                    Word::U8(mem.read_u8(reg_copy.get_de())),
                ),
                0x2 => {
                    let hl = reg_copy.get_hl();
                    registers.set_hl(hl + 1);
                    let ops =
                        Operands::Two(Word::U8Mut(&mut registers.a), Word::U8(mem.read_u8(hl)));
                    ops
                }
                0x3 => {
                    let hl = registers.get_hl();
                    registers.set_hl(hl + 1);
                    let ops =
                        Operands::Two(Word::U8Mut(&mut registers.a), Word::U8(mem.read_u8(hl)));
                    ops
                }
                0xE => {
                    let value: u16 = if let Some(Ret::U16(x)) = value {
                        x
                    } else {
                        panic!("Numeric Value not passed");
                    };

                    Operands::Two(Word::U8Mut(mem.read_u8_mut(value)), Word::U8(registers.a))
                }
                0xF => {
                    let value: u16 = if let Some(Ret::U16(x)) = value {
                        x
                    } else {
                        panic!("Numeric Value not passed");
                    };

                    Operands::Two(Word::U8Mut(&mut registers.a), Word::U8(mem.read_u8(value)))
                }
                _ => panic!("Not Implemented!"),
            },

            0xE => match hi {
                0x0 => Operands::Two(
                    Word::U8Mut(&mut registers.c),
                    Word::U8(mem.read_u8(reg_copy.get_bc())),
                ),
                0x1 => Operands::Two(
                    Word::U8Mut(&mut registers.e),
                    Word::U8(mem.read_u8(reg_copy.get_de())),
                ),
                0x2 => {
                    let hl = reg_copy.get_hl();
                    registers.set_hl(hl + 1);
                    let ops =
                        Operands::Two(Word::U8Mut(&mut registers.l), Word::U8(mem.read_u8(hl)));
                    ops
                }
                0x3 => {
                    let hl = registers.get_hl();
                    registers.set_hl(hl + 1);
                    let ops =
                        Operands::Two(Word::U8Mut(&mut registers.a), Word::U8(mem.read_u8(hl)));
                    ops
                }
                _ => panic!("Not Implemented!"),
            },
            _ => panic!("Not implemented!"),
        }
    }
}

#[cfg(test)]
mod load_instruction_tests {
    use crate::emulator::instructions::*;
    use utils::Word;

    #[test]
    fn test_ld_r8_r8() {
        let source = 10;
        let mut target = 0;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: ld,
        };

        instruction.exec(Operands::Two(Word::U8Mut(&mut target), Word::U8(source)));

        assert_eq!(target, source)
    }

    #[test]
    fn test_ld_r16_r16() {
        let source = 30000;
        let mut target = 0;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: ld,
        };

        instruction.exec(Operands::Two(Word::U16Mut(&mut target), Word::U16(source)));
        assert_eq!(target, source)
    }
}