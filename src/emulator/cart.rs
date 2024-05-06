use std::{fs::File, io::Read};

use super::memory::{Buffer, ReadBuffer};

// TODO: carts may need to keep track of swappable bank state
#[derive(Default)]
pub struct Cart {
    buf: Vec<u8>,
}

impl Cart {
    pub fn load_rom(rom_path: String) -> Self {
        let mut f = File::open(&rom_path).expect("File not found");
        let mut buf: Vec<u8> = vec![];
        let size = f.read_to_end(&mut buf).expect("Error reading file");

        Cart { buf }
    }

    pub fn get_bank(&self, start: u16) -> Buffer<0x4000> {
        Buffer {
            buf: (self.buf[(start as usize)..(start as usize + 0x4000)])
                .try_into()
                .unwrap(),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        if address as usize > self.buf.len() {
            return 0;
        }
        self.buf[address as usize]
    }
}

#[cfg(test)]
mod cart_tests {
    use super::Cart;

    #[test]
    fn test_size_get_bank() {
        let test_cart = Cart {
            buf: vec![0; u16::MAX as usize],
        };

        let bank = test_cart.get_bank(0x100);

        assert_eq!(bank.buf.len(), 0x4000)
    }

    #[test]
    fn test_content_get_bank() {
        let start_idx = 100;
        let mut test_cart = Cart {
            buf: vec![0; u16::MAX as usize],
        };

        test_cart.buf[start_idx + 1] = 0xAA;
        test_cart.buf[start_idx + 2] = 0xBB;
        test_cart.buf[start_idx + 0x3FFF] = 0xCC;

        let bank = test_cart.get_bank(start_idx as u16);

        assert_eq!(bank.buf[1], 0xAA);
        assert_eq!(bank.buf[2], 0xBB);
        assert_eq!(bank.buf[0x3FFF], 0xCC);
    }
}
