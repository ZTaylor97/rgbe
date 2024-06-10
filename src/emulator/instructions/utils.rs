use core::fmt;
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::emulator::memory::U16Wrapper;

#[derive(Debug)]
pub enum Operands<'a> {
    None,
    One(Word<'a>, Option<&'a mut u8>),
    Two(Word<'a>, Word<'a>, Option<&'a mut u8>),
}

#[derive(Debug)]
pub enum Word<'a> {
    U8(u8),
    U8Mut(&'a mut u8),
    U16(u16),
    U16WrapperMut(U16Wrapper<'a>),
    U16Mut(&'a mut u16),
}

#[derive(Debug)]
pub enum Ret {
    U8(u8),
    U16(u16),
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct InstructionData {
    pub mnemonic: String,
    pub bytes: u8,
    pub cycles: Vec<u8>,
    pub immediate: bool,
    pub flags: InstructionDataFlags,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct InstructionDataOperands {
    pub name: String,
    pub immediate: bool,
    pub bytes: Option<u8>,
}

#[allow(non_snake_case)]
#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct InstructionDataFlags {
    pub Z: String,
    pub N: String,
    pub H: String,
    pub C: String,
}

#[derive(Debug)]
pub enum InstructionError<'a> {
    UnimplementedError(u8),
    InvalidOperandsError(Operands<'a>),
    IncorrectOperandsError(String),
    InvalidLiteral(Ret),
}

#[derive(Debug)]
pub struct BranchArgs {
    pub cycles: Vec<u8>,
    pub condition: Option<u8>, // if flags matches this
}

impl<'a> fmt::Display for InstructionError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_str = match self {
            Self::UnimplementedError(opcode) => {
                format!("Opcode {:#04x} not implemented", opcode)
            }
            Self::InvalidOperandsError(operands) => {
                format!("Operands {:?} not implemented", operands)
            }
            Self::InvalidLiteral(ret) => {
                format!(
                    "Literal value {:?} retrieve from memory is an unexpected type",
                    ret
                )
            }
            Self::IncorrectOperandsError(msg) => {
                format!("{}", msg)
            }
            _ => String::from("Unknown error encountered"),
        };

        write!(f, "{}", err_str)
    }
}

impl<'a> Error for InstructionError<'a> {}
