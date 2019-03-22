use sdl2;
use sdl2::render::Canvas;
use sdl2::pixels::Color;

const WINDOW_NAME: &str = "CHIP8";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

const RED: u8 = 0;
const BLUE: u8 = 0;
const GREEN: u8 = 0;

pub struct Display { 
    pub canvas: Canvas<sdl2::video::Window>,
    pub sdl_context: sdl2::Sdl
}

impl Display { 

    pub fn new() -> Display { 
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(WINDOW_NAME, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(RED, GREEN, BLUE));
        canvas.clear();
        canvas.present();
        return Display { 
            canvas: canvas,
            sdl_context: sdl_context
        }
    }
    /*
    pub fn start_display() { 
        let mut event_pump = self.sdl.event_pump().unwrap();
        loop { for _event in event_pump.poll_iter() {
                // match events here.. TODO: how do we clear the display?
                // how do we update the display....
                match event {
                    sdl2::event::Event::Quit {..} => break 'main,
                    _ => {},
                }
            }
            let video_subsystem = sdl.video().unwrap();
            video_subsystem.window("Chip8", 900, 700).build().unwrap();
        }
    }
    */

    /**
     * Overwrites the sprite onto the current size.
     * Returns true if the overwrite was successful.
     * Returns false if the write was successful, but, was not an overwrite.
     */
    pub fn overwrite_sprite(&self, sprite_bytes: &Vec<u8>, x: &u8, y: &u8) -> bool { 
        println!("TODO {} {}", x, y);
        return true;
    }
}

