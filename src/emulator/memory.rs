use super::cart::RomBank;

use super::cpu::{convert_two_u8s_to_u16, convert_u16_to_two_u8s};

#[derive(Default)]
pub struct Memory {
    rom_0: RomBank,            // 0x0000 - 0x3FFF
    rom_n: RomBank,            // 0x4000 - 0x7FFF
    vram: VRAM,                //CGB switchable vram  0x8000 - 0x9FFF
    work_ram_0: Ram,           // C000 - CFFF
    work_ram_1: Ram,           // D000 - DFFF
    echo_ram: Ram,             // E000 - FDFF unused
    oam: OAM,                  // FE00 - FE9F
    _unused: Ram,              // FEA0 - FEFF
    io_reg: IORegisters,       // FF00 - FF7F
    high_ram: Ram,             // FF80 - FFFE
    interrupt_enable_reg: reg, // FFFF
}

impl Memory {
    fn new() -> Self {
        Self::default()
    }

    fn read(address: u16) -> u16 {
        match address {
            0x0000..=0x3FFF => {
                //TODO: rom_0
            }
            0x4000..=0x7FFF => {
                //TODO: rom_n
            }
            0x8000..=0x9FFF => {
                //TODO: vram
            }
            0xA000..=0xBFFF => {
                //TODO: sram (from cartridge)
            }
            0xC000..=0xCFFF => {
                //TODO: wram0
            }
            0xD000..=0xDFFF => {
                //TODO: wram1 switchable in gbc mode
            }
            0xE000..=0xFDFF => {
                //TODO: echo
            }
            0xFE00..=0xFE9F => {
                //TODO: OAM
            }
            0xFEA0..=0xFEFF => {
                //TODO:unused
            }
            0xFF00..=0xFF7F => {
                //TODO:IO registers
            }
            0xFF80..=0xFFFE => {
                //TODO:CPU RAM
            }
            0xFFFF => {
                //TODO: Interrupt enable register
            }
        }

        0
    }

    fn write_u8(value: u8, address: u16) {}
}
#[derive(Default)]
pub struct IORegisters {}
impl IORegisters {}

#[derive(Default)]
pub struct VRAM {}
impl VRAM {}
