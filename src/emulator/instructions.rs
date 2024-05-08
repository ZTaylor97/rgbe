use num_traits::NumAssignRef;

pub struct Instruction<T: NumAssignRef> {
    mnemonic: String,
    bytes: u8,
    cycles: u8,
    func: fn(Operands<T>) -> (),
}
pub enum Operands<'a, T: NumAssignRef> {
    None,
    One(T),
    OneMut(&'a mut T),
    Two(T, T),
    TwoOneMut(&'a mut T, T),
    TwoMut(&'a mut T, &'a mut T),
}
impl<T> Default for Instruction<T>
where
    T: NumAssignRef,
{
    fn default() -> Self {
        Self {
            mnemonic: String::default(),
            bytes: 0,
            cycles: 0,
            func: nop,
        }
    }
}

impl<T: NumAssignRef> Instruction<T> {
    fn exec(&mut self, params: Operands<T>) {
        (self.func)(params)
    }
}

pub fn nop<T: NumAssignRef>(operands: Operands<'_, T>) -> () {}

pub fn ld_rx_rx<T: NumAssignRef>(operands: Operands<'_, T>) {
    if let Operands::TwoOneMut(x, y) = operands {
        *x = y
    }
}

#[cfg(test)]
mod instruction_tests {

    use crate::emulator::instructions::{Instruction, Operands};

    #[test]
    fn test_ld_r8_r8() {
        let source = 10;
        let mut target = 0;

        let mut instruction = Instruction {
            mnemonic: Default::default(),
            bytes: 0,
            cycles: 4,
            func: super::ld_rx_rx,
        };

        instruction.exec(Operands::TwoOneMut(&mut target, source));

        assert_eq!(target, source)
    }
    fn test_ld_r16_r16() {
        let source = 30000;
        let mut target = 0;

        let mut instruction = Instruction {
            mnemonic: Default::default(),
            bytes: 0,
            cycles: 4,
            func: super::ld_rx_rx,
        };

        instruction.exec(Operands::TwoOneMut(&mut target, source));
        assert_eq!(target, source)
    }
}
