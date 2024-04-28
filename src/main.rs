mod context;
mod emulator;

use context::{SDLContext, UpdateEvent};
use emulator::{Emulator, EmulatorBuilder};

pub fn main() {
    let rom_path = std::env::var("TEST_ROM_DIR").unwrap();
    let mut emulator: Emulator = EmulatorBuilder::new()
        .cart(format!("{rom_path}/cpu_instrs/cpu_instrs.gb"))
        .build();

    let mut context = SDLContext::new();

    'running: loop {
        match context.update() {
            UpdateEvent::Stop => break 'running,
            _ => {}
        }

        context.render()
    }
}
