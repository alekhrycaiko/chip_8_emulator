use crate::cpu::CPU;

pub fn decompile(file_buffer: &[u8], cpu_instance: &CPU){
    let mut pc = 0;
    let length = file_buffer.len();

    while pc < length {
        let first_byte = file_buffer[pc];
        let second_byte = file_buffer[pc+1];
        let opcode = ((first_byte as u16) << 8) | second_byte as u16;
        // TODO: I suspect this is incorrect...
        pc += cpu_instance.handle_opcode(opcode) as usize;
    }
}
