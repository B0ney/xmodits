use std::path::PathBuf;
use crate::gui::{JETBRAINS_MONO, style};
use crate::{
    core::cfg::SampleRippingConfig,
    gui::style::{Theme},
};
use iced::widget::{Space, text_input};
use iced::widget::{checkbox, column, pick_list, row, text};
use iced::{widget::container, Alignment, Element, Length, Renderer};

#[derive(Debug, Clone)]
pub enum Message {
    SetDestination(PathBuf),
    SetHint(Option<String>),
    ToggleEmbedLoopPoint(bool),
    ToggleNoFolder(bool),
}

pub fn view(config: &SampleRippingConfig) -> Element<Message, Renderer<Theme>> {
    let settings: _ = container(
        row![
            column![
                checkbox("No Folder", config.no_folder, Message::ToggleNoFolder),
                checkbox("Embed sample loops", config.embed_loop_points, Message::ToggleEmbedLoopPoint),
                // row![
                //     pick_list(
                //         vec![Some("umx".to_string()), None],
                //         Some(config.hint),
                //         Message::IndexPadding
                //     )
                //     .width(Length::Units(50)),
                //     // Space::with_width(Length::FillPortion(4)),
                //     text("Padding"),
                // ]
                // .spacing(5)
                // .align_items(Alignment::Center),
                // checkbox("Index Only", config.hint, Message::IndexOnly),
                // checkbox("Preserve Index", name_cfg.index_raw, Message::IndexRaw),
            ]
            .spacing(8),

        ]
        .spacing(8),
    )
    .style(style::Container::Frame)
    .padding(8)
    .width(Length::Fill);

    container(column![text("Ripping Configuration").font(JETBRAINS_MONO), settings].spacing(10))
        .width(Length::Fill)
        .into()
}

/// input bar
pub fn destination(config: &SampleRippingConfig)  -> Element<Message, Renderer<Theme>> {
    let input: _ = text_input("Output Directory", &format!("{}", config.destination.display()), |s| {
            Message::SetDestination(PathBuf::new().join(s))
        })
        .padding(10);

    input.into()
}

pub fn update(config: &mut SampleRippingConfig, msg: Message) {
    match msg {
        Message::SetDestination(path) => config.destination = path,
        Message::SetHint(hint) => config.hint = hint,
        Message::ToggleEmbedLoopPoint(toggle) => config.embed_loop_points = toggle,
        Message::ToggleNoFolder(toggle) => config.no_folder = toggle,
    }
}