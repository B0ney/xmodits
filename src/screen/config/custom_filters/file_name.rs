use crate::theme::TextInputStyle;
use crate::widget::helpers::{text_elem, fill_container, control_filled};
use crate::widget::Element;
use crate::{theme, widget::helpers::control};
use data::config::filters::Name;
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, pick_list, row, scrollable, slider, text,
    text_input, Space,
};
use iced::{Alignment, Length};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    A,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    Contains,
    StartsWith,
    EndsWith,
}
impl Condition {
    const ALL: &[Self] = &[Self::Contains, Self::StartsWith, Self::EndsWith];
}
impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Condition::Contains => "Contains",
                Condition::StartsWith => "Starts With",
                Condition::EndsWith => "Ends with",
            }
        )
    }
}

pub fn update(filter: &mut Name, msg: Message) {}

pub fn view<'a>(filter: &'a Name) -> Element<'a, Message> {
    let settings = column![
        row![
            column![
                "Value",
                text_input("Add Filter", "")
                    .style(theme::TextInputStyle::Inverted)
                    .width(Length::Fill),
            ]
            .width(Length::Fill)
            .spacing(8),
            column![
                "Condition",
                pick_list(Condition::ALL, Some(Condition::Contains), |_| Message::A)
            ]
            .width(Length::Fill)
            .spacing(8),
        ]
        .width(Length::Fill)
        .spacing(5),
        row![button("Add"), button("Add AND"), button("Add OR")]
            .align_items(Alignment::Center)
            .spacing(5),
        fill_container(scrollable(
            container(filter.contains.iter().fold(column![].spacing(4), |f, y| {
                f.push(row![
                    checkbox("", false, |_| Message::A),
                    button(y.as_ref()).style(theme::Button::HyperlinkInverted)
                ])
            })).width(Length::Fill)
            
        ))
            .padding(8)
            .style(theme::Container::Black),
        row![
            button("Remove Selected"),
            Space::with_width(Length::Fill),
            button("Clear")
        ],
    ]
    .spacing(8);
    control_filled("File Name", settings).into()
}
