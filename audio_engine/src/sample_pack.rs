use std::path::{PathBuf, Path};

use xmodits_lib::dsp;
use xmodits_lib::{Module, Sample};

use crate::sample::buffer::SampleBuffer;
use crate::sample::TrackerSample;

#[derive(Debug)]
pub struct SamplePack {
    pub name: String,
    pub format: String,
    pub total_samples: usize,
    pub total_sample_size: usize,
    pub path: Option<PathBuf>,
    pub samples: Vec<Result<(Sample, TrackerSample), xmodits_lib::Error>>,
}

impl SamplePack {
    pub fn build(module: &dyn Module) -> Self {
        let name = module.name().trim().to_owned();
        let format = module.format().to_owned();
        let total_samples = module.total_samples();
        let total_sample_size = module.samples().iter().map(|m| m.length as usize).sum();

        let samples = module
            .samples()
            .iter()
            .map(|smp| {
                module.pcm(smp).map(|pcm| {
                    let sample = dsp::SampleBuffer::from(dsp::RawSample::new(smp, pcm));
                    let sample = TrackerSample::new(SampleBuffer::from(sample));

                    (smp.to_owned(), sample)
                })
            })
            .collect();

        Self {
            name,
            format,
            total_samples,
            total_sample_size,
            samples,
            path: None,
        }
    }

    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    pub fn matches_path(&self, path: impl AsRef<Path>) -> bool {
        matches!(&self.path, Some(p) if p == path.as_ref())
    }
}
