use std::path::PathBuf;

use iced::widget::{button, column, horizontal_rule, row, scrollable, text};
use iced::{Alignment, Length};

use crate::app::Message;
use crate::logger::crash_handler::Panic;
use crate::widget::helpers::centered_container;
use crate::widget::{Collection, Container, Element, Text};
use crate::{icon, theme};

pub fn view<'a>(
    panics: &'a [Panic],
    bad_modules: &'a [PathBuf],
    is_ripping: bool,
) -> Element<'a, Message> {
    let shutdown_button = button("Close Application")
        .on_press(Message::Shutdown)
        .style(theme::Button::Cancel)
        .padding(5);

    let bad_modules = (!bad_modules.is_empty()).then(|| {
        let msg = big("Here are the list of files that might have caused this:");
        let paths = column(bad_modules.iter().map(|f| text(f.display()).into()));

        column![
            msg,
            horizontal_rule(1),
            scrollable(paths),
            horizontal_rule(1)
        ]
        .align_items(Alignment::Center)
        .padding(4)
    });

    let title = row![
        icon::warning().size(20),
        big("An internal error has occurred."),
        icon::warning().size(20)
    ]
    .align_items(Alignment::Center)
    .spacing(4);

    let rip = is_ripping.then(|| {
        big("The application seems to have crashed \
        while it was ripping, you should close all dialog boxes.")
    });

    let view = column![title]
        .push_maybe(rip)
        .push_maybe(bad_modules)
        .push(shutdown_button)
        .padding(4)
        .spacing(6)
        .align_items(Alignment::Center);

    let view = centered_container(view).style(theme::Container::Black);

    Container::new(view)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(15)
        .into()
}

fn big<'a>(str: impl ToString) -> Text<'a> {
    text(str).size(16)
}
