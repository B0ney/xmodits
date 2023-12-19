use audio_engine::{PlayerHandle, Sample, SamplePack, TrackerSample};

#[derive(Debug, Clone)]
pub enum SampleInfo {
    Invalid { reason: String },
    Sample(Sample),
}

impl SampleInfo {
    pub fn title(&self) -> String {
        match &self {
            Self::Sample(smp) => smp.filename_pretty().to_string(),
            Self::Invalid { .. } => "ERROR".into(),
        }
    }
    
    pub fn is_error(&self) -> bool {
        return matches!(self, Self::Invalid { .. });
    }
}

impl From<&Result<(Sample, TrackerSample), xmodits_lib::Error>> for SampleInfo {
    fn from(value: &Result<(Sample, TrackerSample), xmodits_lib::Error>) -> Self {
        match value {
            Ok((smp, _)) => Self::Sample(smp.to_owned()),
            Err(e) => Self::Invalid {
                reason: e.to_string(),
            },
        }
    }
}