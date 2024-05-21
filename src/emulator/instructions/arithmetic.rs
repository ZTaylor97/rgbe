use crate::emulator::{cpu::cpu_registers::CPURegisters, memory::Memory};

use super::utils::{Operands, Ret, Word};

pub fn add(operands: Operands<'_>) -> Option<Ret> {
    if let Operands::Two(target, source) = operands {
        match (target, source) {
            (Word::U8Mut(target), Word::U8(source)) => {
                let result: (u8, bool) = target.overflowing_add(source);
                let carry = result.1 as u8;
                *target = result.0;

                let zero = if *target == 0 { 1 } else { 0 };

                let half_carry = if ((*target & 0x0f) + (source & 0x0f)) > 0x0f {
                    1
                } else {
                    0
                };

                let negative = 0;

                let ret: u8 = (zero << 7) | (negative << 6) | (half_carry << 5) | (carry << 4);

                Some(Ret::U8(ret))
            }
            _ => panic!("Invalid operands"),
        }
    } else {
        panic!("Incorrect number of operands")
    }
}

pub fn get_arithmetic_operands<'a>(
    registers: &'a mut CPURegisters,
    mem: &'a mut Memory,
    opcode: u8,
    value: Option<Ret>,
) -> Operands<'a> {
    let hi = (opcode & 0xF0) >> 4;
    let lo = opcode & 0x0F;

    let reg_copy = registers.clone();

    let hl = registers.get_hl();
    let mem_hl_val = mem.read_u8(hl).clone();

    // When in that nice block of load instructions
    if let 0x80..=0xBF = opcode {
        let dest = Word::U8Mut(&mut registers.a);
        let source = match hi {
            0x0 | 0x8 => Word::U8(registers.b),
            0x1 | 0x9 => Word::U8(registers.c),
            0x2 | 0xA => Word::U8(registers.d),
            0x3 | 0xB => Word::U8(registers.e),
            0x4 | 0xC => Word::U8(registers.h),
            0x5 | 0xD => Word::U8(registers.l),
            0x6 | 0xE => Word::U8(mem_hl_val),
            0x7 | 0xF => Word::U8(reg_copy.a),
            _ => panic!("Opcode {opcode:#04x} not implemented! No match found for hi nibble"),
        };

        Operands::Two(dest, source)
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
mod arithmetic_instruction_tests {
    use crate::emulator::instructions::*;
    use arithmetic::add;
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

        instruction.exec(Operands::Two(Word::U8Mut(&mut target), Word::U8(source)));

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

        let result = instruction.exec(Operands::Two(Word::U8Mut(&mut target), Word::U8(source)));
        if let Some(Ret::U8(x)) = result {
            assert_eq!(x, 0b10000000)
        }

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

        let result = instruction.exec(Operands::Two(Word::U8Mut(&mut target), Word::U8(source)));
        if let Some(Ret::U8(x)) = result {
            assert_eq!(x, 0b00010000)
        }

        assert_eq!(target, desired_result);
    }
}
