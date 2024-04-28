mod cart;
mod memory;

use std::{fs::File, io::Read};

use cart::Cart;
use memory::Memory;

#[derive(Default)]
pub struct Emulator {
    memory: Memory,
    cart: Cart,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cart: Cart::default(),
            memory: Memory::default(),
        }
    }
}

#[derive(Default)]
pub struct EmulatorBuilder {
    cart: Cart,
    memory: Memory,
}

impl EmulatorBuilder {
    pub fn new() -> EmulatorBuilder {
        EmulatorBuilder {
            ..Default::default()
        }
    }

    pub fn cart(mut self, file_path: String) -> EmulatorBuilder {
        self.cart = Cart::load_rom(file_path);
        self
    }

    pub fn build(self) -> Emulator {
        Emulator {
            memory: self.memory,
            cart: self.cart,
        }
    }
}
