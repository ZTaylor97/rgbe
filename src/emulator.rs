#![allow(unused)]
mod cart;
mod cpu;
mod instructions;
mod memory;

use cart::Cart;
use cpu::CPU;
use memory::Memory;

#[derive(Default)]
pub struct Emulator {
    memory: Memory,
    cart: Cart,
    cpu: CPU,
}

impl Emulator {
    pub fn update(&mut self) {
        self.cpu.execute(&mut self.memory);
    }
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
        self.memory.swap_rom(self.cart.get_bank(0));
        self
    }

    pub fn build(self) -> Emulator {
        let mut cpu = CPU::default();
        cpu.load_instructions();

        Emulator {
            memory: self.memory,
            cart: self.cart,
            cpu,
        }
    }
}
