use crate::display::Display;

const CPU_MEMORY: usize = 4096;
pub struct CPU { 
    reg_v: [u8; 16],
    flag: u8,
    reg_i: u8, // special register; right most 12 bits are used.
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    sp: usize,
    stack: [u16; 16],
    memory: [u8; CPU_MEMORY],
    display: Display
}

impl CPU { 
    // needs to create a new CPU and initialize the memory perhaps.
    pub fn new() -> CPU { 
        let mut memory = [0x00; CPU_MEMORY];
        let mut stack = [0x000; 16];
        return CPU {
            reg_v: [0; 16],
            flag: 0, 
            reg_i: 0, 
            delay_timer: 0, 
            sound_timer: 0, 
            pc: 0x000,
            sp: 0,
            stack: stack,
            memory: memory,
            display: Display::new()
        };
    }
    pub fn handle_opcode(&self, opcode: u16) -> u16 { 
        let tuple_opcode = (
            (opcode & 0xf000) >> 12 as u8,
            (opcode & 0x0f00) >> 8 as u8,
            (opcode & 0x00f0) >> 4 as u8,
            (opcode & 0x000f) as u8
        );
        return match tuple_opcode {
            (0x0, 0x0, 0xe, 0x0) => self.handle_cls(),
            (0x0, 0x0, 0xe, 0xe) => self.handle_ret(),
            (0x0, _, _, _) => self.handle_0nnn(opcode),
            (0x1, _, _, _) => self.handle_1nnn(opcode), 
            (0x2, _, _, _) => self.handle_2nnn(opcode),
            (0x3, _, _, _) => self.handle_3xkk(opcode),
            (0x4, _, _, _) => self.handle_4xkk(opcode),
            (0x5, _, _, _) => self.handle_5xy0(opcode),
            (0x6, _, _, _) => self.handle_6xkk(opcode),
            (0x7, _, _, _) => self.handle_7xkk(opcode),
            (0x8, _, _, 0x0) => self.handle_8xy0(opcode),
            (0x8, _, _, 0x1) => self.handle_8xy1(opcode),
            (0x8, _, _, 0x2) => self.handle_8xy2(opcode),
            (0x8, _, _, 0x3) => self.handle_8xy3(opcode),
            (0x8, _, _, 0x4) =>	self.handle_8xy4(opcode),
            (0x8, _, _, 0x5) => self.handle_8xy5(opcode),
            (0x8, _, _, 0x6) => self.handle_8xy6(opcode),
            (0x8, _, _, 0x7) => self.handle_8xy7(opcode),
            (0x8, _, _, 0xe) => self.handle_8xye(opcode),
            (0x9, _, _, 0x0) => self.handle_9xy0(opcode),
            (0xa, _, _, _) =>	self.handle_annn(opcode),
            (0xb, _, _, _) => self.handle_bnnn(opcode),
            (0xc, _, _, _) => self.handle_cxkk(opcode),
            (0xd, _, _, _) => self.handle_dxyn(opcode),
            (0xe, _, 0x9, 0xe) => self.handle_ex9e(opcode),
            (0xe, _, 0xa, 0x1) => self.handle_exa1(opcode),
            (0xf, _, 0x0, 0x7) => self.handle_fx07(opcode), 
            (0xf, _, 0x0, 0xa) => self.handle_fx0a(opcode),
            (0xf, _, 0x1, 0x5) => self.handle_fx15(opcode),
            (0xf, _, 0x1, 0x8) => self.handle_fx18(opcode),
            (0xf, _, 0x1, 0xe) => self.handle_fx1e(opcode),
            (0xf, _, 0x2, 0x9) => self.handle_fx29(opcode),
            (0xf, _, 0x3, 0x3) => self.handle_fx33(opcode),
            (0xf, _, 0x5, 0x5) => self.handle_fx55(opcode),
            (0xf, _, 0x6, 0x5) => self.handle_fx65(opcode),
            (_, _, _, _ ) => 2,
        };
    }
    fn handle_1nnn(&self, opcode: u16) -> u16 { 
        self.pc = opcode & 0x0fff >> 4;
        return self.pc;
    }
    
    fn handle_0nnn(&self, opcode: u16) -> u16 { 
        self.pc = opcode & 0x0fff >> 4;   
        return self.pc;
    }
    fn handle_cls(&self) -> u16 { 
        self.display.clear();
        return 2 as u16;
    }

    fn handle_ret(&self) -> u16 { 
        self.pc = self.sp as u16;
        self.sp -= 1;
        return self.pc;
    }

