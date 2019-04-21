extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env;
use std::fs::File;
use std::io::Read;
use std::{thread, time};
mod cpu;
mod display;
mod input;
mod keyboard;

const SLEEP_TIMEOUT: std::time::Duration = time::Duration::from_millis(1);

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
    let sdl_context = sdl2::init().unwrap();
    let mut display = display::Display::new(&sdl_context);
    'main_loop: loop {
        //        display.canvas.clear();
        let result = cpu.cycle(&sdl_context);
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
