//! Helper functions to construct widgets
use iced::alignment::Horizontal;
use iced::widget::{button, text};

use crate::widget::{Element, Button, Text};


pub fn centered_button<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
    button(content)
        
}

pub fn text_centered<'a>(input: impl ToString) -> Text<'a> {
    text(input).horizontal_alignment(Horizontal::Center)
}