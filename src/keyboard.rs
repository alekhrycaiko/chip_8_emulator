use crate::input;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Keyboard {
    pub keys: [bool; 16],
    pub key_clicked: u8,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        return Keyboard {
            keys: [false; 16],
            key_clicked: 17,
        };
    }

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

    pub fn cycle(&mut self, sdl_context: &sdl2::Sdl) {
        let mut event_loop = input::Input::new(sdl_context).event_loop;
        for event in event_loop.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Num0),
                    ..
                } => {
                    self.keys[0] = true;
                    self.key_clicked = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num0),
                    ..
                } => {
                    self.keys[0] = false;
                    self.key_clicked = 16;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => {
                    self.keys[1] = true;
                    self.key_clicked = 1;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    ..
                } => {
                    self.keys[1] = false;
                    self.key_clicked = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => {
                    self.keys[2] = true;
                    self.key_clicked = 2;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num2),
                    ..
                } => {
                    self.keys[2] = false;
                    self.key_clicked = 2;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => {
                    self.keys[3] = true;
                    self.key_clicked = 3;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num3),
                    ..
                } => {
                    self.keys[3] = false;
                    self.key_clicked = 3;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => {
                    self.keys[4] = true;
                    self.key_clicked = 4;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num4),
                    ..
                } => {
                    self.keys[4] = false;
                    self.key_clicked = 4;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num5),
                    ..
                } => {
                    self.keys[5] = true;
                    self.key_clicked = 5;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num5),
                    ..
                } => {
                    self.keys[5] = false;
                    self.key_clicked = 5;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num6),
                    ..
                } => {
                    self.keys[6] = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num6),
                    ..
                } => {
                    self.keys[6] = false;
                    self.key_clicked = 6;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Num7),
                    ..
                } => {
                    self.keys[7] = true;
                    self.key_clicked = 7;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num7),
                    ..
                } => {
                    self.keys[7] = false;
                    self.key_clicked = 8;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num8),
                    ..
                } => {
                    self.keys[8] = true;
                    self.key_clicked = 8;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num8),
                    ..
                } => {
                    self.keys[8] = false;
                    self.key_clicked = 8;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num9),
                    ..
                } => {
                    self.keys[9] = true;
                    self.key_clicked = 9;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num9),
                    ..
                } => {
                    self.keys[9] = false;
                    self.key_clicked = 9;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    self.keys[10] = true;
                    self.key_clicked = 10;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    self.keys[10] = false;
                    self.key_clicked = 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => {
                    self.keys[11] = true;
                    self.key_clicked = 11;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::B),
                    ..
                } => {
                    self.keys[11] = false;
                    self.key_clicked = 11;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    self.keys[12] = true;
                    self.key_clicked = 12;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    self.keys[12] = true;
                    self.key_clicked = 12;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    self.keys[13] = true;
                    self.key_clicked = 13;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    self.keys[13] = false;
                    self.key_clicked = 13;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    self.keys[14] = true;
                    self.key_clicked = 14;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    self.keys[14] = false;
                    self.key_clicked = 14;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    self.keys[15] = true;
                    self.key_clicked = 15;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    self.keys[15] = false;
                }
                Event::Quit { .. } => panic!("Exit"),
                _ => continue,
            }
        }
    }
}
