// use crate::core::cfg::FormatHint;
use crate::gui::{style, JETBRAINS_MONO};
use crate::{core::cfg::SampleRippingConfig, gui::style::Theme};
use iced::widget::{checkbox, column, container, pick_list, row, text, text_input, Space};
use iced::Alignment;
use iced::{Element, Length, Renderer};
use xmodits_lib::exporter::AudioFormat;

#[derive(Debug, Clone)]
pub enum Message {
    SetFormat(AudioFormat),
    ToggleNoFolder(bool),
    // ToggleStrictLoad(bool),
    SetRecursionDepth(u8),
}

impl SampleRippingConfig {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SetFormat(format) => self.exported_format = format,
            Message::ToggleNoFolder(toggle) => self.no_folder = toggle,
            Message::SetRecursionDepth(depth) => self.folder_recursion_depth = depth,
            // Message::ToggleStrictLoad(strict) => self.strict = strict,
        }
    }

    pub fn view(&self) -> Element<Message, Renderer<Theme>> {
        let settings: _ = container(
            row![column![
                checkbox("No Folder", self.no_folder, Message::ToggleNoFolder),
                // checkbox("Strict Loading", self.strict, Message::ToggleStrictLoad),
                row![
                    pick_list(
                        &AudioFormat::ALL[..],
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
                    (1..4).collect::<Vec<u8>>(),
                    Some(self.folder_recursion_depth),
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
