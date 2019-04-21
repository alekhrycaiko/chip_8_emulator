use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

const WINDOW_NAME: &str = "CHIP8";

const SCALE: u32 = 10;
const WINDOW_WIDTH: u32 = 64 * SCALE;
const WINDOW_HEIGHT: u32 = 32 * SCALE;

const RED: u8 = 0;
const BLUE: u8 = 0;
const GREEN: u8 = 0;

pub struct Display {
    pub canvas: Canvas<sdl2::video::Window>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Display {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(WINDOW_NAME, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(RED, GREEN, BLUE));
        canvas.clear();
        canvas.present();
        return Display { canvas: canvas };
    }
    /**
     * Given a set of pixels, draw them on the canvas.
     */
    pub fn draw(&mut self, pixels: &[[u8; 64]; 32]) {
        for (x, row) in pixels.iter().enumerate() {
            for (y, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE;
                let y = (y as u32) * SCALE;
                if col == 1 {
                    let white = (255, 255, 255);
                    self.canvas.set_draw_color(white);
                } else {
                    let black = (0, 0, 0);
                    self.canvas.set_draw_color(black);
                }
                let _ = self
                    .canvas
                    .fill_rect(Rect::new(y as i32, x as i32, SCALE, SCALE));
            }
        }
        self.canvas.present();
    }
}
