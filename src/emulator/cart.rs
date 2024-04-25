use std::{fs::File, io::Read};

#[derive(Default)]
pub struct Cart {
    buf: Vec<u8>,
}

pub struct RomBank {
    buf: [u8; 0x4000],
}

impl Default for RomBank {
    fn default() -> Self {
        Self { buf: [0; 0x4000] }
    }
}

impl Cart {
    pub fn load_rom(rom_path: String) -> Self {
        let mut f = File::open(&rom_path).expect("File not found");
        let mut buf: Vec<u8> = vec![];
        let size = f.read_to_end(&mut buf).expect("Error reading file");

        println!("{size} == {}", buf.len());
        Cart { buf }
    }

    pub fn get_bank(&self, start: u16) -> RomBank {
        RomBank {
            buf: (self.buf[(start as usize)..(start as usize + 0x4000)])
                .try_into()
                .unwrap(),
        }
    }
}

#[cfg(test)]
mod cart_tests {
    use super::{Cart, RomBank};

    #[test]
    fn test_size_get_bank() {
        let test_cart = Cart {
            buf: vec![0; u16::MAX as usize],
        };

        let bank: RomBank = test_cart.get_bank(0x100);

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

        let bank: RomBank = test_cart.get_bank(start_idx as u16);

        assert_eq!(bank.buf[1], 0xAA);
        assert_eq!(bank.buf[2], 0xBB);
        assert_eq!(bank.buf[0x3FFF], 0xCC);
    }
}
