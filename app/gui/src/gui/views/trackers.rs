use crate::gui::style::{self, Theme};
use crate::gui::{icons, JETBRAINS_MONO};
use iced::widget::Space;
use iced::widget::{button, checkbox, column, pick_list, row, scrollable, text};
use iced::{widget::container, Element, Length, Renderer};
use iced::{Alignment, Command};
use std::path::{Path, PathBuf};
use tracing::{info, warn};
use xmodits_lib::{load_module, TrackerModule};

#[derive(Debug, Clone)]
pub enum Message {
    // Remove(usize),
    Add(Option<Vec<PathBuf>>),
    Probe(usize),
    Clear,
    Select((usize, bool)),
    SelectAll(bool),
    DeleteSelected,
    TrackerInfo(Option<Info>),
    AddFileDialog,
    AddFolderDialog,
}

#[derive(Default, Debug, Clone)]
pub struct Info {
    module_name: String,
    format: String,
    samples: usize,
    path: PathBuf,
    total_sample_size: usize,
}

impl Info {
    pub fn read(tracker: TrackerModule, path: PathBuf) -> Self {
        Self {
            module_name: tracker.module_name().to_owned(),
            format: tracker.format().to_owned(),
            samples: tracker.number_of_samples(),
            path,
            total_sample_size: tracker
                .list_sample_data()
                .iter()
                .map(|f| f.len)
                .sum::<usize>()
                / 1024,
        }
    }
}
struct File {
    path: PathBuf,
    filename: String,
    selected: bool,
}

impl File {
    pub fn new(path: PathBuf) -> Self {
        Self {
            filename: path.file_name().unwrap().to_string_lossy().to_string(),
            path,
            selected: false,
        }
    }
}

#[derive(Default)]
pub struct Trackers {
    paths: Vec<File>,
    current: Option<Info>,
    all_selected: bool,
}

