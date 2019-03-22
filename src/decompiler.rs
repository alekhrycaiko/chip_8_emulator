use crate::cpu::CPU;

pub fn decompile(file_buffer: &[u8], cpu_instance: &mut CPU){
    let mut pc = 0;
    let length = file_buffer.len();
    loop { 
    cpu_instance.display.canvas.present();    
    while pc < length {
        let first_byte = file_buffer[pc];
        let second_byte = file_buffer[pc+1];
        let opcode = ((first_byte as u16) << 8) | second_byte as u16;
        pc += cpu_instance.handle_opcode(opcode) as usize;
    }
    }
}
