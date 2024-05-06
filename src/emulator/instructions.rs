pub struct Instruction {
    mnemonic: String,
    bytes: u8,
    cycles: u8,
}
enum Operands<'a> {
    None,
    One(&'a u8),
    OneMut(&'a mut u8),
    Two(&'a u8, &'a u8),
    TwoOneMut(&'a mut u8, &'a u8),
    TwoMut(&'a mut u8, &'a mut u8),
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            mnemonic: String::default(),
            bytes: 0,
            cycles: 0,
        }
    }
}

// pub fn ld_r8_r8(operands: &mut Operands<'_>) {
//     if let Operands::TwoOneMut(&mut mut x, &y) = *operands {
//         x = y;
//     } else {
//         panic!("Wrong operands passed to function")
//     }
// }

#[cfg(test)]
mod instruction_tests {

    use crate::emulator::instructions::{Instruction, Operands};

    #[test]
    fn test_ld_r8_r8() {
        let mut target = 0;

        let nop = Instruction {
            mnemonic: Default::default(),
            bytes: 0,
            cycles: 4,
        };
        let source = 10;

        let mut op = Operands::TwoOneMut(&mut target, &source);

        ld_r8_r8(&mut op);
        assert_eq!(target, 10)
    }
}