impl Trackers {
    pub fn add(&mut self, path: PathBuf) {
        if !self.paths.iter().map(|e| &e.path).any(|x| x == &path) {
            self.paths.push(File::new(path));
        }
    }
    // TODO: explore draining iterator
    pub fn delete_selected(&mut self) {
        if self.paths.len() == self.total_selected() {
            self.paths.clear();
            self.current = None;
            return;
        }
        let mut i = 0;
        while i < self.paths.len() {
            let path = &self.paths[i];
            if path.selected {
                if self.current_exists(&path.path) {
                    self.current = None;
                }
                let _ = self.paths.remove(i);
            } else {
                i += 1;
            }
        }
        self.all_selected = false;
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            // Message::Remove(idx) => {
            //     if idx < self.paths.len() {
            //         // self.paths.swap_remove(idx); // faster but not user friendly
            //         self.paths.remove(idx);
            //     }
            // }
            Message::Probe(idx) => {
                let path = &self.paths[idx].path;
                if !self.current_exists(path) {
                    return Command::perform(tracker_info(path.to_owned()), Message::TrackerInfo);
                }
            }
            Message::Clear => {
                self.paths.clear();
                self.current = None;
            }
            Message::Select((idx, toggle)) => {
                self.paths[idx].selected = toggle;
                if !toggle {
                    self.all_selected = toggle
                }
            }
            Message::SelectAll(b) => {
                self.all_selected = b;
                self.paths.iter_mut().for_each(|f| f.selected = b)
            }
            Message::DeleteSelected => self.delete_selected(),
            Message::TrackerInfo(module) => {
                if module.is_some() {
                    self.current = module;
                }
            }
            Message::AddFileDialog => {
                return Command::perform(
                    async {
                        // TODO: make this a separate async function
                        rfd::AsyncFileDialog::new()
                            .pick_files()
                            .await
                            .map(|filehandles| {
                                filehandles
                                    .into_iter()
                                    .map(|d| d.path().to_owned())
                                    .collect()
                            })
                    },
                    Message::Add,
                )
            }
            Message::AddFolderDialog => {
                return Command::perform(
                    async {
                        // TODO: make this a separate async function
                        rfd::AsyncFileDialog::new()
                            .pick_folders()
                            .await
                            .map(|filehandles| {
                                filehandles
                                    .into_iter()
                                    .map(|d| d.path().to_owned())
                                    .collect()
                            })
                    },
                    Message::Add,
                )
            }
            Message::Add(path) => {
                if let Some(paths) = path {
                    paths.into_iter().for_each(|path| self.add(path));
                }
            }
        }
        Command::none()
    }

    pub fn total_modules(&self) -> usize {
        self.paths.len()
    }

    pub fn cloned_paths(&self) -> Vec<PathBuf> {
        self.paths.iter().map(|f| f.path.to_owned()).collect()
    }

    pub fn current_exists(&self, path: &Path) -> bool {
        matches!(&self.current, Some(d) if d.path == path)
    }

    pub fn total_selected(&self) -> usize {
        self.paths.iter().filter(|f| f.selected).count()
    }

    pub fn view_trackers(&self) -> Element<Message, Renderer<Theme>> {
        let total_modules: _ =
            text(format!("Modules: {}", self.total_modules())).font(JETBRAINS_MONO);
        let total_selected: _ =
            text(format!("Selected: {}", self.total_selected())).font(JETBRAINS_MONO);

        let tracker_list: _ = if self.paths.is_empty() {
            container(text("Drag and drop").font(JETBRAINS_MONO))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
        } else {
            container(scrollable(self.paths.iter().enumerate().fold(
                column![].spacing(10).padding(5),
                |s, (idx, gs)| {
                    s.push(row![
                        button(
                            row![
                                checkbox("", gs.selected, move |b| Message::Select((idx, b))),
                                text(&gs.filename),
                            ]
                            .spacing(1)
                        )
                        .width(Length::Fill)
                        .on_press(Message::Probe(idx))
                        .padding(4)
                        .style(style::button::Button::NormalPackage),
                        Space::with_width(Length::Units(15))
                    ])
                },
            )))
            .height(Length::Fill)
        };
        let bottom_button = row![
            button("Add").padding(10).on_press(Message::AddFileDialog),
            button("Add Folder")
                .padding(10)
                .on_press(Message::AddFolderDialog),
            Space::with_width(Length::Fill),
            button(row![icons::delete_icon(), "Delete Selected"])
                .padding(10)
                .on_press(Message::DeleteSelected),
            button("Clear").padding(10).on_press(Message::Clear),
        ]
        .spacing(10);

        container(
            column![
                row![
                    total_modules,
                    total_selected,
                    Space::with_width(Length::Fill),
                    checkbox("Select all", self.all_selected, Message::SelectAll)
                        .style(style::checkbox::CheckBox::PackageDisabled),
                ]
                .spacing(15)
                .align_items(Alignment::Center),
                tracker_list
                    .padding(5)
                    .style(style::Container::Black)
                    .width(Length::Fill),
                bottom_button
            ]
            .spacing(5),
        )
        .height(Length::Fill)
        .into()
    }

    pub fn view_current_tracker(&self) -> Element<Message, Renderer<Theme>> {
        let title: _ = text("Current Tracker Infomation").font(JETBRAINS_MONO);
        let title_2: _ = text("None selected").font(JETBRAINS_MONO);

        let content: _ = match &self.current {
            Some(info) => {
                let name = &info.module_name;
                let format = &info.format;
                let samples = &info.samples;
                let total = &info.total_sample_size;

                container(
                    scrollable(
                        column![
                            text(format!("Module Name: {}", name)),
                            text(format!("Format: {}", format)),
                            text(format!("Samples: {}", samples)),
                            text(format!("Total Sample Size: {} KiB", total)),
                        ]
                        .spacing(5),
                    )
                    .style(style::scrollable::Scrollable::Dark),
                )
            }
            None => container(title_2),
        };
        container(
            column![
                title,
                content
                    .style(style::Container::Frame)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(8)
            ]
            .spacing(10),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

async fn tracker_info(path: PathBuf) -> Option<Info> {
    let Some((Ok(tracker), path)) = tokio::task::spawn_blocking(move || (load_module(&path), path)).await.ok() else {
        return None;
    };

    Some(Info::read(tracker, path))
}
