use std::io::{Read};
use std::fs::File;
use std::env;
mod decompiler;
mod cpu;
mod display;

/**
 * exercise:
 * read in the rom file.
 * determine the op code.
 * progress pointer to the next opcode.
 *
 * We will need to use rust's `match` characteristic in order to find the appropriate byte to
 * match. Then move it onto the next step.
 *
 */
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 1 {
        panic!()
    }
    let file_name = &args[1];
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(..)  => panic!("file didnt exist"),
    };
    let mut buffer = Vec::new();
    // read the whole file
    file.read_to_end(&mut buffer).unwrap();
    let cpu = cpu::CPU::new();
    decompiler::decompile(&buffer, &cpu);
}

