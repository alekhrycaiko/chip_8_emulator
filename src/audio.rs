use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use std::time::Duration;

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Audio {
    audio_device: AudioDevice<SquareWave>,
}

impl Audio {
    pub fn new(sdl_context: &sdl2::Sdl) -> Audio {
        let audio_system = match sdl_context.audio() {
            Ok(v) => v,
            Err(e) => panic!("Error, SDL2 for audio subsystem failed to init. E is {}", e),
        };
        let desired_spec = AudioSpecDesired {
            freq: None,
            channels: None, // mono
            samples: None,  // default sample size
        };

        let audio_device = match audio_system.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        }) {
            Ok(v) => v,
            Err(_) => panic!("Error couldn't construct a squarewave"),
        };
        Audio { audio_device }
    }
    pub fn play(&self) {
        self.audio_device.resume();
        std::thread::sleep(Duration::from_millis(10));
        self.audio_device.pause();
    }
}
