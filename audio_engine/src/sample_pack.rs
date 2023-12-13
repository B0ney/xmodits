use xmodits_lib::dsp;
use xmodits_lib::{Sample, Module};

use crate::sample::TrackerSample;
use crate::sample::buffer::SampleBuffer;

pub struct SamplePack {
    pub samples: Vec<Result<(Sample, TrackerSample), xmodits_lib::Error>>,
}

impl SamplePack {
    pub fn build(module: &dyn Module) -> Self {
        let samples: Vec<Result<(Sample, TrackerSample), xmodits_lib::Error>> = module
            .samples()
            .iter()
            .map(|smp| {
                module.pcm(smp).map(|pcm| {
                    let sample = dsp::SampleBuffer::from(dsp::RawSample::new(&smp, pcm));
                    let sample = TrackerSample::new(SampleBuffer::from(sample));
                    
                    (smp.to_owned(), sample)
                })
            })
            .collect();

        Self { samples }
    }
}