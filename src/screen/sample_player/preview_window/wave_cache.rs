use std::{collections::HashMap, sync::Arc};
use std::time::Duration;

use audio_engine::{SamplePack, TrackerSample};

use crate::widget::waveform_view::WaveData;

#[derive(Debug, Default)]
pub struct WaveCache {
    pub cache: HashMap<usize, Arc<WaveData>>,
}

impl WaveCache {
    pub fn from_sample_pack(sample_pack: &SamplePack) -> Self {
        let mut wave_cache = Self::default();

        for (idx, result) in sample_pack.samples.iter().enumerate() {
            if let Ok((_, sample)) = result {
                wave_cache.generate(idx, sample)
            }
        }

        wave_cache
    }

    pub fn generate(&mut self, index: usize, sample: &TrackerSample) {
        let peaks = sample.buf.peaks(Duration::from_millis(5));
        self.cache.insert(index, Arc::new(WaveData::from(peaks)));
    }

    pub fn get(&self, index: usize) -> Option<&WaveData> {
        Some(self.cache.get(&index)?.as_ref())

    }
}