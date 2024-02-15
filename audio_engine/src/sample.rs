pub mod buffer;

use std::sync::Arc;
use std::time::{Duration, Instant};

pub use buffer::SampleBuffer;

#[derive(Debug, Clone)]
pub struct TrackerSample {
    pub buf: Arc<SampleBuffer>,
    pub is_reversed: bool,
    pub is_looping: bool,
    pub frame: usize,
}

impl TrackerSample {
    pub fn new(buf: impl Into<Arc<SampleBuffer>>) -> Self {
        Self {
            buf: buf.into(),
            is_reversed: false,
            is_looping: false,
            frame: 0,
        }
    }

    pub fn channels(&self) -> usize {
        self.buf.channels()
    }

    pub fn frame(&self) -> usize {
        self.frame / self.buf.channels()
    }
}

type Callback = Box<dyn Fn(&TrackerSample, &mut Instant) + Send>;

pub(crate) struct FramesIter {
    pub sample: TrackerSample,
    pub timer: Instant,
    pub callback: Option<Callback>,
}

impl Iterator for FramesIter {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let tracker_sample = &mut self.sample;
        let sample = tracker_sample.buf.get_sample(tracker_sample.frame);

        if let Some(callback) = &self.callback {
            callback(tracker_sample, &mut self.timer);
        }
        tracker_sample.frame += 1;
        sample
    }
}

impl rodio::Source for FramesIter {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.sample.buf.channels() as u16
    }

    fn sample_rate(&self) -> u32 {
        self.sample.buf.rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(self.sample.buf.duration())
    }
}
