//! Hopefully nobody should ever see this, but if they do, make it... nice

use std::path::PathBuf;

use iced::widget::{button, column, container, horizontal_rule, row, scrollable, text, Space};
use iced::{window, Alignment, Task, Length, Subscription};

use crate::logger::crash_handler::SavedPanic;
use crate::widget::helpers::{control_filled, fill_container, text_icon_srnd};
use crate::widget::{Button, Container, Element, Text};
use crate::{app, icon, logger, style};

#[derive(Debug, Clone)]
pub enum Message {
    Panic(SavedPanic),
    BadModule(PathBuf),
    Shutdown,
    Ignore,
    Open(PathBuf),
}

#[derive(Debug, Default, Clone)]
pub struct Crashes {
    panics: Vec<SavedPanic>,
    bad_modules: Vec<PathBuf>,
}

impl Crashes {
    pub fn occurred(&self) -> bool {
        !self.panics.is_empty()
    }

    fn add_bad_module(&mut self, file: PathBuf) {
        tracing::error!(
            "This module might have caused the fatal error: {}",
            file.display()
        );
        self.bad_modules.push(file)
    }

    fn add_panic(&mut self, panic: SavedPanic) {
        tracing::error!("Detected Panic");
        self.panics.push(panic);
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Panic(panic) => self.add_panic(panic),
            Message::BadModule(file) => self.add_bad_module(file),
            Message::Shutdown => return window::close(app::MAIN_ID.get().cloned().unwrap()),
            Message::Open(log) => {
                let _ = open::that_detached(log);
            }
            Message::Ignore => (),
        }
        Task::none()
    }

    /// TODO:
    /// * add option to generate an even more detailed crash log if there are bad modules
    ///     * store md5 hash (to reference if it's on modarchive) + file name
    /// * Add option to save crash if the panic handler failed to save it to a file
    pub fn view(&self) -> Element<Message> {
        let has_bad_modules = !self.bad_modules.is_empty();

        let title = row![
            Space::with_width(Length::Fill),
            icon::warning().size(20),
            big("An internal error has occurred."),
            icon::warning().size(20),
            Space::with_width(Length::Fill),
        ]
        .align_y(Alignment::Center)
        .spacing(10);

        let shutdown_button = button(text_icon_srnd("Close Application", icon::error()))
            .on_press(Message::Shutdown)
            .style(style::button::cancel)
            .padding(10);

        let bad_modules = has_bad_modules.then(|| {
            let msg = "The following files might be the cause:";
            let paths = column(self.bad_modules.iter().map(|f| text(f.to_string_lossy()).into()));

            column![
                msg,
                fill_container(scrollable(paths))
                    .padding(10)
                    .style(style::container::black),
            ]
            .spacing(6)
        });

        let multiple_errors = self.panics.len() > 1;

        let errors = self.panics.iter().enumerate().map(|(idx, f)| {
            let open_log_button = multiple_errors.then(|| open_crash_button(f)).flatten();
            let line = match f.line() {
                Some(line) => text(format!("Line: {}", line)),
                None => text("Line: Unknown"),
            };
            let separator = {
                (!self.panics.is_empty() && self.panics.len() - 1 != idx)
                    .then(|| horizontal_rule(1))
            };

            column![
                text(format!("File: {}", f.file())),
                line,
                text(format!("Message: {}", &f.message())),
            ]
            .push_maybe(open_log_button)
            .push_maybe(separator)
            .spacing(10)
            .into()
        });

        let content = column![
            "Oh no... XMODITS has crashed :(",
            container(scrollable(column(errors).spacing(10)))
                .padding(10)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(style::container::black)
        ]
        .push_maybe(bad_modules)
        .padding(4)
        .spacing(6);

        let open_single_log = (!multiple_errors)
            .then(|| self.panics.first().and_then(open_crash_button))
            .flatten();

        let view = column![
            control_filled(title, content),
            row![]
                .push_maybe(open_single_log)
                .push(Space::with_width(Length::Fill))
                .push(shutdown_button)
                .align_y(Alignment::Center)
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

fn open_crash_button(panic: &SavedPanic) -> Option<Button<Message>> {
    panic.saved_to.clone().map(|f| {
        button("Open Crash Report")
            .on_press(Message::Open(f))
            .style(style::button::start)
            .padding(10)
    })
}

fn big<'a>(str: impl text::IntoFragment<'a>) -> Text<'a> {
    text(str).size(16)
}

pub fn subscription() -> Subscription<Message> {
    Subscription::batch([
        logger::bad_modules::subscription().map(Message::BadModule),
        logger::crash_handler::subscription().map(Message::Panic),
    ])
}
