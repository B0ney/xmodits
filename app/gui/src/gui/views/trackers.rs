use crate::core::cfg::SampleRippingConfig;
use crate::core::log::async_write_error_log;
use crate::core::xmodits::{DownloadMessage, StartSignal};
use crate::gui::style::{self, Theme};
use crate::gui::{icons, JETBRAINS_MONO};
use iced::widget::{
    button, checkbox, column, container, progress_bar, row, scrollable, text, Space,
};
use iced::{alignment::Horizontal, Alignment, Command, Element, Length, Renderer};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc::Sender;
use tracing::{info, warn};
use xmodits_common::filename;
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
    SetState(State),
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

struct Entry {
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

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub enum State {
    #[default]
    None,
    Ripping(Option<String>),
    Done,
    DoneWithErrors(Vec<(PathBuf, String)>),
    DoneWithTooMuchErrors(PathBuf),
    DoneWithTooMuchErrorsNoLog(String),
}

#[derive(Default)]
pub struct Trackers {
    paths: Vec<Entry>,
    current: Option<Box<Info>>,
    all_selected: bool,
    hint: Option<String>,
    sender: Option<Sender<StartSignal>>,
    progress: f32,
    state: State,
    errors: Vec<(PathBuf, String)>,
    log_path: PathBuf,
}

impl Trackers {
    pub fn add(&mut self, path: PathBuf) {
        if matches!(self.state, State::Ripping(_)) {
            return;
        }
        if !self.paths.iter().any(|x| x.path == path) {
            self.paths.push(Entry::new(path));
        }
        if self.state != State::None {
            self.state = State::None
        }
    }
    pub fn set_hint(&mut self, hint: Option<String>) {
        self.hint = hint;
    }
    pub fn with_hint(mut self, hint: Option<String>) -> Self {
        self.set_hint(hint);
        self
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
    pub fn total_modules(&self) -> usize {
        self.paths.len()
    }
    pub fn current_exists(&self, path: &Path) -> bool {
        matches!(&self.current, Some(info) if info.path() == path)
    }
    pub fn total_selected(&self) -> usize {
        self.paths.iter().filter(|f| f.selected).count()
    }
    pub fn start_rip(&mut self, cfg: &SampleRippingConfig) {
        let total_modules = self.total_modules() > 0;
        if let Some(tx) = &mut self.sender {
            if total_modules {
                let _ = tx.try_send((
                    {
                        self.log_path = cfg.destination.to_owned();
                        self.progress = 0.0;
                        self.current = None;
                        std::mem::take(&mut self.paths)
                            .into_iter()
                            .map(|f| f.path)
                            .collect()
                    },
                    cfg.to_owned(),
                ));
                self.state = State::Ripping(None);
            }
        }
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
                        Some(ref info) if !info.is_valid() || info.path() != path => {
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

                if !matches!(self.state, State::None | State::Ripping(_)) {
                    self.state = State::None;
                }
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
            Message::SubscriptionMessage(msg) => match msg {
                DownloadMessage::Ready(tx) => self.sender = Some(tx),
                DownloadMessage::Info(info) => self.state = State::Ripping(info),
                DownloadMessage::Done => {
                    let errors = std::mem::take(&mut self.errors);
                    let log_path = self.log_path.to_owned();

                    return Command::perform(
                        async move {
                            match errors.len() {
                                0 => State::Done,
                                1..=150 => State::DoneWithErrors(errors),
                                // If there's too many errors, don't display them, put them in a file.
                                _ => match async_write_error_log(log_path, errors).await {
                                    Ok(log_path) => State::DoneWithTooMuchErrors(log_path),
                                    Err(e) => State::DoneWithTooMuchErrorsNoLog(e.to_string()),
                                },
                            }
                        },
                        Message::SetState,
                    );

                    info!("Done!"); // notify when finished ripping
                }
                DownloadMessage::Progress { progress, result } => {
                    info!("{}", progress);
                    self.progress = progress;
                    if let Err((path, e)) = result {
                        warn!("{} <-- {}", &path.display(), &e);
                        self.errors.push((path, e));
                    }
                }
            },
            Message::SetState(state) => self.state = state,
        }
        Command::none()
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
                                        checkbox("", gs.selected, move |b| Message::Select((
                                            idx, b
                                        ))),
                                        text(&gs.filename),
                                        Space::with_width(Length::Fill),
                                        icons::folder_icon()
                                    ]
                                    .spacing(1)
                                    .align_items(Alignment::Center)
                                } else {
                                    row![
                                        checkbox("", gs.selected, move |b| Message::Select((
                                            idx, b
                                        ))),
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
            }
            State::Ripping(ref message) => container(
                column![
                    text(match message.as_ref() {
                        Some(info) => info,
                        None => "Ripping...",
                    })
                    .font(JETBRAINS_MONO),
                    progress_bar(0.0..=100.0, self.progress)
                        .height(Length::Units(5))
                        .width(Length::Units(200))
                ]
                .spacing(5)
                .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y(),
            State::Done => container(
                column![
                    text("Done! \\(^_^)/").font(JETBRAINS_MONO),
                    text("Drag and drop").font(JETBRAINS_MONO)
                ]
                .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y(),
            State::DoneWithErrors(ref errors) => container(column![
                column![
                    text("Done... But Xmodits could not rip everything... (._.)")
                        .font(JETBRAINS_MONO)
                        .horizontal_alignment(Horizontal::Center)
                ]
                .padding(4),
                scrollable(
                    errors
                        .iter()
                        .fold(column![].spacing(10).padding(5), |t, (s, x)| {
                            t.push(row![
                                container(
                                    column![
                                        text(filename(s)),
                                        text(x).horizontal_alignment(Horizontal::Center)
                                    ]
                                    .width(Length::Fill)
                                    .align_items(Alignment::Center)
                                )
                                .style(style::Container::Frame)
                                .width(Length::Fill)
                                .padding(4),
                                Space::with_width(Length::Units(15))
                            ])
                        })
                        .width(Length::Fill),
                ),
            ])
            .width(Length::Fill)
            .height(Length::Fill),
            State::DoneWithTooMuchErrors(ref error_log) => container(
                column![
                    text("Done...").font(JETBRAINS_MONO),
                    text("But there's too many errors to display! (-_-')").font(JETBRAINS_MONO),
                    text("Check the logs at:").font(JETBRAINS_MONO),
                    text(format!("{}", error_log.display()))
                        .font(JETBRAINS_MONO)
                        .horizontal_alignment(Horizontal::Center)
                ]
                .padding(4)
                .spacing(5),
            )
            .width(Length::Fill)
            .height(Length::Fill),
            State::DoneWithTooMuchErrorsNoLog(ref error) => container(
                column![
                    text("Done...").font(JETBRAINS_MONO),
                    text("But there's too many errors to display! (-_-')").font(JETBRAINS_MONO),
                    // TODO: maybe display the first 150 errors?
                    text("Unfortunatley, it's not possible to produce an error log:")
                        .font(JETBRAINS_MONO),
                    text(error)
                        .font(JETBRAINS_MONO)
                        .horizontal_alignment(Horizontal::Center)
                ]
                .padding(4)
                .spacing(5),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        };

        container(
            column![
                row![
                    total_modules,
                    total_selected,
                    Space::with_width(Length::Fill),
                    // checkbox is 5 units taller than the other elements
                    checkbox("Select all", self.all_selected, Message::SelectAll)
                        .style(style::checkbox::CheckBox::Disabled),
                ]
                .spacing(15)
                .align_items(Alignment::Center),
                tracker_list
                    .padding(5)
                    .style(style::Container::Black)
                    .width(Length::Fill),
            ]
            .spacing(10),
        )
        .height(Length::Fill)
        .into()
    }
    pub fn bottom_button() -> Element<'static, Message, Renderer<Theme>> {
        row![
            button(text("Add File"))
                .padding(10)
                .on_press(Message::AddFileDialog),
            button(text("Add Folder"))
                .padding(10)
                .on_press(Message::AddFolderDialog),
            Space::with_width(Length::Fill),
            button("Delete Selected")
                .padding(10)
                .on_press(Message::DeleteSelected),
                // .style(style::button::Button::Delete),
            button("Clear").padding(10).on_press(Message::Clear),
        ]
        .spacing(10)
        .into()
    }
    pub fn view_current_tracker(&self) -> Element<Message, Renderer<Theme>> {
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
                        text(format!("Failed to load \"{}\"", info.filename()))
                            .horizontal_alignment(Horizontal::Center),
                        // .style(style::text::Text::Danger),
                        text(error).horizontal_alignment(Horizontal::Center),
                    ]
                    .spacing(5)
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
                ),
            },
            None => container(text("None selected").font(JETBRAINS_MONO)),
        };
        container(
            column![
                text("Current Tracker Information").font(JETBRAINS_MONO),
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

async fn tracker_info(path: PathBuf, hint: Option<String>) -> Option<Box<Info>> {
    let (tracker_result, path) = tokio::task::spawn_blocking(move || {
        (
            match hint {
                Some(hint) => load_from_ext(&path, &hint),
                None => load_module(&path),
            },
            path,
        )
    })
    .await
    .ok()?;
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
