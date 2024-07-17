use crate::theme::TextInputStyle;
use crate::widget::helpers::{control_filled, fill_container, text_elem};
use crate::widget::Element;
use crate::{theme, widget::helpers::control};
use data::config::filters::Name;
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, pick_list, row, scrollable, slider, text,
    text_input, Space,
};
use iced::{Alignment, Task, Length};
use once_cell::sync::Lazy;

static TEXTBOX_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

#[derive(Debug, Clone)]
pub enum Message {
    Input(String),
    CaseSensitive(bool),
    Outcome(Outcome),
    Condition(Condition),
    Check(),
    Add,
}

#[derive(Debug, Default)]
pub struct NameFilter {
    buffer: String,
    case_sensitive: bool,
    condition: Condition,
    outcome: Outcome,
    filters: Name,
}

impl NameFilter {
    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Input(word) => self.buffer = word,
            Message::CaseSensitive(case) => self.case_sensitive = case,
            Message::Outcome(outcome) => self.outcome = outcome,
            Message::Condition(condition) => self.condition = condition,
            Message::Check() => (),
            Message::Add => {
                if self.buffer.is_empty() {
                    return text_input::focus(TEXTBOX_ID.clone());
                }
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        let input_box = text_input("Filter...", &self.buffer)
            .id(TEXTBOX_ID.clone())
            .style(theme::TextInputStyle::Inverted)
            .width(Length::Fill)
            .on_input(Message::Input)
            .padding(8);

        let set_conditions = row![
            pick_list(Outcome::ALL, Some(self.outcome), Message::Outcome),
            pick_list(Condition::ALL, Some(self.condition), Message::Condition),
        ]
        .spacing(5)
        .width(Length::Fill);

        let add_buttons = row![
            button("Add").on_press(Message::Add),
            button("Add AND"),
            button("Add OR")
        ]
        .align_items(Alignment::Center)
        .spacing(5);

        let filter_list = fill_container(scrollable(
            container(self.filters.contains.iter().fold(column![].spacing(4), |f, y| {
                f.push(row![
                    checkbox("", false, |_| Message::Check()),
                    button(y.as_ref()).style(theme::Button::HyperlinkInverted)
                ])
            }))
            .width(Length::Fill),
        ))
        .padding(8)
        .style(theme::Container::Black);

        let settings = column![
            set_conditions,
            input_box,
            checkbox("Case Sensitive", self.case_sensitive, Message::CaseSensitive),
            add_buttons,
            filter_list,
            row![
                button("Remove Selected"),
                Space::with_width(Length::Fill),
                button("Clear")
            ],
        ]
        .spacing(8);
        control_filled("File Name", settings).into()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    #[default]
    Contains,
    StartsWith,
    EndsWith,
}

impl Condition {
    const ALL: &'static [Self] = &[Self::Contains, Self::StartsWith, Self::EndsWith];
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
    const ALL: &'static [Self] = &[Self::Allow, Self::Reject];
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
