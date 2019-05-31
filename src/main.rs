extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env;
use std::{thread, time};
mod audio;
mod cpu;
mod display;
mod font;
mod input;
mod keyboard;
mod rom;

const SLEEP_TIMEOUT: std::time::Duration = time::Duration::from_millis(1);

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.is_empty() {
        panic!()
    }
    let file_name = &args[1];
    let rom = rom::Rom::new(file_name.to_string());
    let mut cpu = cpu::CPU::new(&rom.data);
    let sdl_context = match sdl2::init() {
        Ok(sdl2) => sdl2,
        Err(err) => panic!("{}", err),
    };
    let mut display = display::Display::new(&sdl_context);
    let audio_driver = audio::Audio::new(&sdl_context);
    'main_loop: loop {
        let result = cpu.cycle(&sdl_context, &audio_driver);
        let mut event_loop = input::Input::new(&sdl_context).event_loop;
        for event in event_loop.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }
        if result.display_changed {
            display.draw(result.display_memory);
        }
        thread::sleep(SLEEP_TIMEOUT);
    }
}
