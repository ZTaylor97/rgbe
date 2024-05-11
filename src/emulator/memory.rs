use std::ops::Index;

#[derive(Default)]
pub struct Memory {
    rom_0: Buffer<0x4000>,      // 0x0000 - 0x3FFF
    rom_n: Buffer<0x4000>,      // 0x4000 - 0x7FFF
    vram: Buffer<0x2000>,       //CGB switchable vram  0x8000 - 0x9FFF
    sram: Buffer<0x2000>,       // A000 - BFFF (cartridge RAM)
    work_ram_0: Buffer<0x1000>, // C000 - CFFF
    work_ram_n: Buffer<0x1000>, // D000 - DFFF
    echo_ram: Buffer<0x1DFF>,   // E000 - FDFF unused
    oam: Buffer<0x9F>,          // FE00 - FE9F
    io_reg: IORegisters,        // FF00 - FF7F
    high_ram: Buffer<0x7F>,     // FF80 - FFFF
}

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_u8_mut(&mut self, address: u16) -> &mut u8 {
        return match address {
            0x0000..=0x3FFF => self.rom_0.read_u8_mut(address),
            0x4000..=0x7FFF => self.rom_n.read_u8_mut(address - 0x4000),
            0x8000..=0x9FFF => self.vram.read_u8_mut(address - 0x8000),
            0xA000..=0xBFFF => self.sram.read_u8_mut(address - 0xA000),
            0xC000..=0xCFFF => self.work_ram_0.read_u8_mut(address - 0xC000),
            0xD000..=0xDFFF => self.work_ram_n.read_u8_mut(address - 0xD000),
            0xE000..=0xFDFF => self.echo_ram.read_u8_mut(address - 0xE000),
            0xFE00..=0xFE9F => self.oam.read_u8_mut(address - 0xFE00),
            0xFEA0..=0xFEFF => {
                panic!("Unuseable memory accessed")
            }
            0xFF00..=0xFF7F => self.io_reg.read_u8_mut(address - 0xFF00),
            0xFF80..=0xFFFF => self.high_ram.read_u8_mut(address - 0xFF80),
        };
    }
    pub fn read_u8(&self, address: u16) -> u8 {
        return match address {
            0x0000..=0x3FFF => self.rom_0.read_u8(address),
            0x4000..=0x7FFF => self.rom_n.read_u8(address - 0x4000),
            0x8000..=0x9FFF => self.vram.read_u8(address - 0x8000),
            0xA000..=0xBFFF => self.sram.read_u8(address - 0xA000),
            0xC000..=0xCFFF => self.work_ram_0.read_u8(address - 0xC000),
            0xD000..=0xDFFF => self.work_ram_n.read_u8(address - 0xD000),
            0xE000..=0xFDFF => self.echo_ram.read_u8(address - 0xE000),
            0xFE00..=0xFE9F => self.oam.read_u8(address - 0xFE00),
            0xFEA0..=0xFEFF => {
                panic!("Unuseable memory accessed")
            }
            0xFF00..=0xFF7F => self.io_reg.read_u8(address - 0xFF00),
            0xFF80..=0xFFFF => self.high_ram.read_u8(address - 0xFF80),
        };
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        (self.read_u8(address) as u16) | ((self.read_u8(address + 1) as u16) << 8)
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x3FFF => self.rom_0.write_u8(address, value),
            0x4000..=0x7FFF => self.rom_n.write_u8(address - 0x4000, value),
            0x8000..=0x9FFF => self.vram.write_u8(address - 0x8000, value),
            0xA000..=0xBFFF => self.sram.write_u8(address - 0xA000, value),
            0xC000..=0xCFFF => self.work_ram_0.write_u8(address - 0xC000, value),
            0xD000..=0xDFFF => self.work_ram_n.write_u8(address - 0xD000, value),
            0xE000..=0xFDFF => self.echo_ram.write_u8(address - 0xE000, value),
            0xFE00..=0xFE9F => self.oam.write_u8(address - 0xFE00, value),
            0xFEA0..=0xFEFF => {
                panic!("Unuseable memory accessed")
            }
            0xFF00..=0xFF7F => self.io_reg.write_u8(address - 0xFF00, value),
            0xFF80..=0xFFFF => self.high_ram.write_u8(address - 0xFF80, value),
        }
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        self.write_u8(address, (value & 0x00ff) as u8);
        self.write_u8(address + 1, ((value & 0xff00) >> 8) as u8);
    }
}

// TODO: implement IO registers properly, might be worth putting them in another module
pub struct IORegisters {
    buf: [u8; 0x100],
}
impl Default for IORegisters {
    fn default() -> Self {
        Self { buf: [0; 0x100] }
    }
}
impl ReadBuffer for IORegisters {
    fn read_u8_mut(&mut self, address: u16) -> &mut u8 {
        self.buf
            .get_mut(address as usize)
            .expect("Error reading Rom Buffer")
    }
    fn read_u8(&self, address: u16) -> u8 {
        *self.buf.get(address as usize).expect("")
    }
}
impl WriteBuffer for IORegisters {
    fn write_u8(&mut self, address: u16, value: u8) {
        self.buf[address as usize] = value;
    }
}

pub struct Buffer<const N: usize> {
    pub buf: [u8; N],
}
impl<const N: usize> Default for Buffer<N> {
    fn default() -> Self {
        Buffer { buf: [0; N] }
    }
}
impl<const N: usize> ReadBuffer for Buffer<N> {
    fn read_u8_mut(&mut self, address: u16) -> &mut u8 {
        self.buf.get_mut(address as usize).expect("")
    }
    fn read_u8(&self, address: u16) -> u8 {
        self.buf.get(address as usize).expect("").clone()
    }
}
impl<const N: usize> WriteBuffer for Buffer<N> {
    fn write_u8(&mut self, address: u16, value: u8) {
        if let Some(x) = self.buf.get_mut(address as usize) {
            *x = value;
        } else {
            panic!(
                "Index {address} into Buffer out of bounds, length is {}",
                self.buf.len()
            )
        }
    }
}

pub trait ReadBuffer {
    fn read_u8_mut(&mut self, address: u16) -> &mut u8;
    fn read_u8(&self, address: u16) -> u8;
}
pub trait WriteBuffer {
    fn write_u8(&mut self, address: u16, value: u8);
}

#[cfg(test)]
mod memory_tests {
    use super::Memory;

    #[test]
    fn test_get_u8() {
        let mut test_memory = Memory::new();

        test_memory.rom_0.buf[0x0000] = 69;
        test_memory.rom_n.buf[0x0000] = 42;

        assert_eq!(test_memory.read_u8(0x0000), 69);
        assert_eq!(test_memory.read_u8(0x4000), 42);

        // TODO: coverage of all memory blocks, including edge cases
    }

    #[test]
    fn test_get_u16() {
        let mut test_memory = Memory::new();
        test_memory.rom_0.buf[0] = 0xFA;
        test_memory.rom_0.buf[1] = 0xAF;

        assert_eq!(test_memory.read_u16(0), 0xAFFA)
        // TODO: coverage of all memory blocks, including edge cases
    }

    #[test]
    fn test_write_u8() {
        let mut test_memory = Memory::new();
        test_memory.write_u8(0, 69);
        assert_eq!(test_memory.rom_0.buf[0], 69);
    }

    #[test]
    fn test_write_u16() {
        let mut test_memory = Memory::new();
        test_memory.write_u16(0, 0xAFFA);
        assert_eq!(test_memory.rom_0.buf[0], 0xFA);
        assert_eq!(test_memory.rom_0.buf[1], 0xAF);
    }
}
