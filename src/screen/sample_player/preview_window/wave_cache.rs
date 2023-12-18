use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use audio_engine::{SamplePack, TrackerSample};
use parking_lot::RwLock;

use crate::widget::waveform_view::WaveData;

#[derive(Debug, Default)]
pub struct WaveCache {
    pub cache: HashMap<usize, WaveData>,
}


impl WaveCache {
    pub fn generate(&mut self, index: usize, sample: &TrackerSample) {
        let peaks = sample.buf.peaks(Duration::from_millis(5));
        self.cache.insert(index, WaveData::from(peaks));
    }
}