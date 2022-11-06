use iced::Alignment;
use iced::{Element, Renderer, widget::container, Length};
use iced::widget::{text, pick_list,checkbox,column, row, scrollable, button};
use iced::widget::Space;
use iced_native::Widget;
use tracing::info;
use crate::{gui::style::{self, Theme}, core::cfg::Config};
use crate::gui::JETBRAINS_MONO;
use std::path::{PathBuf, Path};
use xmodits_lib::{load_module, TrackerModule};

#[derive(Debug, Clone)]
pub enum Message {
    Add(PathBuf),
    Remove(usize),
    Probe(usize),
    Beep(String),
    Clear,
    Select((usize, bool)),
}

#[derive(Default)]
pub struct Info {
    // id: usize,
    module_name: String,
    format: String,
    samples: usize,
    path: PathBuf,
    // total_sample_size: usize,
    // comments: Option<String>,
}
impl Info {
    fn read(tracker: TrackerModule, path: PathBuf) -> Self {
        Self {
            module_name: tracker.module_name().to_owned(),
            format: tracker.format().to_owned(),
            samples: tracker.number_of_samples(),
            path,
            // total_sample_size: todo,
            // comments: todo!(),
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
pub struct Xmodits {
    paths: Vec<File>,
    current: Option<Info>,
}

impl Xmodits {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Add(path) => {
                if !self.paths
                    .iter()
                    .map(|e| &e.path)
                    .collect::<Vec<&PathBuf>>()
                    .contains(&&path) 
                {
                    self.paths.push(File::new(path));
                }
            },
            Message::Remove(idx) => {
                if idx < self.paths.len() {
                    // self.paths.swap_remove(idx); // faster but not user friendly
                    self.paths.remove(idx);
                }
            },
            Message::Probe(idx) => {
                let path = &self.paths[idx].path;

                match &self.current {
                    Some(d) if d.path == *path => (), // ignore if Info has the same path
                    _ => {
                        if let Ok(tracker) = load_module(path) {
                            self.current = Some(Info::read(tracker, path.to_owned()));
                        }
                    },
                }
            },
            Message::Beep(_) => (),
            Message::Clear => {
                self.paths.clear();
                self.current = None;
            },
            Message::Select((idx, toggle)) => {
                self.paths[idx].selected = toggle
            },
        }
    }

    pub fn total_modules(&self) -> usize {
        self.paths.len()
    }

    pub fn total_selected(&self) -> usize {
        self.paths
            .iter()
            .filter(|f| f.selected == true)
            .count()
    }

    pub fn view_trackers(&self) -> Element<Message, Renderer<Theme>> {
        let total_modules: _ =  text(format!("Modules: {}", self.total_modules())).font(JETBRAINS_MONO);
        let total_selected: _ = text(format!("Selected: {}", self.total_selected())).font(JETBRAINS_MONO);

        let tracker_list: _ = if self.paths.is_empty() {
            container(text("Drag and drop").font(JETBRAINS_MONO))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
        } else {
            container(scrollable(
                self
                .paths
                .iter()
                .enumerate()
                .fold(
                    column![].spacing(10).padding(5),
                    |s, (idx, gs)| s.push(row![
                        
                        button(
                            row![
                                checkbox("", gs.selected, move |b| Message::Select((idx, b))),
                                text(&gs.filename),
                                ].spacing(1).align_items(Alignment::Center)
                            )
                            .style(style::button::Button::NormalPackage)
                            .width(Length::Fill)
                            .on_press(Message::Probe(idx))
                            .padding(4),

                        Space::with_width(Length::Units(15))
                    ])
                ))
            )
        };
        container(
            column![
            row![
                total_modules, total_selected,
                Space::with_width(Length::Fill),
            ].spacing(15),
            tracker_list
                .padding(5)
                .style(style::Container::Black)
                .width(Length::Fill)
        ].spacing(5))
        .height(Length::Fill)
        .into()       
    }

    pub fn view_current_tracker(&self) -> Element<Message, Renderer<Theme>> {
        let title: _ = text("Current Tracker Infomation:").font(JETBRAINS_MONO);
        let title_2: _ = text("None selected").font(JETBRAINS_MONO);

        let content: _ = match &self.current {
            Some(info) => {
                let name = &info.module_name;
                let format = &info.format;
                let samples = &info.samples;
                // let total = &info.total_sample_size;

                container(
                scrollable(
                    column![
                        text(format!("Module Name: {}", name)),
                        text(format!("Format: {}", format)),
                        text(format!("Samples: {}", samples)),
                        // text("Approx Total Sample Size (KiB): 1532"),
                        // text("Comments: \n"),
                    ]
                    .spacing(5)
                    
                )
                .style(style::scrollable::Scrollable::Dark)
            )
            },
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

            ].spacing(5)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
