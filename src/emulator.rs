mod cart;

use std::{fs::File, io::Read};

use cart::Cart;

pub struct Emulator {
    cart: Cart,
}

impl Emulator {
    pub fn new() -> Self {
        Self { cart: Cart {} }
    }
    pub fn load_rom(&mut self, rom_path: String) {
        let mut f = File::open(&rom_path).expect("File not found");
        let mut buf: Vec<u8> = vec![];
        let size = f.read_to_end(&mut buf).expect("Error reading file");

        println!("size: {size}\n{buf:?}");
    }
}
