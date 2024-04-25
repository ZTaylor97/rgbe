mod context;
mod emulator;

use context::{SDLContext, UpdateEvent};
use emulator::Emulator;

pub fn main() {
    let mut emulator = Emulator::new();
    let rom_path = std::env::var("TEST_ROM_DIR").unwrap();

    emulator.load_rom(format!("{rom_path}/cpu_instrs/cpu_instrs.gb"));

    let mut context = SDLContext::new();

    'running: loop {
        match context.update() {
            UpdateEvent::Stop => break 'running,
            _ => {}
        }

        context.render()
    }
}
