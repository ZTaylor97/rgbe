use num_traits::{ops::overflowing::OverflowingSub, WrappingAdd};

use crate::emulator::{cpu::cpu_registers::CPURegisters, memory::Memory};

use super::utils::{InstructionError, Operands, Ret, Word};

pub fn inc(operands: Operands<'_>) -> Result<Option<Ret>, InstructionError> {
    if let Operands::One(target, flags) = operands {
        match (target) {
            Word::U8Mut(target) => {
                *target = ncrementu8_with_flags(*target, true, flags.unwrap());
                Ok(None)
            }
            Word::U16Mut(target) => {
                *target += 1;
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
            Word::U16Mut(target) => {
                *target -= 1;
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
