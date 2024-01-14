use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use crate::icon;
use crate::theme;
use crate::widget::helpers::centered_container;
use crate::widget::waveform_view::WaveData;
use crate::widget::{Collection, Element};

use audio_engine;

use audio_engine::TrackerSample;
use iced::widget::{button, horizontal_rule, row, text, Space, column};
use iced::{Alignment, Length};

use super::Message;


#[derive(Debug, Clone)]
pub struct SamplePack {
    name: String,
    path: PathBuf,
    samples: Vec<SampleResult>
}

impl SamplePack {
    pub fn new(name: String, path: PathBuf, samples: Vec<SampleResult>) -> Self {
        Self {
            name, 
            path,
            samples
        }
    }

    pub fn inner(&self) -> &[SampleResult] {
        &self.samples
    }

    pub fn waveform(&self, index: usize) -> Option<&WaveData> {
        self.samples.get(index).and_then(SampleResult::waveform)
    }

    pub fn view_sample_info(&self, index: usize) -> Element<Message> {
        self.inner()[index].view_sample_info()
    }

    pub fn tracker_sample(&self, index: usize) -> Option<TrackerSample> {
        self.inner().get(index).and_then(SampleResult::tracker_sample)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn is_empty(&self) -> bool {
        self.inner().len() == 0
    }
}

#[derive(Debug, Clone)]
pub enum SampleResult {
    Invalid(String),
    Valid {
        metadata: audio_engine::Metadata,
        buffer: TrackerSample,
        waveform: WaveData,
    },
}

impl SampleResult {
    pub fn waveform(&self) -> Option<&WaveData> {
        match self {
            SampleResult::Invalid(_) => None,
            SampleResult::Valid { waveform, .. } => Some(waveform),
        }
    }

    pub fn title(&self) -> String {
        match self {
            SampleResult::Invalid(_) => "ERROR".into(),
            SampleResult::Valid { metadata, .. } => metadata.filename_pretty().into(),
        }
    }

    pub fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }

    pub fn tracker_sample(&self) -> Option<TrackerSample> {
        match &self {
            SampleResult::Valid { buffer,.. } => Some(buffer.clone()),
            _ => None
        }
    }

    pub fn view_sample(&self, index: usize) -> Element<Message> {
        let error_icon = || {
            row![]
                .push(Space::with_width(Length::Fill))
                .push(icon::warning())
                .align_items(iced::Alignment::Center)
        };

        let title = row![]
            .push(text(match self.title() {
                title if title.is_empty() => format!("{}", index + 1),
                title => format!("{} - {}", index + 1, title),
            }))
            .push_maybe(self.is_invalid().then_some(error_icon()))
            .spacing(5);

        let theme = match self.is_invalid() {
            true => theme::Button::EntryError,
            false => theme::Button::Entry,
        };

        row![
            button(title)
                .width(Length::Fill)
                .style(theme)
                .on_press(Message::Select(index)),
            Space::with_width(15)
        ]
        .into()
    }

    pub fn view_sample_info(&self) -> Element<Message> {
        match self {
            SampleResult::Invalid(reason) => centered_container(text(reason)).into(),
            SampleResult::Valid { metadata, .. } => {
                let smp = metadata;

                let sample_name =
                    (!smp.name.trim().is_empty()).then_some(text(format!("Name: {}", smp.name.trim())));

                let sample_filename = smp
                    .filename
                    .as_ref()
                    .map(|s| s.trim())
                    .and_then(|s| (!s.is_empty()).then_some(text(format!("File Name: {}", s))));

                let metadata = text(format!(
                    "{} Hz, {}-bit ({}), {}",
                    smp.rate,
                    smp.bits(),
                    if smp.is_signed() { "Signed" } else { "Unsigned" },
                    if smp.is_stereo() { "Stereo" } else { "Mono" },
                ));

                let round_100th = |x: f32| (x * 100.0).round() / 100.0;

                let duration = Duration::from_micros(
                    ((smp.length_frames() as f64 / smp.rate as f64) * 1_000_000.0) as u64,
                );
                let duration_secs = round_100th(duration.as_secs_f32());
                let plural = if duration_secs == 1.0 { "" } else { "s" };
                let duration = text(format!("Duration: {} sec{plural}", duration_secs));

                let size = match smp.length {
                    l if l < 1000 => format!("{} bytes", l),
                    l if l < 1_000_000 => format!("{} KB", round_100th(l as f32 / 1000.0)),
                    l => format!("{} MB", round_100th(l as f32 / 1_000_000.0)),
                };

                let info = column![]
                    .push_maybe(sample_name)
                    .push_maybe(sample_filename)
                    .push(duration)
                    .push(text(format!("Size: {}", size)))
                    .push(text(format!("Loop type: {:#?}", smp.looping.kind())))
                    .push(text(format!("Internal Index: {}", smp.index_raw())))
                    .push(horizontal_rule(1))
                    .push(metadata)
                    .push(horizontal_rule(1))
                    .spacing(5)
                    .align_items(Alignment::Center);
                centered_container(info).into()
            }
        }
    }
}
