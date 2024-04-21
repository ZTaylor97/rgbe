extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub struct SDLContext {
    context: Sdl,
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
}

pub enum UpdateEvent {
    Continue,
    Stop,
}

impl SDLContext {
    pub fn new() -> Self {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();

        let window = video
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        let event_pump = context.event_pump().unwrap();
        SDLContext {
            context,
            canvas,
            event_pump,
        }
    }

    pub fn update(&mut self) -> UpdateEvent {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return UpdateEvent::Stop,
                _ => {}
            }
        }
        UpdateEvent::Continue
    }

    pub fn render(&mut self) {
        self.canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
