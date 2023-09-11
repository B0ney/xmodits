use std::sync::Arc;

use self::buffer::SampleBuffer;

pub mod buffer;
pub mod loop_point;

pub struct TrackerSample {
    pub buf: Arc<SampleBuffer>,
    pub is_reversed: bool,
    pub is_looping: bool,
}

impl TrackerSample {
    pub fn new(sample: impl Into<Arc<SampleBuffer>>) -> Self {
        todo!()
    }
}
