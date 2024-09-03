#![allow(unused)]
mod arithmetic;
mod increment;
mod jump;
mod load;
mod utils;
mod stack;

use std::fs;

use crate::emulator::instructions::jump::get_jump_operands;

use super::{cpu::cpu_registers::CPURegisters, memory::Memory};
use arithmetic::*;
use increment::*;
use jump::*;
use load::*;
use stack::*;
use utils::{Args, BranchArgs, InstructionData, InstructionError, Operands, Ret};

#[derive(Clone)]
pub struct Instruction {
    pub data: InstructionData,
    func: fn(Operands, BranchArgs) -> Result<u8, InstructionError>,
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
    fn exec(&self, params: Operands, branch_args: BranchArgs) -> u8 {
        match (self.func)(params, branch_args) {
            Ok(ret) => ret,
            Err(e) => panic!("{}", e),
        }
    }
}

pub fn execute_instruction(
    instruction: &Instruction,
    registers: &mut CPURegisters,
    memory: &mut Memory,
) -> u8 {
    let opcode = memory.read_u8(registers.pc);

    let value: Option<Ret> = match instruction.data.bytes {
        1 => None,
        2 => Some(Ret::U8(memory.read_u8(registers.pc + 1))),
        3 => Some(Ret::U16(memory.read_u16(registers.pc + 1))),
        _ => panic!("Bytes is invalid"),
    };

    // TODO: debug
    println!(
        "Executing instruction - {:#04x}:\n\tInstruction Data - {:?}\n\tPC - {}\n\tSP - {}",
        opcode, instruction.data, registers.pc, registers.sp
    );
    registers.pc += (instruction.data.bytes) as u16;

    let get_operands_result: Result<Args, InstructionError> =
        match instruction.data.mnemonic.as_str() {
            "LD" => get_ld_operands(registers, memory, opcode, value),
            "ADD" | "ADC" | "SUB" | "SBC" | "XOR" | "OR" | "AND" | "CP" => {
                get_arithmetic_operands(registers, memory, opcode, value)
            }
            "INC" | "DEC" => get_ncrement_operands(registers, memory, opcode, value),
            "JP" | "JR" => get_jump_operands(registers, memory, opcode, value),
            _ => panic!(
                "{}:\n\tInstruction Data - {:?}\n\tPC - {}\n\tSP - {}",
                InstructionError::UnimplementedError(opcode),
                instruction.data,
                registers.pc,
                registers.sp
            ),
        };

    // TODO: debug
    println!("\tOperands - {:?}", get_operands_result);

    let instruction_cycles: u8 = match get_operands_result {
        Ok((operands, condition)) => instruction.exec(
            operands,
            BranchArgs {
                cycles: instruction.data.cycles.clone(),
                condition,
            },
        ),
        Err(e) => panic!("{}", e),
    };

    instruction_cycles
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
                "ADD" => add,
                "ADC" => adc,
                "SUB" => sub,
                "SBC" => sbc,
                "XOR" => xor,
                "AND" => and,
                "OR" => or,
                "CP" => cp,
                "INC" => inc,
                "DEC" => dec,
                "JP" => jp,
                "JR" => jr,
                _ => nop,
            };

            Instruction { data, func }
        })
        .collect()
}

pub fn nop(operands: Operands<'_>, branch_args: BranchArgs) -> Result<u8, InstructionError> {
    Ok(branch_args.cycles[0])
}

#[cfg(test)]
mod instruction_integration_tests {
    use crate::emulator::{
        cpu::cpu_registers::{convert_u16_to_two_u8s, CPURegisters},
        memory::Memory,
    };

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

