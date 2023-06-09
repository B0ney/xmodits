use super::core::AudioOutputDevice;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use xmodits_lib::dsp::sample::FramesIter;
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
