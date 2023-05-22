use std::path::PathBuf;

use crate::core::cfg::GeneralConfig;
use crate::gui::style::{self, Theme, Themes};
use crate::gui::JETBRAINS_MONO;
use iced::widget::container;
use iced::widget::{checkbox, column, pick_list, row, text};
use iced::Alignment;
use iced::{Element, Length, Renderer};
use tracing::trace;

#[derive(Debug, Clone)]
pub enum Message {
    SetTheme(Themes),
    SetLogDirectory(PathBuf),
    SetWorkerThreads(usize),
    NonGuiQuietOutput(bool),
    NonGuiUseCwd(bool),
}

impl GeneralConfig {
    pub fn update(&mut self, msg: Message) {
        trace!("{:?}", &msg);

        match msg {
            Message::SetTheme(theme) => self.theme = theme,
            Message::SetWorkerThreads(workers) => self.worker_threads = workers,
            Message::NonGuiQuietOutput(quiet_output) => self.non_gui_quiet_output = quiet_output,
            Message::NonGuiUseCwd(use_cwd) => self.non_gui_use_cwd = use_cwd,
            Message::SetLogDirectory(log_dir) => self.logging_path = Some(log_dir),
        }
    }

    pub fn view(&self) -> Element<Message, Renderer<Theme>> {
        let setting: _ = container(
            column![
                row![
                    pick_list(&Themes::ALL[..], Some(self.theme), Message::SetTheme),
                    text("Theme"),
                ]
                .align_items(Alignment::Center)
                .spacing(5),
                row![
                    pick_list(
                        [0usize, 1, 2, 4, 6, 8, 10, 12, 16]
                            .into_iter()
                            .map(Workers)
                            .collect::<Vec<Workers>>(),
                        Some(Workers(self.worker_threads)),
                        |f| Message::SetWorkerThreads(f.0)
                    ),
                    text("Worker Threads"),
                ]
                .align_items(Alignment::Center)
                .spacing(5),
                checkbox(
                    "(non-gui) Quiet output",
                    self.non_gui_quiet_output,
                    Message::NonGuiQuietOutput
                ),
                checkbox(
                    "(non-gui) Use current working directory",
                    self.non_gui_use_cwd,
                    Message::NonGuiUseCwd
                ),
            ]
            .spacing(5),
        )
        .style(style::Container::Frame)
        .padding(8)
        .height(Length::Fill)
        .width(Length::Fill);

        container(column![text("Settings").font(JETBRAINS_MONO), setting,].spacing(15))
            .width(Length::Fill)
            .into()
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq)]
#[repr(transparent)]
struct Workers(pub usize);

impl std::fmt::Display for Workers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "Automatic"),
            n => write!(f, "{}", format!("{}", n)),
        }
    }
}
