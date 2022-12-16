use crate::core::cfg::SampleRippingConfig;
use crate::core::xmodits::DownloadMessage;
use crate::gui::style::{self, Theme};
use crate::gui::{icons, JETBRAINS_MONO};
use iced::widget::{container, button, checkbox, column, row, scrollable, text, progress_bar};
use iced::widget::{Row, Space};
use iced::{Element, Length, Renderer};
use iced::{Alignment, Command};
use tokio::sync::mpsc::Sender;
use tracing::{warn, info};
use std::path::{Path, PathBuf};
// use tracing::{info, warn};
use xmodits_lib::{load_from_ext, load_module, TrackerModule};

#[derive(Debug, Clone)]
pub enum Message {
    Add(Option<Vec<PathBuf>>),
    Probe(usize),
    Clear,
    Select((usize, bool)),
    SelectAll(bool),
    DeleteSelected,
    TrackerInfo(Option<Box<Info>>),
    AddFileDialog,
    AddFolderDialog,
    SubscriptionMessage(DownloadMessage),
}

#[derive(Debug, Clone)]
pub enum Info {
    Valid {
        module_name: String,
        format: String,
        samples: usize,
        path: PathBuf,
        total_sample_size: usize,
    },
    Invalid {
        error: String,
        path: PathBuf,
    },
}

impl Info {
    pub fn valid(tracker: TrackerModule, path: PathBuf) -> Self {
        Self::Valid {
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

    pub fn invalid(error: String, path: PathBuf) -> Self {
        Self::Invalid { error, path }
    }
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Valid { .. } => true,
            Self::Invalid { .. } => false,
        }
    }

    pub fn path(&self) -> &Path {
        match self {
            Info::Valid { path, .. } | Info::Invalid { path, .. } => path,
        }
    }

    pub fn filename(&self) -> String {
        filename(self.path())
    }
}

pub struct Entry {
    path: PathBuf,
    filename: String,
    selected: bool,
}

impl Entry {
    pub fn new(path: PathBuf) -> Self {
        Self {
            filename: filename(&path),
            path,
            selected: false,
        }
    }
    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }
}
#[derive(Default, PartialEq, Eq)]
pub enum State {
    #[default]
    None,
    Ripping,
    Done
}

#[derive(Default)]
pub struct Trackers {
    pub paths: Vec<Entry>,
    pub current: Option<Box<Info>>,
    pub all_selected: bool,
    pub hint: Option<String>,
    pub sender: Option<Sender<(Vec<PathBuf>, SampleRippingConfig)>>,
    pub progress: f32,
    pub state: State
}

impl Trackers {
    pub fn add(&mut self, path: PathBuf) {
        if self.state != State::Ripping {
            if !self.paths.iter().map(|e| &e.path).any(|x| x == &path) {
                self.paths.push(Entry::new(path));
            }        
            if self.state == State::Done {
                self.state = State::None
            }
        }
    }
    pub fn set_hint(&mut self, hint: Option<String>) {
        self.hint = hint;
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
            Message::Probe(idx) => {
                let path = &self.paths[idx].path;
                if path.is_file() {
                    let command = Command::perform(
                        tracker_info(path.to_owned(), self.hint.to_owned()),
                        Message::TrackerInfo,
                    );
                    match self.current {
                        Some(ref e) if !e.is_valid() || e.path() != path => {
                            return command;
                        }
                        None => return command,
                        _ => (),
                    }
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
                return Command::perform(files_dialog(), Message::Add);
            }
            Message::AddFolderDialog => {
                return Command::perform(folders_dialog(), Message::Add);
            }
            Message::Add(path) => {
                if let Some(paths) = path {
                    paths.into_iter().for_each(|path| self.add(path));
                }
            }
            Message::SubscriptionMessage(msg) =>  match msg {
                DownloadMessage::Ready(tx) => self.sender = Some(tx),
                DownloadMessage::Done => {
                    self.state = State::Done;
                    // success();
                    info!("Done!"); // notify when finished ripping
                }
                DownloadMessage::Progress { progress, result } => {
                    info!("{}", progress);
                    self.progress = progress;
                    if let Err((path, e)) = result {
                        warn!("{} <-- {}", &path.display(), e);
                        // self.audio.play("sfx_2")
                    }
                } // useful for progress bars
                _ => (),
            },
        }
        Command::none()
    }

    pub fn start_rip(&mut self, cfg: &SampleRippingConfig) {
        let total_modules = self.total_modules() > 0;
        if let Some(tx) = &mut self.sender {
            if total_modules {
                let _ = tx.try_send((
                    {
                        self.current = None;
                        std::mem::take(&mut self.paths)
                            .into_iter()
                            .map(|f| f.path)
                            .collect()
                    },
                    cfg.to_owned()
                ));
                self.state = State::Ripping;
                // self.audio.play("sfx_1")
            }
        }
    }

    pub fn total_modules(&self) -> usize {
        self.paths.len()
    }

    // pub fn move_paths(&mut self) -> Vec<PathBuf> {
    //     self.current = None;
    //     self.paths.drain(..).into_iter().map(|f| f.path).collect()
    // }

    pub fn current_exists(&self, path: &Path) -> bool {
        matches!(&self.current, Some(info) if info.path() == path)
    }

