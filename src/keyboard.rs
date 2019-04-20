use sdl2;
//use sdl2::event::Event;
pub struct Keyboard<'a> {
    keys: [bool; 16],
    sdl2_context: &'a sdl2::Sdl,
}

impl<'a> Keyboard<'a> {
    pub fn new(sdl_context: &'a sdl2::Sdl) -> Keyboard {
        return Keyboard {
            keys: [false; 16],
            sdl2_context: sdl_context,
        };
    }

    /**
     * Blocking call that waits for a keyboard input before returning.
     */
    pub fn block_for_input(&self) -> u8 {
        'blocking_loop: loop {
            //for event in self.sdl_context.event_pump.poll_iter() {
            //   Event::KeyDown { keycode: {} }
            //}
        }
    }

    pub fn poll(&mut self) {}

    /**
     * Returns true if the provided key is pressed.
     */
    pub fn is_key_pressed(&mut self, key: u8) -> bool {
        return match key {
            0x0 => self.keys[0],
            0x1 => self.keys[1],
            0x2 => self.keys[2],
            0x3 => self.keys[3],
            0x4 => self.keys[4],
            0x5 => self.keys[5],
            0x6 => self.keys[6],
            0x7 => self.keys[7],
            0x8 => self.keys[8],
            0x9 => self.keys[9],
            0xa => self.keys[10],
            0xb => self.keys[11],
            0xc => self.keys[12],
            0xd => self.keys[13],
            0xe => self.keys[14],
            0xf => self.keys[15],
            _ => false,
        };
    }
}
