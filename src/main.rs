extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env;
use std::fs::File;
use std::io::Read;
use std::{thread, time};

mod cpu;
mod display;
mod keyboard;

const SLEEP_TIMEOUT: std::time::Duration = time::Duration::from_millis(10);

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 1 {
        panic!()
    }
    let file_name = &args[1];
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(..) => panic!("file didnt exist"),
    };
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let mut cpu = cpu::CPU::new();
    cpu.load_memory(&buffer);
    let mut display = display::Display::new();
    let sdl_context = &display.sdl_context;
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main_loop: loop {
        display.canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }
        let result = cpu.cycle();
        if result.display_changed {
            display.draw(result.display_memory);
        }

        thread::sleep(SLEEP_TIMEOUT);
    }
}
