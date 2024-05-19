use num_traits::NumAssignRef;

use super::{cpu::cpu_registers::CPURegisters, memory::Memory};

pub enum Operands<'a> {
    None,
    One(Word<'a>),
    Two(Word<'a>, Word<'a>),
}

pub enum Word<'a> {
    U8(u8),
    U8Mut(&'a mut u8),
    U16(u16),
    U16Mut(&'a mut u16),
}

pub enum Ret {
    U8(u8),
    U16(u16),
}

pub struct Instruction {
    pub mnemonic: String,
    pub bytes: u8,
    pub cycles: u8,
    func: fn(Operands) -> Option<Ret>,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            mnemonic: String::default(),
            bytes: 0,
            cycles: 0,
            func: nop,
        }
    }
}

impl Instruction {
    pub fn exec(&self, params: Operands) -> Option<Ret> {
        (self.func)(params)
    }
}

pub fn execute_instruction(registers: &mut CPURegisters, memory: &mut Memory) {
    let opcode = memory.read_u8(registers.pc);
    let instruction = match opcode {
        0xEA => Instruction {
            mnemonic: String::from("LD"),
            bytes: 3,
            cycles: 4,
            func: ld,
        },
        _ => Instruction {
            mnemonic: String::from("LD"),
            bytes: 1,
            cycles: 4,
            func: ld,
        },
    };

    let opcode = memory.read_u8(registers.pc);

    let value = match instruction.bytes {
        1 => None,
        2 => Some(Ret::U8(memory.read_u8(registers.pc + 1))),
        3 => Some(Ret::U16(memory.read_u16(registers.pc + 1))),
        _ => panic!("Bytes is invalid"),
    };

    let operands = get_ld_operands(registers, memory, opcode, value);

    instruction.exec(operands);

    registers.pc += (instruction.bytes + 1) as u16;

    // TODO: todo!("Implement instruction fetching")
}

pub fn nop(operands: Operands<'_>) -> Option<Ret> {
    None
}

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
                    panic!("Numeric Value not passed");
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
mod instruction_tests {

    use crate::emulator::instructions::{Instruction, Operands, Word};

    #[test]
    fn test_ld_r8_r8() {
        let source = 10;
        let mut target = 0;

        let instruction = Instruction {
            mnemonic: Default::default(),
            bytes: 0,
            cycles: 4,
            func: super::ld,
        };

        instruction.exec(Operands::Two(Word::U8Mut(&mut target), Word::U8(source)));

        assert_eq!(target, source)
    }

    #[test]
    fn test_ld_r16_r16() {
        let source = 30000;
        let mut target = 0;

        let instruction = Instruction {
            mnemonic: Default::default(),
            bytes: 0,
            cycles: 4,
            func: super::ld,
        };

        instruction.exec(Operands::Two(Word::U16Mut(&mut target), Word::U16(source)));
        assert_eq!(target, source)
    }
}

#[cfg(test)]
mod instruction_integration_tests {
    use crate::emulator::{cpu::cpu_registers::CPURegisters, memory::Memory};

    use super::execute_instruction;

    #[test]
    fn test_execute_ld_rx_rx_instruction() {
        let mut registers = CPURegisters::default();
        let mut memory = Memory::default();

        assert_eq!(registers.pc, 0);

        // First instruction should be: LD B, C
        memory.write_u8(0x0, 0x41);
        registers.b = 18;
        registers.c = 60;

        execute_instruction(&mut registers, &mut memory);

        assert_eq!(registers.pc, 2);
        assert_eq!(registers.b, 60);
        assert_eq!(registers.c, 60);
    }
    #[test]
    fn test_execute_ld_rx_hl_mem_instruction() {
        let desired_result = 69;
        let target_address = 0x0101;

        let mut registers = CPURegisters::default();
        let mut memory = Memory::default();

        assert_eq!(registers.pc, 0);

        // First instruction should be: LD [HL], B
        let instruction = 0x70;
        memory.write_u8(0x0, instruction);
        registers.b = desired_result;
        registers.set_hl(target_address);
        memory.write_u8(target_address, 0);

        execute_instruction(&mut registers, &mut memory);

        assert_eq!(registers.pc, 2);
        assert_eq!(registers.b, desired_result);
        assert_eq!(registers.get_hl(), target_address);
        assert_eq!(memory.read_u8(target_address), desired_result);
    }
    #[test]
    fn test_execute_ld_a16_mem_rx_instruction() {
        let desired_result: u8 = 69;

        let mut registers = CPURegisters::default();
        let mut memory = Memory::default();

        assert_eq!(registers.pc, 0);

        // First instruction should be: LD [a16], A
        let instruction = 0xEA;
        memory.write_u8(0x0, instruction);

        let address: u16 = 0x0101;
        memory.write_u16(0x01, address);

        registers.a = desired_result;

        execute_instruction(&mut registers, &mut memory);

        assert_eq!(registers.pc, 4);
        assert_eq!(memory.read_u8(address), desired_result);
    }
}
