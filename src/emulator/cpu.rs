mod cpu_registers;

use num_traits::{AsPrimitive, NumAssignRef};
use std::default;

use self::cpu_registers::CPURegisters;
use super::{
    instructions::{self, Instruction, Operands, Word},
    memory::Memory,
};

pub struct CPU {
    registers: CPURegisters,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            registers: CPURegisters::default(),
        }
    }
}

impl CPU {
    pub fn execute(&mut self, mem: &mut Memory) {
        let pc = self.registers.pc;
        self.registers.pc += 1;

        let opcode = mem.read_u8(pc);
        let instruction: Instruction = instructions::get_instruction(opcode);

        let operands = self.get_ld_operands(mem, opcode);

        instruction.exec(operands);

        self.registers.pc += instruction.bytes as u16;
    }

    fn get_ld_operands<'a>(&'a mut self, mem: &'a mut Memory, opcode: u8) -> Operands {
        let hi = opcode & 0xF0 >> 4;
        let lo = opcode & 0x0F;

        let reg_copy = self.registers.clone();
        // When in that nice block of load instructions
        if let 0x40..=0x7F = opcode {
            let hl = self.registers.get_hl();
            let mem_hl_val = mem.read_u8(hl).clone();

            let dest = match hi {
                0x4 => {
                    if lo <= 0x7 {
                        Word::U8Mut(&mut self.registers.b)
                    } else {
                        Word::U8Mut(&mut self.registers.c)
                    }
                }
                0x5 => {
                    if lo <= 0x7 {
                        Word::U8Mut(&mut self.registers.d)
                    } else {
                        Word::U8Mut(&mut self.registers.e)
                    }
                }
                0x6 => {
                    if lo <= 0x7 {
                        Word::U8Mut(&mut self.registers.h)
                    } else {
                        Word::U8Mut(&mut self.registers.l)
                    }
                }
                0x7 => {
                    if lo <= 0x7 {
                        Word::U8Mut(mem.read_u8_mut(hl))
                    } else {
                        Word::U8Mut(&mut self.registers.a)
                    }
                }
                _ => panic!("Not implemented!"),
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
                _ => panic!("Not implemented"),
            }
        } else {
            match lo {
                0x2 => {
                    let source = self.registers.a;

                    match hi {
                        0x0 => Operands::Two(
                            Word::U8Mut(mem.read_u8_mut(self.registers.get_bc())),
                            Word::U8(source),
                        ),
                        0x1 => Operands::Two(
                            Word::U8Mut(mem.read_u8_mut(self.registers.get_de())),
                            Word::U8(source),
                        ),
                        0x2 => {
                            let hl = self.registers.get_hl();
                            let ops =
                                Operands::Two(Word::U8Mut(mem.read_u8_mut(hl)), Word::U8(source));
                            self.registers.set_hl(hl + 1);
                            ops
                        }
                        0x3 => {
                            let hl = self.registers.get_hl();
                            let ops =
                                Operands::Two(Word::U8Mut(mem.read_u8_mut(hl)), Word::U8(source));
                            self.registers.set_hl(hl - 1);
                            ops
                        }
                        _ => panic!("Not Implemented!"),
                    }
                }
                0xA => match hi {
                    0x0 => Operands::Two(
                        Word::U8Mut(&mut self.registers.a),
                        Word::U8(mem.read_u8(reg_copy.get_bc())),
                    ),
                    0x1 => Operands::Two(
                        Word::U8Mut(&mut self.registers.a),
                        Word::U8(mem.read_u8(reg_copy.get_de())),
                    ),
                    0x2 => {
                        let hl = reg_copy.get_hl();
                        let ops = Operands::Two(
                            Word::U8Mut(&mut self.registers.a),
                            Word::U8(mem.read_u8(hl)),
                        );
                        self.registers.set_hl(hl + 1);
                        ops
                    }
                    0x3 => {
                        let hl = self.registers.get_hl();
                        let ops = Operands::Two(
                            Word::U8Mut(&mut self.registers.a),
                            Word::U8(mem.read_u8(hl)),
                        );
                        self.registers.set_hl(hl + 1);
                        ops
                    }
                    _ => panic!("Not Implemented!"),
                },

                _ => panic!("Not implemented!"),
            }
        }
    }
}
