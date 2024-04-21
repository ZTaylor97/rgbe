mod context;
mod emulator;

use context::{SDLContext, UpdateEvent};
use emulator::Emulator;

pub fn main() {
    let rom_path = std::env::var("TEST_ROM_DIR").unwrap();
    println!("{}", rom_path);
    let mut emulator = Emulator::new();
    let mut context = SDLContext::new();

    'running: loop {
        match context.update() {
            UpdateEvent::Stop => break 'running,
            _ => {}
        }

        context.render()
    }
}
