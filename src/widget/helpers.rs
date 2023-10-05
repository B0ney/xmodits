//! Helper functions to construct widgets

use iced::alignment::Horizontal;
use iced::widget::{button, container, text};
use iced::Length;

use crate::theme;
use crate::widget::{Button, Container, Element, Text};

/// TODO
pub fn centered_button<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> Button<'a, Message> {
    button(content)
}

pub fn action<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    message: Message,
    predicate: impl Fn() -> bool,
) -> Button<'a, Message> {
    button(content).on_press_maybe(predicate().then_some(message))
}

pub fn centered_text<'a>(input: impl ToString) -> Text<'a> {
    text(input).horizontal_alignment(Horizontal::Center)
}

pub fn warning<'a>(predicate: impl Fn() -> bool, warning: impl ToString) -> Option<Text<'a>> {
    predicate().then_some(text(warning).style(theme::Text::Error))
}

pub fn centered_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> Container<'a, Message> {
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
}


pub fn fill_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> Container<'a, Message> {
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
}