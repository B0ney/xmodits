use xmodits_lib::{
    dsp::{RawSample, SampleBuffer},
    interface::{Error, Sample as Metadata},
    traits::Module,
};

use super::{sample::TrackerSample, sample_cache::Cache};

/// A Pack of processed samples from a tracker
///
pub struct SamplePack {
    // Cache of raw samples, can be used for drawing waveforms, etc
    sample_cache: Cache<Metadata, SampleBuffer>,
    samples: Vec<Result<TrackerSample, Error>>,
}

impl SamplePack {
    pub fn from_module(module: Box<dyn Module>) -> Self {
        let sample_cache = Cache::new();

        let samples = module
            .samples()
            .iter()
            .cloned()
            .map(|sample| {
                module.pcm(&sample).map(|pcm| {
                    let raw_sample = RawSample::new(&sample, pcm);
                    let mut sample_buffer = SampleBuffer::from(raw_sample.into());
                    xmodits_lib::dsp::resampler::resample(&mut sample_buffer, 48000); // todo
                    let cached_sample = sample_cache.add(sample.clone(), sample_buffer);

                    TrackerSample::new(cached_sample)
                })
            })
            .collect();

        Self {
            sample_cache,
            samples,
        }
    }
    pub fn play() {}
}
