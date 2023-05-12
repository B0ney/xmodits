// use crate::core::cfg::FormatHint;
use crate::gui::{style, JETBRAINS_MONO};
use crate::{core::cfg::SampleRippingConfig, gui::style::Theme};
use iced::widget::{checkbox, column, container, pick_list, row, text, text_input, Space};
use iced::Alignment;
use iced::{Element, Length, Renderer};
use tracing::trace;
use xmodits_lib::exporter::AudioFormat;

const SUPPORTED_FORMATS: &[AudioFormat] = &[
    AudioFormat::WAV,
    AudioFormat::AIFF,
    AudioFormat::IFF,
    AudioFormat::RAW,
];

#[derive(Debug, Clone)]
pub enum Message {
    SetFormat(AudioFormat),
    ToggleSelfContained(bool),
    ToggleStrictLoad(bool),
    SetRecursionDepth(u8),
}

impl SampleRippingConfig {
    pub fn update(&mut self, msg: Message) {
        trace!("{:?}", &msg);
        
        match msg {
            Message::SetFormat(format) => self.exported_format = format,
            Message::ToggleSelfContained(toggle) => self.self_contained = toggle,
            Message::SetRecursionDepth(depth) => self.folder_max_depth = depth,
            Message::ToggleStrictLoad(strict) => self.strict = strict,
        }
    }

    pub fn view(&self) -> Element<Message, Renderer<Theme>> {
        let settings: _ = container(
            row![column![
                checkbox("Self Contained", self.self_contained, Message::ToggleSelfContained),
                checkbox("Strict Loading", self.strict, Message::ToggleStrictLoad),
                row![
                    pick_list(
                        SUPPORTED_FORMATS,
                        Some(self.exported_format),
                        Message::SetFormat
                    ),
                    text("Export Format"),
                ]
                .align_items(Alignment::Center)
                .spacing(5),
            ]
            .spacing(8),]
            .spacing(8),
        )
        .style(style::Container::Frame)
        .padding(8)
        .width(Length::Fill);

        container(column![text("Ripping Configuration").font(JETBRAINS_MONO), settings].spacing(10))
            .width(Length::Fill)
            .into()
    }

    pub fn view_folder_scan_depth(&self) -> Element<Message, Renderer<Theme>> {
        let settings: _ = container(
            row![
                pick_list(
                    (1..=7).collect::<Vec<u8>>(),
                    Some(self.folder_max_depth),
                    Message::SetRecursionDepth
                ),
                text("Folder Scan Depth "),
            ]
            .align_items(Alignment::Center)
            .spacing(5),
        )
        .style(style::Container::Frame)
        .padding(8)
        .width(Length::Fill);

        // container(column![text("Misc").font(JETBRAINS_MONO), settings].spacing(10))
        container(settings).width(Length::Fill).into()
    }
}
