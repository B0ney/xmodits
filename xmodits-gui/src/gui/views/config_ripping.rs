// use crate::core::cfg::FormatHint;
use crate::gui::{style, JETBRAINS_MONO};
use crate::{core::cfg::SampleRippingConfig, gui::style::Theme};
use iced::widget::{checkbox, column, container, pick_list, row, text, text_input};
use iced::Alignment;
use iced::{Element, Length, Renderer};
use std::path::PathBuf;
use xmodits_lib::exporter::AudioFormat;

#[derive(Debug, Clone)]
pub enum Message {
    SetDestination(PathBuf),
    // SetHint(FormatHint),
    SetFormat(AudioFormat),
    // ToggleEmbedLoopPoint(bool),
    ToggleNoFolder(bool),
    SetRecursionDepth(u8),
}

impl SampleRippingConfig {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SetDestination(path) => self.destination = path,
            Message::SetFormat(format) => self.exported_format = format,
            // Message::ToggleEmbedLoopPoint(toggle) => self.embed_loop_points = toggle,
            Message::ToggleNoFolder(toggle) => self.no_folder = toggle,
            Message::SetRecursionDepth(depth) => self.folder_recursion_depth = depth,
        }
    }

    pub fn view(&self) -> Element<Message, Renderer<Theme>> {
        let settings: _ = container(
            row![column![
                checkbox("No Folder", self.no_folder, Message::ToggleNoFolder),
                // checkbox(
                //     "Embed Sample Loops",
                //     self.embed_loop_points,
                //     Message::ToggleEmbedLoopPoint
                // ),
                row![
                    pick_list(&AudioFormat::ALL[..], Some(self.exported_format), Message::SetFormat),
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

    pub fn view_destination_bar(&self) -> Element<Message, Renderer<Theme>> {
        let input: _ = text_input(
            "Output Directory",
            &format!("{}", self.destination.display()),
            |s| Message::SetDestination(PathBuf::new().join(s)),
        )
        .padding(10);

        input.into()
    }
}
