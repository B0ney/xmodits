use std::fmt::Display;

use crate::theme::TextInputStyle;
use crate::widget::helpers::control;
use crate::widget::Element;
use data::config::filters::{size::Modifier, Size};
use iced::widget::{button, checkbox, column, horizontal_rule, pick_list, row, slider, text, text_input};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    DatePicker,
}

pub fn update(msg: Message) {}

pub fn view<'a>() -> Element<'a, Message> {
    // IDEA: buttons could have a tooltip showing the date&time in more detail
    // TODO: buttons must bring up date and time picker, should be an overlay
    control(
        "File Date",
        column![
            pick_list(["Created", "Modified"].as_slice(), Some("Created"), |_| {
                Message::DatePicker
            }),
            horizontal_rule(1),
            row![
                row!["From:", button("2005-12-12").on_press(Message::DatePicker)]
                    .align_items(iced::Alignment::Center)
                    .spacing(4),
                row!["To:", button("2009-12-12").on_press(Message::DatePicker)]
                    .align_items(iced::Alignment::Center)
                    .spacing(4)
            ]
            .align_items(iced::Alignment::Center)

            .spacing(4),
            
        ]
        .spacing(4),
    )
    .into()
}
