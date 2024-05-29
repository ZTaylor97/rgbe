use num_traits::{ops::overflowing::OverflowingSub, WrappingAdd, WrappingSub};

use super::utils::{InstructionError, Operands, Ret, Word};
use crate::emulator::{
    cpu::cpu_registers::CPURegisters,
    memory::{Memory, U16Wrapper},
};

pub fn inc(operands: Operands<'_>) -> Result<Option<Ret>, InstructionError> {
    if let Operands::One(target, flags) = operands {
        match (target) {
            Word::U8Mut(target) => {
                *target = ncrementu8_with_flags(*target, true, flags.unwrap());
                Ok(None)
            }
            Word::U16WrapperMut(target) => {
                let mut val: u16 = target.into_u16();
                val.wrapping_add(1);
                target.from_u16(val);
                Ok(None)
            }
            Word::U16Mut(target) => {
                *target = target.wrapping_add(1);
                Ok(None)
            }
            (word1) => {
                return Err(InstructionError::IncorrectOperandsError(format!(
                    "Incorrect word {:?} passed to function inc",
                    word1
                )));
            }
        }
    } else {
        return Err(InstructionError::InvalidOperandsError(operands));
    }
}
pub fn dec(operands: Operands<'_>) -> Result<Option<Ret>, InstructionError> {
    if let Operands::One(target, flags) = operands {
        match (target) {
            Word::U8Mut(target) => {
                *target = ncrementu8_with_flags(*target, false, flags.unwrap());
                Ok(None)
            }
            Word::U16WrapperMut(target) => {
                let mut val: u16 = target.into_u16();
                val.wrapping_sub(1);
                target.from_u16(val);
                Ok(None)
            }
            Word::U16Mut(target) => {
                *target = target.wrapping_sub(1);
                Ok(None)
            }
            (word1) => {
                return Err(InstructionError::IncorrectOperandsError(format!(
                    "Incorrect word {:?} passed to function dec",
                    word1
                )));
            }
        }
    } else {
        return Err(InstructionError::InvalidOperandsError(operands));
    }
}

fn ncrementu8_with_flags(target: u8, increment: bool, flags: &mut u8) -> u8 {
    let (result, negative) = if increment {
        (target.overflowing_add(1), 0)
    } else {
        (target.overflowing_sub(1), 1)
    };
    let zero = if result.0 == 0 { 1 } else { 0 };
    let half_carry = if (result.0 & 0x0f) == 0x0f { 1 } else { 0 };
    *flags |= (zero << 7) | (negative << 6) | (half_carry << 5);

    result.0
}

pub fn get_ncrement_operands<'a>(
    registers: &'a mut CPURegisters,
    mem: &'a mut Memory,
    opcode: u8,
    value: Option<Ret>,
) -> Result<Operands<'a>, InstructionError<'a>> {
    let hl = registers.get_hl();

    let dest = match opcode {
        0x03 | 0x08 => Word::U16WrapperMut(U16Wrapper(&mut registers.b, &mut registers.c)),
        0x13 | 0x18 => Word::U16WrapperMut(U16Wrapper(&mut registers.d, &mut registers.e)),
        0x23 | 0x28 => Word::U16WrapperMut(U16Wrapper(&mut registers.h, &mut registers.l)),
        0x33 | 0x38 => Word::U16Mut(&mut registers.sp),
        0x04 | 0x05 => Word::U8Mut(&mut registers.b),
        0x14 | 0x15 => Word::U8Mut(&mut registers.d),
        0x24 | 0x25 => Word::U8Mut(&mut registers.h),
        0x34 | 0x35 => Word::U8Mut(mem.read_u8_mut(hl)),
        0x0C | 0x0D => Word::U8Mut(&mut registers.c),
        0x1C | 0x1D => Word::U8Mut(&mut registers.e),
        0x2C | 0x2D => Word::U8Mut(&mut registers.l),
        0x3C | 0x3D => Word::U8Mut(&mut registers.a),
        _ => return Err(InstructionError::UnimplementedError(opcode)),
    };

    Ok(Operands::One(dest, Some(&mut registers.f)))
}
#[cfg(test)]
mod ncrement_instruction_tests {
    use crate::emulator::{
        cpu::cpu_registers::convert_u16_to_two_u8s, instructions::*, memory::U16Wrapper,
    };
    use utils::Word;

    #[test]
    fn test_inc_r8() {
        let mut target = 0;
        let desired_value = target + 1;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: inc,
        };

        let mut flags = 0;

        instruction.exec(Operands::One(Word::U8Mut(&mut target), Some(&mut flags)));

        assert_eq!(target, desired_value)
    }
    #[test]
    fn test_dec_r8() {
        let mut target = 1;
        let desired_value = target - 1;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: dec,
        };

        let mut flags = 0;

        instruction.exec(Operands::One(Word::U8Mut(&mut target), Some(&mut flags)));

        assert_eq!(target, desired_value)
    }
    #[test]
    fn test_inc_r16() {
        let mut target = 1000;
        let desired_value = target + 1;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: inc,
        };

        let mut flags = 0;

        instruction.exec(Operands::One(Word::U16Mut(&mut target), Some(&mut flags)));

        assert_eq!(target, desired_value)
    }
    #[test]
    fn test_dec_r16() {
        let mut target = 1000;
        let desired_value = target - 1;

        let instruction = Instruction {
            data: InstructionData::default(),
            func: dec,
        };

        let mut flags = 0;

        instruction.exec(Operands::One(Word::U16Mut(&mut target), Some(&mut flags)));

        assert_eq!(target, desired_value)
    }
}
