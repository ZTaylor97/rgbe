mod load;
mod utils;
use std::fs;

use super::{cpu::cpu_registers::CPURegisters, memory::Memory};
use load::*;
use utils::{InstructionData, Operands, Ret};

#[derive(Clone)]
pub struct Instruction {
    pub data: InstructionData,
    func: fn(Operands) -> Option<Ret>,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            data: InstructionData::default(),
            func: nop,
        }
    }
}

impl Instruction {
    fn exec(&self, params: Operands) -> Option<Ret> {
        (self.func)(params)
    }
}

pub fn execute_instruction(
    instruction: &Instruction,
    registers: &mut CPURegisters,
    memory: &mut Memory,
) -> u8 {
    let opcode = memory.read_u8(registers.pc);

    let value = match instruction.data.bytes {
        1 => None,
        2 => Some(Ret::U8(memory.read_u8(registers.pc + 1))),
        3 => Some(Ret::U16(memory.read_u16(registers.pc + 1))),
        _ => panic!("Bytes is invalid"),
    };

    let operands = get_ld_operands(registers, memory, opcode, value);

    instruction.exec(operands);

    registers.pc += (instruction.data.bytes + 1) as u16;

    instruction.data.cycles[0]
}

pub fn fetch_instructions() -> Vec<Instruction> {
    let json_string = fs::read_to_string("instructions.json").expect("File not found");

    let json: serde_json::Value = serde_json::from_str(json_string.as_str()).expect("Invalid JSON");

    let array = json.get("instructions").unwrap().to_string();

    let instructions_data: Vec<InstructionData> = serde_json::from_str(array.as_str()).unwrap();

    instructions_data
        .into_iter()
        .map(|data| {
            let func = match data.mnemonic.as_str() {
                "LD" => ld,
                "NOP" => nop,
                _ => nop,
            };

            Instruction { data, func }
        })
        .collect()
}

pub fn nop(operands: Operands<'_>) -> Option<Ret> {
    None
}

#[cfg(test)]
mod instruction_integration_tests {
    use crate::emulator::{cpu::cpu_registers::CPURegisters, memory::Memory};

    use super::{execute_instruction, fetch_instructions, Instruction};

    #[test]
    fn test_execute_ld_rx_rx_instruction() {
        let mut registers = CPURegisters::default();
        let mut memory = Memory::default();
        let instructions: Vec<Instruction> = fetch_instructions();

        assert_eq!(registers.pc, 0);

        // First instruction should be: LD B, C
        let instruction = 0x41;
        memory.write_u8(0x0, instruction);
        registers.b = 18;
        registers.c = 60;

        execute_instruction(
            &instructions[instruction as usize],
            &mut registers,
            &mut memory,
        );

        assert_eq!(registers.pc, 2);
        assert_eq!(registers.b, 60);
        assert_eq!(registers.c, 60);
    }
    #[test]
    fn test_execute_ld_rx_hlmem_instruction() {
        let desired_result = 69;
        let target_address = 0x0101;

        let mut registers = CPURegisters::default();
        let mut memory = Memory::default();
        let instructions: Vec<Instruction> = fetch_instructions();

        assert_eq!(registers.pc, 0);

        // First instruction should be: LD [HL], B
        let instruction = 0x70;
        registers.b = desired_result;
        registers.set_hl(target_address);
        memory.write_u8(0x0, instruction);
        memory.write_u8(target_address, 0);

        execute_instruction(
            &instructions[instruction as usize],
            &mut registers,
            &mut memory,
        );

        assert_eq!(registers.pc, 2);
        assert_eq!(registers.b, desired_result);
        assert_eq!(registers.get_hl(), target_address);
        assert_eq!(memory.read_u8(target_address), desired_result);
    }
    #[test]
    fn test_execute_ld_a16mem_rx_instruction() {
        let desired_result: u8 = 69;

        let mut registers = CPURegisters::default();
        let mut memory = Memory::default();
        let instructions: Vec<Instruction> = fetch_instructions();

        assert_eq!(registers.pc, 0);

        // First instruction should be: LD [a16], A
        let instruction = 0xEA;
        let address: u16 = 0x0101;
        memory.write_u8(0x0, instruction);
        memory.write_u16(0x01, address);

        registers.a = desired_result;

        execute_instruction(
            &instructions[instruction as usize],
            &mut registers,
            &mut memory,
        );

        assert_eq!(registers.pc, 4);
        assert_eq!(memory.read_u8(address), desired_result);
    }
    #[test]
    fn test_execute_ld_rx_a16mem_instruction() {
        let desired_result: u8 = 69;

        let mut registers = CPURegisters::default();
        let mut memory = Memory::default();
        let instructions: Vec<Instruction> = fetch_instructions();

        assert_eq!(registers.pc, 0);

        // First instruction should be: LD A, [a16]
        let instruction = 0xFA;
        let address: u16 = 0x0101;
        memory.write_u8(0x0, instruction);
        memory.write_u16(0x01, address);
        memory.write_u8(address, desired_result);

        registers.a = 0;

        execute_instruction(
            &instructions[instruction as usize],
            &mut registers,
            &mut memory,
        );

        assert_eq!(registers.pc, 4);
        assert_eq!(registers.a, desired_result);
    }
}
