use super::core::{AudioOutputDevice, PlayHandle};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use xmodits_lib::dsp::sample::{FramesIter, SampleFrame};
use xmodits_lib::dsp::{resampler::resample, RawSample, SampleBuffer};
use xmodits_lib::interface::sample::{self, LoopType};

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
    pub buffer: Arc<SampleBuffer>,
    frame: usize,
    reversed: bool,
    looping: bool,
}

impl TrackerSample {
    pub fn new(sample: impl Into<Arc<SampleBuffer>>) -> Self {
        Self {
            buffer: sample.into(),
            frame: 0,
            reversed: false,
            looping: false,
        }
    }

    pub fn end_frame(&self) -> usize {
        match self.buffer.loop_data.is_disabled() {
            true => self.buffer.duration() - 1,
            false => self.buffer.end(),
        }
    }

    pub fn start_frame(&self) -> usize {
        match self.buffer.loop_data.is_disabled() {
            true => 0,
            false => self.buffer.start(),
        }
    }

    pub fn hit_barrier(&self) -> bool {
        if self.frame >= self.buffer.duration() || self.frame >= self.end_frame() {
            return true;
        }

        match self.looping {
            false => self.frame == 0 && self.reversed,
            true => self.frame <= self.start_frame(),
        }
    }

    pub fn reverse(&mut self) {
        self.reversed = !self.reversed;

        self.frame = match self.buffer.loop_type() {
            LoopType::Forward => self.start_frame(),
            LoopType::Backward | LoopType::PingPong => match self.reversed {
                true => self.end_frame() - 1,
                false => self.start_frame(),
            },
            _ => unreachable!(),
        }
    }

    pub fn rate(&self) -> usize {
        self.buffer.rate as usize
    }

    pub fn rate_original(&self) -> usize {
        self.buffer.rate_original() as usize
    }
}

impl PlayHandle for TrackerSample {
    fn next(&mut self) -> Option<[f32; 2]> {
        if self.hit_barrier() {
            match self.buffer.loop_data.is_disabled() {
                true => return None,
                false => {
                    self.looping = true;
                    self.reverse();
                }
            }
        }

        let result = self.buffer.frame(self.frame).map(|f| f.get_stereo_frame());

        match self.reversed {
            true => self.frame -= 1,
            false => self.frame += 1,
        };

        result
    }

    fn reset(&mut self) {
        self.frame = 0;
        self.looping = false;
    }

    fn jump(&mut self, tick: usize) {
        self.frame = tick
    }
}
