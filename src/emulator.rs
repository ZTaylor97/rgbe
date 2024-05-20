mod cart;
mod cpu;
mod instructions;
mod memory;

use cart::Cart;
use cpu::CPU;
use memory::Memory;

use instructions::Instruction;

#[derive(Default)]
pub struct Emulator {
    memory: Memory,
    cart: Cart,
}

impl Emulator {
    pub fn update(&mut self) {}
}

#[derive(Default)]
pub struct EmulatorBuilder {
    cart: Cart,
    memory: Memory,
    cpu: CPU,
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
