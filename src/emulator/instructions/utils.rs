use serde::{Deserialize, Serialize};

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

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct InstructionDataFlags {
    pub Z: String,
    pub N: String,
    pub H: String,
    pub C: String,
}
