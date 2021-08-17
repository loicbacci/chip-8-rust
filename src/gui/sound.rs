//! Contains code used to produce sound

use sdl2;
use sdl2::audio::{AudioDevice, AudioCallback, AudioSpecDesired};

/// Struct containing the audio device
pub struct Sound {
    /// The audio device
    device: AudioDevice<SquareWave>,
}

impl Sound {
    /// Creates a new Sound instance
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let audio = sdl_context.audio().unwrap();

        let des_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let device = audio
            .open_playback(None, &des_spec, |spec| {
                SquareWave {
                    phase_inc: 240.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                }
            })
            .unwrap();

        Sound { device }
    }

    /// Starts the beep
    pub fn start_beep(&self) {
        self.device.resume();
    }

    /// Ends the beep
    pub fn stop_beep(&self) {
        self.device.pause();
    }
}

/// AudioCallback instance for a square wave
struct SquareWave {
    /// Phase_inc of the wave
    phase_inc: f32,
    /// Phase of the wave
    phase: f32,
    /// Volume of the wave
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = self.volume * if self.phase < 0.5 { 1.0 } else { -1.0 };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}