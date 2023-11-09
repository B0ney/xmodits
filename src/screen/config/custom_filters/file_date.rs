use std::fmt::Display;

use crate::widget::helpers::control;
use crate::widget::Element;
use crate::{icon, theme};
use crate::{theme::TextInputStyle, widget::helpers::text_icon};
use data::config::filters::{size::Modifier, Size};
use iced::widget::tooltip::Position;
use iced::widget::{
    button, checkbox, column, horizontal_rule, pick_list, row, slider, text, text_input, tooltip,
};

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
        row![
            pick_list(["Created", "Modified"].as_slice(), Some("Created"), |_| {
                Message::DatePicker
            }),
            tooltip(
                button("2005-12-12")
                    .on_press(Message::DatePicker)
                    .padding(8)
                    .style(theme::Button::Dark),
                "12 December 2005 @ 19:00",
                Position::Bottom,
            )
            .padding(6)
            .style(theme::Container::Frame),
            "-",
            tooltip(
                button("2009-12-12")
                    .on_press(Message::DatePicker)
                    .padding(8)
                    .style(theme::Button::Dark),
                "12 December 2009 @ 13:00",
                Position::Bottom,
            )
            .padding(6)
            .style(theme::Container::Frame),
        ]
        .align_items(iced::Alignment::Center)
        .spacing(8),
    )
    .into()
}