    fn handle_fx65(&self, opcode: u16) -> u16 { 
        // TODO
        return 2;
    }   
    fn handle_fx55(&self, opcode: u16) -> u16 { 
        // TODO
        return 2;
    }
    fn handle_fx33(&self, opcode: u16) -> u16 { 
        // TODO
        return 2;
    }

    fn handle_fx1e(&self, opcode: u16) -> u16 { 
        // TODO
        return 2;
    }
    fn handle_fx29(&self, opcode: u16) -> u16 { 
        // TODO
        return 2;
    }

    fn handle_fx0a(&self, opcode: u16) -> u16 { 
        // TODO
        return 2;
    }

    fn handle_fx07(&self, opcode: u16) -> u16 { 
        // TODO
        return 2;
    }
    fn handle_fx18(&self, opcode: u16) -> u16 { 
        // TODO
        return 2;
    }
    fn handle_fx15(&self, opcode: u16) -> u16 { 
        // TODO
        return 2;
    }
    fn handle_exa1(&self, opcode: u16) -> u16 { 
        // TODO
        return 2;
    }
    fn handle_ex9e(&self, opcode: u16) -> u16 { 
        // TODO
        return 2;
    }

    fn handle_dxyn(&self, opcode: u16) -> u16{ 
        // TODO
        return 2;
    }

    fn handle_cxkk(&self, opcode: u16) -> u16{ 
        // TODO
        return 2;
    }

    fn handle_bnnn(&self, opcode: u16) -> u16{ 
        // TODO
        return 2;
    }


    fn handle_annn(&self, opcode: u16) -> u16{ 
        // TODO
        return 2;
    }


    fn handle_9xy0(&self, opcode: u16) -> u16{ 
        // TODO
        return 2;
    }

    fn handle_8xye(&self, opcode: u16) -> u16{ 
        // TODO
        return 2;
    }
    fn handle_8xy7(&self, opcode: u16) -> u16{ 
        // TODO
        return 2;
    }

    fn handle_8xy6(&self, opcode: u16) -> u16{ 
        // TODO
        return 2;
    }
    fn handle_8xy5(&self, opcode: u16) -> u16{ 
        // TODO
        return 2;
    }



    fn handle_8xy4(&self, opcode: u16) -> u16{ 
        // TODO
        return 2;
    }
    fn handle_8xy3(&self, opcode: u16) -> u16{ 
        // TODO
        return 2;
    }


    fn handle_8xy2(&self, opcode: u16) -> u16{ 
        // TODO
        return 2;
    }
    /**
      Set Vx = Vx OR Vy.
      Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
      */
    fn handle_8xy1(&self, opcode: u16) -> u16 { 
        // TODO
        return 2;
    }
    /**
     * Set Vx = Vy.
     * Stores the value of register Vy in register Vx.
     */
    fn handle_8xy0(&self, opcode: u16) -> u16 { 
        // TODO
        return 2;
    }
    /**
     * Set Vx = Vx + kk.
     * Adds the value kk to the value of register Vx, then stores the result in
     */
    fn handle_7xkk(&self, opcode: u16) -> u16 { 
        // TODO
        return 2;
    }
    /**
     * Set Vx = kk.
     * The interpreter puts the value kk into register Vx.
     */
    fn handle_6xkk(&self, opcode: u16) -> u16 { 
        return 2;
        //TODO;
    }
    /**
     * Skip next instruction if Vx = Vy.
     * The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
     **/
    fn handle_5xy0(&self, opcode: u16) -> u16 { 
        // TODO;
        return 2;
    }
    /**
     * Skip next instruction if Vx != kk.
     The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
     */
    fn handle_4xkk(&self, opcode: u16) -> u16 { 
        // TODO
        return 2; 
    }
    /*
     * Skip next instruction if Vx = kk.
     * The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
     */
    fn handle_3xkk(&self, opcode: u16) -> u16 { 
        let kk = (opcode & 0x00ff >> 8) as u8;
        let x = opcode & 0x0f00 >> 12;
        let vx = self.reg_v[x as usize] as u8;
        if vx == kk { 
            self.pc += 2;
            return 2;
        }
        return 0;
    }
    /**
     * Call subroutine at nnn.
     The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
     */
    fn handle_2nnn(&self, opcode: u16) -> u16 { 
        self.stack[self.sp] = self.pc + 2;
        self.sp += 1;
        self.pc = opcode; 
        return self.pc;
    }
}