    pub fn total_selected(&self) -> usize {
        self.paths.iter().filter(|f| f.selected).count()
    }

    pub fn view_trackers(&self) -> Element<Message, Renderer<Theme>> {
        let total_modules: _ =
            text(format!("Modules: {}", self.total_modules())).font(JETBRAINS_MONO);
        let total_selected: _ =
            text(format!("Selected: {}", self.total_selected())).font(JETBRAINS_MONO);

        let tracker_list: _ = match self.state {
            State::None => {
                if self.paths.is_empty() {
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
                                button(if gs.is_dir() {
                                    row![
                                        checkbox("", gs.selected, move |b| Message::Select((idx, b))),
                                        text(&gs.filename),
                                        Space::with_width(Length::Fill),
                                        icons::folder_icon()
                                    ]
                                    .spacing(1)
                                    .align_items(Alignment::Center)
                                } else {
                                    row![
                                        checkbox("", gs.selected, move |b| Message::Select((idx, b))),
                                        text(&gs.filename),
                                    ]
                                    .spacing(1)
                                    .align_items(Alignment::Center)
                                })
                                .width(Length::Fill)
                                .on_press(Message::Probe(idx))
                                .padding(4)
                                .style(style::button::Button::NormalPackage),
                                Space::with_width(Length::Units(15))
                            ])
                        },
                    )))
                    .height(Length::Fill)
                }
            },
            State::Ripping => {
                container(
                    column![
                        text("Ripping...").font(JETBRAINS_MONO),
                        progress_bar(0.0..=100.0, self.progress)
                            .height(Length::Units(5))
                            .width(Length::Units(200))

                    ]
                    .spacing(5)
                    .align_items(Alignment::Center)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
            },
            State::Done => {
                container(text("Done! Drag and drop.").font(JETBRAINS_MONO))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
            },
        };

        container(
            column![
                row![
                    total_modules,
                    total_selected,
                    Space::with_width(Length::Fill),
                    // checkbox is 5 units taller than the other elements
                    checkbox("Select all", self.all_selected, Message::SelectAll)
                        .style(style::checkbox::CheckBox::PackageDisabled),
                ]
                .spacing(15)
                .align_items(Alignment::Center),
                tracker_list
                    .padding(5)
                    .style(style::Container::Black)
                    .width(Length::Fill),
                // Self::bottom_button()
            ]
            .spacing(10),
        )
        .height(Length::Fill)
        .into()
    }

    pub fn bottom_button() -> Element<'static, Message, Renderer<Theme>> {
        row![
            button(
                //row![
                icons::add_file_icon(),
                // text("Add")
                //]
            )
            .padding(10)
            .on_press(Message::AddFileDialog),
            button("Add Folder")
                .padding(10)
                .on_press(Message::AddFolderDialog),
            Space::with_width(Length::Fill),
            button("Delete Selected")
                .padding(10)
                .on_press(Message::DeleteSelected),
            button("Clear").padding(10).on_press(Message::Clear),
        ]
        .spacing(10)
        .into()
    }

    pub fn view_current_tracker(&self) -> Element<Message, Renderer<Theme>> {
        let title: _ = text("Current Tracker Information").font(JETBRAINS_MONO);
        let title_2: _ = text("None selected").font(JETBRAINS_MONO);

        let content: _ = match &self.current {
            Some(info) => match &**info {
                Info::Valid {
                    module_name,
                    format,
                    samples,
                    total_sample_size,
                    ..
                } => container(
                    column![
                        text(format!("Module Name: {}", module_name)),
                        text(format!("Format: {}", format)),
                        text(format!("Samples: {}", samples)),
                        text(format!("Total Sample Size: {} KiB", total_sample_size)),
                    ]
                    .spacing(5)
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
                ),
                Info::Invalid { error, .. } => container(
                    column![
                        text(format!("Failed to load \"{}\"", info.filename())),
                        // .style(style::text::Text::Danger),
                        text(error)
                    ]
                    .spacing(5)
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
                ),
            },
            None => container(title_2),
        };
        container(
            column![
                title,
                content
                    .style(style::Container::Frame)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .padding(8)
                    .center_x()
                    .center_y()
            ]
            .spacing(15),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

pub fn filename(path: &Path) -> String {
    path.file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_default()
}

async fn tracker_info(path: PathBuf, hint: Option<String>) -> Option<Box<Info>> {
    let (tracker_result, path) = tokio::task::spawn_blocking(move ||
        (match hint {
            Some(hint) => load_from_ext(&path, &hint),
            None => load_module(&path)
        }, path)
    ).await.ok()?;
    match tracker_result {
        Ok(tracker) => Some(Box::new(Info::valid(tracker, path))),
        Err(error) => Some(Box::new(Info::invalid(error.to_string(), path))),
    }
}

fn paths(h: Option<Vec<rfd::FileHandle>>) -> Option<Vec<PathBuf>> {
    h.map(|filehandles| {
        filehandles
            .into_iter()
            .map(|d| d.path().to_owned())
            .collect()
    })
}

pub async fn folders_dialog() -> Option<Vec<PathBuf>> {
    paths(rfd::AsyncFileDialog::new().pick_folders().await)
}

pub async fn files_dialog() -> Option<Vec<PathBuf>> {
    paths(rfd::AsyncFileDialog::new().pick_files().await)
}
