use num_traits::NumAssignRef;

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

pub fn get_instruction(idx: u8) -> Instruction {
    todo!("Implement instruction fetching")
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

#[cfg(test)]
mod instruction_tests {

    use crate::emulator::instructions::{Instruction, Operands, Word};

    #[test]
    fn test_ld_r8_r8() {
        let source = 10;
        let mut target = 0;

        let mut instruction = Instruction {
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

        let mut instruction = Instruction {
            mnemonic: Default::default(),
            bytes: 0,
            cycles: 4,
            func: super::ld,
        };

        instruction.exec(Operands::Two(Word::U16Mut(&mut target), Word::U16(source)));
        assert_eq!(target, source)
    }
}
