use super::core::{AudioOutputDevice, PlayHandle};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use xmodits_lib::dsp::sample::{FramesIter, SampleFrame};
use xmodits_lib::dsp::{resampler::resample, RawSample, SampleBuffer};

pub struct SampleInfo {
    name: String,
    filename: Option<String>,
    buffer: Arc<SampleBuffer>,
}

pub struct AudioDevice {
    out: Box<dyn AudioOutputDevice>,
    cache: RwLock<HashMap<String, Weak<SampleBuffer>>>,
}

#[derive(Clone)]
pub struct TrackerSample {
    buffer: Arc<SampleBuffer>,
    frame: usize,
    reversed: bool,
}

impl TrackerSample {
    pub fn new(sample: impl Into<Arc<SampleBuffer>>) -> Self {
        Self {
            buffer: sample.into(),
            frame: 0,
            reversed: false,
        }
    }

    pub fn reverse(&mut self) {
        self.reversed = !self.reversed;

        if self.frame == 0 || self.frame >= self.buffer.duration() - 1 {
            self.frame = match self.reversed {
                true => self.buffer.duration() - 1,
                false => 0,
            }
        };
    }
}

impl PlayHandle for TrackerSample {
    fn next(&mut self) -> Option<[f32; 2]> {
        if self.frame >= self.buffer.duration() || self.frame == 0 && self.reversed {
            // self.reverse();
            return None;
        }

        let result = self.buffer.frame(self.frame).map(|f| f.get_stereo_frame());

        match self.reversed {
            true => self.frame -= 1,
            false => self.frame += 1,
        };

        result
    }

    fn reset(&mut self) {
        self.frame = 0
    }

    fn jump(&mut self, tick: usize) {
        self.frame = tick
    }
}
