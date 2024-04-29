use super::cart::RomBank;

use super::cpu::{convert_two_u8s_to_u16, convert_u16_to_two_u8s};

#[derive(Default)]
pub struct Memory {
    rom_0: RomBank,            // 0x0000 - 0x3FFF
    rom_n: RomBank,            // 0x4000 - 0x7FFF
    vram: VRAM,                //CGB switchable vram  0x8000 - 0x9FFF
    work_ram_0: RAM,           // C000 - CFFF
    work_ram_1: RAM,           // D000 - DFFF
    echo_ram: RAM,             // E000 - FDFF unused
    oam: OAM,                  // FE00 - FE9F
    _unused: RAM,              // FEA0 - FEFF
    io_reg: IORegisters,       // FF00 - FF7F
    high_ram: RAM,             // FF80 - FFFF
    interrupt_enable_reg: , // FFFF
}

impl Memory {
    fn new() -> Self {
        Self::default()
    }

    fn read_u8(&self, address: u16) -> u8 {
        return match address {
            0x0000..=0x3FFF => {
               self.rom_0.read_u8(address) 
            }
            0x4000..=0x7FFF => {
                self.rom_n.read_u8(address - 0x4000)
            }
            0x8000..=0x9FFF => {
                self.vram.read_u8(address - 0x8000)
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
                //TODO:CPU HRAM
            }
            0xFFFF => {
                //TODO: Interrupt enable register
            }
        }
    }

    fn read_u16(&self, address: u16) -> u16 {
        (self.read_u8(address) as u16) | ((self.read_u8(address + 1) as u16) << 8)
    }

    fn write_u8(&mut self, value: u8, address: u16) {}

    fn write_u16(&mut self, value: u8, address: u16) {}
}
#[derive(Default)]
pub struct IORegisters {}
impl IORegisters {}

pub struct VRAM {
    buf: [u8; 0x1000]
}
impl VRAM {}
impl Default for VRAM {
    fn default() -> Self {
        Self { buf: [0; 0x1000] }
    }
}

impl ReadBuffer for VRAM {
    fn read_u8(&self, address: u16) -> u8 {
        *self
            .buf
            .get(address as usize)
            .expect("Error reading Rom Buffer")
    }

    fn read_u16(&self, address: u16) ->u16 {
        todo!()
    }
}


#[derive(Default)]
pub struct SRAM {}
impl SRAM {}


#[derive(Default)]
pub struct HRAM {}
impl HRAM {}

pub trait ReadBuffer {
    fn read_u8(&self, address: u16) -> u8{
        0
    }
    fn read_u16(&self, address: u16) ->u16{
        0
    }
}
pub trait WriteBuffer {
    fn write_u8() -> u8;
    fn write_u16() ->u16;
}
