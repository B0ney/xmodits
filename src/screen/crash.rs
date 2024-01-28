//! Hopefully nobody should every see this, but if they do, make it... nice

use std::collections::HashSet;
use std::path::PathBuf;

use iced::widget::{button, column, container, horizontal_rule, row, scrollable, text, Space};
use iced::{window, Alignment, Command, Length, Subscription};

use crate::logger::crash_handler::Panic;
use crate::widget::helpers::{control_filled, fill_container, text_icon_srnd};
use crate::widget::{Collection, Container, Element, Text};
use crate::{icon, logger, theme};

#[derive(Debug, Clone)]
pub enum Message {
    Panic(Panic),
    BadModule(PathBuf),
    Shutdown,
}

#[derive(Debug, Default)]
pub struct Crashes {
    pub panics: HashSet<Panic>,
    pub bad_modules: Vec<PathBuf>,
}

impl Crashes {
    pub fn occurred(&self) -> bool {
        !self.panics.is_empty()
    }

    pub fn add_bad_module(&mut self, file: PathBuf) {
        tracing::error!(
            "This module might have caused the fatal error: {}",
            file.display()
        );
        self.bad_modules.push(file)
    }

    pub fn add_panic(&mut self, panic: Panic) {
        tracing::error!("Detected Panic");
        let _ = self.panics.insert(panic);
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Panic(panic) => self.add_panic(panic),
            Message::BadModule(file) => self.add_bad_module(file),
            Message::Shutdown => return window::close(window::Id::MAIN),
        }
        Command::none()
    }

    /// TODO:
    /// * provide link to generated crash log
    /// * add option to generate an even more detailed crash log if there are bad modules
    ///     * store md5 hash (to reference if it's on modarchive) + file name
    ///
    pub fn view(&self) -> Element<Message> {
        let has_bad_modules = !self.bad_modules.is_empty();

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
            let msg = "The following files might be the cause:";
            let paths = column(self.bad_modules.iter().map(|f| text(f.display()).into()));

            column![
                msg,
                fill_container(scrollable(paths)).padding(10).style(theme::Container::Black),
                "A *basic* crash log has been automatically generated and saved to your *Downloads* folder. \
                \nBut you can also create a more detailed report that includes those problematic files."
            ]
            .spacing(6)
        });

        let errors = self.panics.iter().enumerate().map(|(idx, f)| {
            column![
                text(format!("File: {}", f.file)),
                if let Some(line) = f.line {
                    text(format!("Line: {}", line))
                } else {
                    text(format!("Line: Unknown"))
                },
                text(format!("Message: {}", &f.message)),
            ]
            .push_maybe({
                let should_show = self.panics.len() >= 1 && self.panics.len() - 1 != idx;
                should_show.then(|| horizontal_rule(1))
            })
            .spacing(6)
            .into()
        });

        let content = column![
            "Oh no... XMODITS has crashed :(",
            container(scrollable(column(errors).spacing(10)))
                .padding(10)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::Container::Black)
        ]
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
}

fn big<'a>(str: impl ToString) -> Text<'a> {
    text(str).size(16)
}

pub fn subscription() -> Subscription<Message> {
    Subscription::batch([
        logger::bad_modules::subscription().map(Message::BadModule),
        logger::crash_handler::subscription().map(Message::Panic),
    ])
}
