#![allow(unused)]

#[derive(Default)]

pub struct Memory {
    buf: Buffer<0xFFFF>,
}

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_u8_mut(&mut self, address: u16) -> &mut u8 {
        self.buf.read_u8_mut(address)
    }
    pub fn read_u8(&self, address: u16) -> u8 {
        self.buf.read_u8(address)
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        (self.read_u8(address) as u16) | ((self.read_u8(address + 1) as u16) << 8)
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        self.buf.write_u8(address, value)
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        self.write_u8(address, (value & 0x00ff) as u8);
        self.write_u8(address + 1, ((value & 0xff00) >> 8) as u8);
    }

    pub fn read_u16wrapper(&mut self, address: u16) -> U16Wrapper {
        self.buf.read_u16wrapper(address)
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

    fn read_u16wrapper(&mut self, address: u16) -> U16Wrapper {
        let (left, right) = self.buf.split_at_mut(address as usize + 1);

        U16Wrapper(left.last_mut().unwrap(), right.first_mut().unwrap())
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
    fn read_u16wrapper(&mut self, address: u16) -> U16Wrapper;
    fn read_u8(&self, address: u16) -> u8;
}
pub trait WriteBuffer {
    fn write_u8(&mut self, address: u16, value: u8);
}

#[derive(Debug)]
pub struct U16Wrapper<'a>(pub &'a mut u8, pub &'a mut u8);

impl<'a> U16Wrapper<'a> {
    pub fn from_u16(self, value: u16) {
        *self.0 = ((value & 0xFF00) >> 8) as u8;
        *self.1 = (value & 0xFF) as u8;
    }

    pub fn into_u16(&self) -> u16 {
        (*self.0 as u16) << 8 | *self.1 as u16
    }
}

#[cfg(test)]
mod memory_tests {
    use crate::emulator::memory::{ReadBuffer, U16Wrapper, WriteBuffer};

    use super::Memory;

    #[test]
    fn test_get_u8() {
        let mut test_memory = Memory::new();

        test_memory.buf.buf[0x0000] = 69;

        assert_eq!(test_memory.read_u8(0x0000), 69);
    }

    #[test]
    fn test_get_u16() {
        let mut test_memory = Memory::new();
        test_memory.buf.buf[0] = 0xFA;
        test_memory.buf.buf[1] = 0xAF;

        assert_eq!(test_memory.read_u16(0), 0xAFFA)
    }

    #[test]
    fn test_write_u8() {
        let mut test_memory = Memory::new();
        test_memory.write_u8(0, 69);
        assert_eq!(test_memory.buf.buf[0], 69);
    }

    #[test]
    fn test_write_u16() {
        let mut test_memory = Memory::new();
        test_memory.write_u16(0, 0xAFFA);
        assert_eq!(test_memory.buf.buf[0], 0xFA);
        assert_eq!(test_memory.buf.buf[1], 0xAF);
    }

    #[test]
    fn test_read_u16wrapper() {
        let mut test_memory = Memory::new();
        test_memory.write_u8(10, 0xF0);
        test_memory.write_u8(11, 0x0F);

        let U16Wrapper(val1, val2) = test_memory.read_u16wrapper(10);

        // sanity check values returned from function
        assert_eq!(*val1, 0xF0);
        assert_eq!(*val2, 0x0F);

        // test interior mutability
        *val1 = 69;
        *val2 = 99;
        assert_eq!(test_memory.read_u8(10), 69);
        assert_eq!(test_memory.read_u8(11), 99);
    }
}
