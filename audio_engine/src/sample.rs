use std::sync::Arc;
pub mod buffer;

use rodio::Source;

use self::buffer::SampleBuffer;

#[derive(Clone)]
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
}

impl Iterator for TrackerSample {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.buf.get_sample(self.frame);
        self.frame += 1;
        sample
    }
}

impl Source for TrackerSample {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.buf.channels() as u16
    }

    fn sample_rate(&self) -> u32 {
        self.buf.rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        Some(self.buf.duration())
    }
}
