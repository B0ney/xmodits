//! Hopefully nobody should every see this, but if they do, make it... nice

use std::path::PathBuf;

use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Length};

use crate::app::Message;
use crate::logger::crash_handler::Panic;
use crate::widget::helpers::{centered_container, control_filled, text_icon_srnd};
use crate::widget::{Collection, Container, Element, Text};
use crate::{icon, theme};

/// TODO: 
/// * provide link to generated crash log
/// * add option to generate an even more detailed crash log if there are bad modules
///     * store md5 hash (to reference if it's on modarchive) + file name
/// 
pub fn view<'a>(panics: &'a [Panic], bad_modules: &'a [PathBuf]) -> Element<'a, Message> {
    let has_bad_modules = !bad_modules.is_empty();

    let title = row![
        Space::with_width(Length::Fill),
        icon::warning().size(20),
        big("An internal error has occurred."),
        icon::warning().size(20),
        Space::with_width(Length::Fill),
    ]
    .align_items(Alignment::Center)
    .spacing(10);

    let shutdown_button = button(text_icon_srnd("Close Application", icon::error()))
        .on_press(Message::Shutdown)
        .style(theme::Button::Cancel)
        .padding(10);

    let report_button = button(text_icon_srnd("Generate Detailed Report", icon::save()))
        .on_press(Message::Shutdown)
        .style(theme::Button::Start)
        .padding(10);

    let report_button = (has_bad_modules).then(|| report_button);

    let open_log_button = button("Open Crash Report")
        .on_press(Message::Shutdown)
        .style(theme::Button::Start)
        .padding(10);

    let bad_modules = has_bad_modules.then(|| {
        let msg = big("Here are a list of files that might have caused this:");
        let paths = column(bad_modules.iter().map(|f| text(f.display()).into()));

        column![
            msg,
            centered_container(scrollable(paths)).style(theme::Container::Black),
            "A *basic* crash log has been automatically generated and saved to your *Downloads* folder. \
            \nBut you can also create a more detailed report that includes those problematic files."
        ]
        .spacing(6)
    });

    let first = panics.first().map(|f| {
        column![
            big("Message: "),
            container(text(f.message.clone()))
                .padding(10)
                .width(Length::Fill)
                // .height(Length::Fill)
                .style(theme::Container::Black)
        ]
        .spacing(6)
    });

    let content = column![]
        .push_maybe(first)
        .push_maybe(bad_modules)
        .padding(4)
        .spacing(6);

    let view = column![
        control_filled(title, content),
        row![]
            .push_maybe(report_button)
            .push_maybe(has_bad_modules.then(|| Space::with_width(Length::Fill)))
            .push(open_log_button)
            .push(Space::with_width(Length::Fill))
            .push(shutdown_button)
            .align_items(Alignment::Center)
            .width(Length::Fill)
    ]
    .spacing(10);

    Container::new(view)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(15)
        .into()
}

fn big<'a>(str: impl ToString) -> Text<'a> {
    text(str).size(16)
}
