use sdl2;
// use sdl2::event::Event;

pub struct Input {
    pub event_loop: sdl2::EventPump,
}

impl Input {
    pub fn new(sdl_context: &sdl2::Sdl) -> Input {
        return Input {
            event_loop: match sdl_context.event_pump() {
                Ok(event_loop) => event_loop,
                Err(err) => panic!("{}", err),
            },
        };
    }
}
