mod sample;

use std::path::PathBuf;

use audio_engine::PlayerHandle;
use iced::widget::{column, row, scrollable, slider, text, Space};
use iced::{Alignment, Command, Length};

use crate::screen::main_panel::Entries;
use crate::widget::{waveform_view::WaveformViewer, Element};
use crate::widget::{Button, Container, Row};
use crate::{icon, theme};

use sample::{SamplePack, SampleResult};

const MAX_VOLUME: f32 = 1.25;
const MIN_VOLUME: f32 = 0.0;

#[derive(Debug, Clone)]
pub enum Message {
    Select(usize),
    Play,
    Pause,
    Stop,
    SetPlayOnSelection(bool),
    SetVolume(f32),
    AddEntry(PathBuf),
    Loaded(Result<SamplePack, String>),
}

/// The state of the sample player
pub enum State {
    /// Nothing has been loaded
    None,
    /// Currently loading
    Loading,
    /// Could not load samples
    Failed { path: PathBuf, reason: String },
    /// Successfully loaded samples
    Loaded {
        path: PathBuf,
        module_name: String,
        selected: Option<usize>,
        samples: SamplePack,
    },
}

#[derive(Debug, Default)]
pub struct MediaSettings {
    pub volume: f32,
    pub play_on_selection: bool,
    pub enable_looping: bool,
}

/// Sample player instance
pub struct Instance {
    state: State,
    player: PlayerHandle,
    settings: MediaSettings,
    hovered: bool,
    progress: Option<f32>,
}

impl Instance {
    pub fn new(player: PlayerHandle, path: PathBuf) -> (Self, Command<Message>) {
        let mut instance = Self::new_empty(player);
        let command = instance.load_samples(path);

        (instance, command)
    }

    pub fn new_empty(player: PlayerHandle) -> Self {
        Self {
            state: State::None,
            player,
            settings: MediaSettings::default(),
            hovered: false,
            progress: None,
        }
    }

    pub fn settings(mut self, settings: MediaSettings) -> Self {
        self.settings = settings;
        self
    }

    pub fn update(&mut self, entries: &mut Entries, message: Message) -> Command<Message> {
        match message {
            Message::Select(index) => match &self.state {
                State::Loaded { selected, .. } => *selected = Some(index),
                _ => (),
            },
            Message::Play => todo!(),
            Message::Pause => todo!(),
            Message::Stop => todo!(),
            Message::SetPlayOnSelection(_) => todo!(),
            Message::AddEntry(_) => todo!(),
            Message::Loaded(_) => todo!(),
            Message::SetVolume(_) => todo!(),
        }
        Command::none()
    }

    pub fn load_samples(&mut self, module_path: PathBuf) -> Command<Message> {
        let mut load = |path: &PathBuf| {
            self.state = State::Loading;
            return todo!();
        };

        match &self.state {
            State::None => load(&module_path),
            State::Loading => Command::none(),
            State::Failed { path, .. } | State::Loaded { path, .. } => match path == &module_path {
                true => Command::none(),
                false => load(&module_path),
            },
        }
    }

    pub fn view(&self, entries: &Entries) -> Element<Message> {
        todo!()
    }

    pub fn title(&self) -> String {
        todo!()
    }

    /// top left quadrant
    fn view_sample_info(&self) -> Element<Message> {
        match &self.state {
            State::None => todo!(),
            State::Loading => todo!(),
            State::Failed { reason, .. } => todo!(),
            State::Loaded {
                selected, samples, ..
            } => match selected {
                Some(_) => todo!(),
                None => todo!(),
            },
        }
    }

    /// List out the samples
    fn view_samples(&self) -> Element<Message> {
        match self.state {
            State::None => todo!(),
            State::Loading => todo!(),
            State::Failed { path, reason } => todo!(),
            State::Loaded { samples, .. } => {
                let samples = samples
                    .inner()
                    .iter()
                    .enumerate()
                    .map(|(index, result)| result.view_sample(index))
                    .collect();

                let content = column(samples).spacing(10).padding(4);

                scrollable(content).into()
            }
        }
    }

    fn view_waveform(&self) -> Element<Message> {
        WaveformViewer::new_maybe(match &self.state {
            State::Loaded {
                path,
                module_name,
                selected,
                samples,
            } => selected.and_then(|index| samples.waveform(index)),
            _ => None,
        })
        .into()
    }

    fn media_buttons(&self) -> Element<Message> {
        let media_controls = media_button([
            (icon::play().size(18), Message::Play),
            (icon::stop().size(18), Message::Stop),
            (icon::pause().size(18), Message::Pause),
            // (icon::repeat().size(18), Message::Stop),
        ]);

        let volume_slider = column![
            text(format!("Volume: {}%", (self.settings.volume * 100.0).round())),
            slider(MIN_VOLUME..=MAX_VOLUME, self.settings.volume, Message::SetVolume).step(0.01)
        ]
        .align_items(Alignment::Start);

        Container::new(row![media_controls, volume_slider].spacing(8))
            .padding(8)
            .style(theme::Container::Black)
            .width(Length::Fill)
            .height(Length::Shrink)
            .center_x()
            .into()
    }
}


fn media_button<'a, Label, R, Message>(rows: R) -> Element<'a, Message>
where
    Message: Clone + 'a,
    Label: Into<Element<'a, Message>>,
    R: IntoIterator<Item = (Label, Message)>,
{
    let mut media_row: Row<'a, Message> = Row::new().spacing(4.0);
    let elements: Vec<(Label, Message)> = rows.into_iter().collect();
    let end_indx = elements.len() - 1;

    for (idx, (label, message)) in elements.into_iter().enumerate() {
        let style = if idx == 0 {
            theme::Button::MediaStart
        } else if idx == end_indx {
            theme::Button::MediaEnd
        } else {
            theme::Button::MediaMiddle
        };
        let button = Button::new(label).padding(8.0).on_press(message).style(style);
        media_row = media_row.push(button);
    }

    media_row.into()
}
