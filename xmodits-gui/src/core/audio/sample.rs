use super::core::{AudioOutputDevice, PlayHandle};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use xmodits_lib::dsp::sample::{FramesIter, SampleFrame};
use xmodits_lib::dsp::{resampler::resample, RawSample, SampleBuffer};
use xmodits_lib::interface::sample::{self, LoopType};

#[derive(Clone)]
pub struct TrackerSample {
    pub buffer: Arc<SampleBuffer>,
    frame: usize,
    is_reversed: bool,
    is_looping: bool,
    loop_force_disabled: bool,
}

impl TrackerSample {
    pub fn new(sample: impl Into<Arc<SampleBuffer>>) -> Self {
        Self {
            buffer: sample.into(),
            frame: 0,
            is_reversed: false,
            is_looping: false,
            loop_force_disabled: true,
        }
    }

    pub fn inner_sample(&self) -> &SampleBuffer {
        &self.buffer
    }

    pub fn loop_enabled(&self) -> bool {
        // override loop data
        match self.loop_force_disabled {
            true => false,
            false => !self.buffer.loop_data.is_disabled(),
        }
    }

    pub fn end_frame(&self) -> usize {
        match self.loop_enabled() {
            true => self.buffer.end(),
            false => self.buffer.duration(),
        }
    }

    pub fn start_frame(&self) -> usize {
        match self.loop_enabled() {
            true => 0,
            false => self.buffer.start(),
        }
    }

    pub fn hit_barrier(&self) -> bool {
        // If the frame has reached the end of the sample
        // or if the frame reached an end point.
        if self.frame >= self.buffer.duration() || self.frame >= self.end_frame() {
            return true;
        }

        // If the sample is currently playing backwards, i.e. is looping,
        // Check if the frame is equal or below the start frame.
        //
        // Otherwise check if the frame is zero
        match self.is_looping {
            false => self.frame == 0 && self.is_reversed,
            true => self.frame <= self.start_frame(),
        }
    }

    fn jump_to_loop_point(&mut self) {
        self.frame = match self.buffer.loop_type() {
            LoopType::Forward => self.start_frame(),
            LoopType::Backward | LoopType::PingPong => {
                self.is_reversed = !self.is_reversed;

                match self.is_reversed {
                    true => self.end_frame() - 1,
                    false => self.start_frame(),
                }
            }
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
            match self.loop_enabled() {
                true => {
                    self.is_looping = true;
                    self.jump_to_loop_point();
                }

                false => return None,
            }
        }

        let result = self
            .buffer
            .frame(self.frame)
            .map(SampleFrame::get_stereo_frame);

        match self.is_reversed {
            true => self.frame -= 1,
            false => self.frame += 1,
        };

        result
    }

    fn reset(&mut self) {
        self.frame = 0;
        self.is_looping = false;
    }

    fn jump(&mut self, tick: usize) {
        self.frame = tick
    }
}
