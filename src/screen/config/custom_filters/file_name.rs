use crate::theme::TextInputStyle;
use crate::widget::helpers::{control_filled, fill_container, text_elem};
use crate::widget::Element;
use crate::{theme, widget::helpers::control};
use data::config::filters::Name;
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, pick_list, row, scrollable, slider, text,
    text_input, Space,
};
use iced::{Alignment, Length};

#[derive(Debug, Clone)]
pub enum Message {
    Input(String),
    CaseSensitive(bool),
    Outcome(Outcome),
    Condition(Condition),
    Check(),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    #[default]
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    #[default]
    Allow,
    Reject,
}

impl Outcome {
    const ALL: &[Self] = &[Self::Allow, Self::Reject];
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Allow => "Allow if it",
                Self::Reject => "Reject if it",
            }
        )
    }
}


#[derive(Debug, Default)]
pub struct NameFilter {
    buffer: String,
    case_sensitive: bool,
    condition: Condition,
    outcome: Outcome,

    filters: Name,
}

pub fn update(filter: &mut NameFilter, msg: Message) {
    match msg {
        Message::Input(word) => filter.buffer = word,
        Message::CaseSensitive(case) => filter.case_sensitive = case,
        Message::Outcome(outcome) => filter.outcome = outcome,
        Message::Condition(condition) => filter.condition = condition,
        Message::Check() => (),

    }
}

pub fn view<'a>(filter: &'a NameFilter) -> Element<'a, Message> {
    let settings = column![
        row![
            pick_list(Outcome::ALL, Some(filter.outcome), Message::Outcome),
            pick_list(Condition::ALL, Some(filter.condition), Message::Condition),
        ]
        .spacing(5)
        .width(Length::Fill),

        text_input("Filter...", &filter.buffer)
            .style(theme::TextInputStyle::Inverted)
            .width(Length::Fill)
            .on_input(Message::Input)
            .padding(8),

        checkbox("Case Sensitive", filter.case_sensitive, Message::CaseSensitive),
        row![button("Add"), button("Add AND"), button("Add OR")]
            .align_items(Alignment::Center)
            .spacing(5),

        fill_container(scrollable(
            container(filter.filters.contains.iter().fold(column![].spacing(4), |f, y| {
                f.push(row![
                    checkbox("", false, |_| Message::Check()),
                    button(y.as_ref()).style(theme::Button::HyperlinkInverted)
                ])
            }))
            .width(Length::Fill)
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
