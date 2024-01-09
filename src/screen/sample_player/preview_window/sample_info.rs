use std::{rc::Rc, sync::Arc};

use audio_engine::{Sample, TrackerSample};

use crate::widget::waveform_view::WaveData;

#[derive(Debug, Clone)]
pub enum SampleInfo {
    None,
    Invalid {
        reason: String,
    },
    Sample {
        metadata: Sample,
        data: TrackerSample,
        waveform: Arc<WaveData>,
    },
}

impl SampleInfo {
    pub fn new(
        result: &Result<(Sample, TrackerSample), xmodits_lib::Error>,
        wavedata: Arc<WaveData>,
    ) -> Self {
        match result {
            Ok((metadata, data)) => Self::Sample {
                metadata: metadata.clone(),
                data: data.clone(),
                waveform: wavedata
            },
            Err(error) => Self::Invalid {
                reason: error.to_string(),
            },
        }
    }
    pub fn title(&self) -> String {
        match &self {
            Self::Sample { metadata, .. } => {
                let smp = metadata;
                let filename = smp.filename_pretty();
                match filename.is_empty() {
                    true => smp.name_pretty().into(),
                    false => filename.into(),
                }
            }
            Self::Invalid { .. } => "ERROR".into(),
            Self::None => "None Selected".into(),
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Invalid { .. })
    }

    pub fn waveform(&self) -> Option<&WaveData> {
        match &self {
            SampleInfo::Sample { waveform, .. } => Some(waveform.as_ref()),
            _ => None
        }
    }
}
