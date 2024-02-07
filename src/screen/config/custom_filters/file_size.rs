use std::fmt::Display;

use crate::widget::helpers::control;
use crate::theme::TextInputStyle;
use crate::widget::Element;
use data::config::filters::{size::Modifier, Size};
use iced::widget::{column, pick_list, row, slider, text_input, text, horizontal_rule};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SetMin(u64),
    SetMax(u64),
    SetMinModifier(Modifier),
    SetMaxModifier(Modifier),
    Ignore,
}

pub fn view(filter: &Size) -> Element<Message> {
    let settings = column![
        row![
            "Min:",
            text_input("", &format!("{}", filter.min)).on_input(|input| {
                if input.is_empty() {
                    return Message::SetMin(0);
                }
                input
                    .parse::<u64>()
                    .map(Message::SetMin)
                    .unwrap_or(Message::Ignore)
            })
            .style(TextInputStyle::Inverted),
            pick_list(Modifier::ALL, Some(filter.min_modifier), Message::SetMinModifier)
        ]
        .spacing(8)
        .align_items(iced::Alignment::Center),
        row![
            "Max:",
            text_input("", &format!("{}", filter.max)).on_input(|input| {
                if input.is_empty() {
                    return Message::SetMax(0);
                }

                input
                    .parse::<u64>()
                    .map(Message::SetMax)
                    .unwrap_or(Message::Ignore)
            })
            .style(TextInputStyle::Inverted),
            pick_list(Modifier::ALL, Some(filter.max_modifier), Message::SetMaxModifier)
        ]
        .spacing(8)
        .align_items(iced::Alignment::Center),
        // horizontal_rule(1),
        // text("Hint: 1000 bytes = 1 KB, 1000 KB = 1 MB"),
    ]
    .spacing(8);
    control("File Size", settings).into()
}

pub fn update(filter: &mut Size, msg: Message) {
    tracing::info!("{:?}", msg);
    match msg {
        Message::SetMin(min) => filter.min = min,
        Message::SetMax(max) => filter.max = max,
        Message::SetMinModifier(modi) => filter.min_modifier = modi,
        Message::SetMaxModifier(modi) => filter.max_modifier = modi,
        Message::Ignore => (),
    }
}