        assert_eq!(registers.pc, 1);
        assert_eq!(registers.b, 60);
        assert_eq!(registers.c, 60);
    }
    #[test]
    fn test_execute_ld_r16_n16_instruction() {
        let mut registers = CPURegisters::default();
        let mut memory = Memory::default();
        let instructions: Vec<Instruction> = fetch_instructions();

        let desired_value = 6666;
        let expected_values = convert_u16_to_two_u8s(desired_value);

        assert_eq!(registers.pc, 0);

        // First instruction should be: LD B, C
        let instruction = 0x01;
        memory.write_u8(0x0, instruction);
        memory.write_u16(0x1, desired_value);
        registers.set_bc(desired_value);

        execute_instruction(
            &instructions[instruction as usize],
            &mut registers,
            &mut memory,
        );

        assert_eq!(registers.pc, 3);
        assert_eq!(registers.b, expected_values.0);
        assert_eq!(registers.c, expected_values.1);
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

        assert_eq!(registers.pc, 1);
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

        assert_eq!(registers.pc, 3);
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

        assert_eq!(registers.pc, 3);
        assert_eq!(registers.a, desired_result);
    }
    #[test]
    fn test_execute_add_rx_rx_instruction() {
        let mut registers = CPURegisters::default();
        let mut memory = Memory::default();
        let instructions: Vec<Instruction> = fetch_instructions();

        assert_eq!(registers.pc, 0);

        // First instruction should be: LD A, [a16]
        let instruction = 0x80;

        memory.write_u8(0x0, instruction);
        registers.a = 10;
        registers.b = 20;

        let desired_result: u8 = registers.a + registers.b;

        execute_instruction(
            &instructions[instruction as usize],
            &mut registers,
            &mut memory,
        );

        assert_eq!(registers.pc, 1);
        assert_eq!(registers.a, desired_result);
    }
    #[test]
    fn test_execute_adc_rx_rx_instruction() {
        let mut registers = CPURegisters::default();
        let mut memory = Memory::default();
        let instructions: Vec<Instruction> = fetch_instructions();

        assert_eq!(registers.pc, 0);

        // First instruction should be: LD A, [a16]
        let instruction = 0x88;

        memory.write_u8(0x0, instruction);
        registers.a = 10;
        registers.b = 20;

        let desired_result: u8 = registers.a + registers.b;

        execute_instruction(
            &instructions[instruction as usize],
            &mut registers,
            &mut memory,
        );

        assert_eq!(registers.pc, 1);
        assert_eq!(registers.a, desired_result);
    }
    #[test]
    fn test_execute_sub_rx_rx_instruction() {
        let mut registers = CPURegisters::default();
        let mut memory = Memory::default();
        let instructions: Vec<Instruction> = fetch_instructions();

        assert_eq!(registers.pc, 0);

        // First instruction should be: LD A, [a16]
        let instruction = 0x90;

        memory.write_u8(0x0, instruction);
        registers.a = 30;
        registers.b = 20;

        let desired_result: u8 = registers.a - registers.b;

        execute_instruction(
            &instructions[instruction as usize],
            &mut registers,
            &mut memory,
        );

        assert_eq!(registers.pc, 1);
        assert_eq!(registers.a, desired_result);
    }
    #[test]
    fn test_execute_sbc_rx_rx_instruction() {
        let mut registers = CPURegisters::default();
        let mut memory = Memory::default();
        let instructions: Vec<Instruction> = fetch_instructions();

        assert_eq!(registers.pc, 0);

        // First instruction should be: LD A, [a16]
        let instruction = 0x98;

        memory.write_u8(0x0, instruction);
        registers.a = 30;
        registers.b = 20;

        let desired_result: u8 = registers.a - registers.b;

        execute_instruction(
            &instructions[instruction as usize],
            &mut registers,
            &mut memory,
        );

        assert_eq!(registers.pc, 1);
        assert_eq!(registers.a, desired_result);
    }
    #[test]
    fn test_execute_jp_zc_instruction() {
        let mut registers = CPURegisters::default();
        let mut memory = Memory::default();
        let instructions: Vec<Instruction> = fetch_instructions();

        assert_eq!(registers.pc, 0);

        // First instruction should be: LD A, [a16]
        let instruction = 0x98;

        memory.write_u8(0x0, instruction);
        registers.a = 30;
        registers.b = 20;

        let desired_result: u8 = registers.a - registers.b;

        execute_instruction(
            &instructions[instruction as usize],
            &mut registers,
            &mut memory,
        );

        assert_eq!(registers.pc, 1);
        assert_eq!(registers.a, desired_result);
    }
}
