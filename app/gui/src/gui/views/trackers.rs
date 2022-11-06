use iced::{Element, Renderer, widget::container, Length};
use iced::widget::{text, pick_list,checkbox,column, row, scrollable, button};
use iced::widget::Space;
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
}

#[derive(Default)]
pub struct Info {
    id: usize,
    module_name: String,
    format: String,
    samples: usize,
    // total_sample_size: usize,
    // comments: Option<String>,
}
impl Info {
    fn read(tracker: TrackerModule, id: usize) -> Self {
        Self {
            module_name: tracker.module_name().to_owned(),
            format: tracker.format().to_owned(),
            samples: tracker.number_of_samples(),
            id,
            // total_sample_size: todo,
            // comments: todo!(),
        }
    }
}
struct File {
    path: PathBuf,
    filename: String,
}
impl File {
    pub fn new(path: PathBuf) -> Self {
        Self {
            filename: path.file_name().unwrap().to_string_lossy().to_string(),
            path,
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
                // let path = format!("{}", path.display());
                if !self.paths.iter().map(|e| &e.path).collect::<Vec<&PathBuf>>().contains(&&path) {
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
                let a = &self.current;
                let path = &self.paths[idx].path;

                // let mut probe = || {
                //     if let Ok(tracker) = load_module(path) {
                //         self.current = Some(Info::read(tracker, idx));
                //     };
                // };

                match a {
                    Some(d) => {
                        if d.id != idx {
                            if let Ok(tracker) = load_module(path) {
                                self.current = Some(Info::read(tracker, idx));
                            };
                        } else {
                            info!("nope!")
                        }
                    },
                    None => {
                        if let Ok(tracker) = load_module(path) {
                            self.current = Some(Info::read(tracker, idx));
                        };
                    },
                }

                // match &self.current {
                //     Some( a @ Info { id: idx, ..}) => info!("{}",format!("{} is loaded", idx)),
                //     Some(a) => info!("boring!"),
                //     _ => {
                        
                //     }
                // }  
            },
            Message::Beep(_) => (),
            Message::Clear => {
                self.paths.clear();
                self.current = None;
            },
        }
    }
    pub fn total_modules(&self) -> usize {
        self.paths.len()
    }

    pub fn view_trackers(&self) -> Element<Message, Renderer<Theme>> {
        let container: _ = if self.paths.len() == 0 {
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
                        button(text(&gs.filename))
                            .style(style::button::Button::NormalPackage)
                            .width(Length::Fill)
                            .on_press(Message::Probe(idx)),

                        Space::with_width(Length::Units(15))
                    ])
                ))
            ).height(Length::Fill)
                
        };
        container.into()
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
        // .padding(8)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
        
    

        // container(stats).into()
    }

}
