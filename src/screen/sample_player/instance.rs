mod sample;

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use audio_engine::{PlayerHandle, TrackerSample};
use iced::widget::{button, checkbox, column, progress_bar, row, scrollable, slider, text, Space};
use iced::{command, Alignment, Command, Length};

use crate::screen::entry::Entries;
use crate::widget::helpers::{centered_container, fill_container, warning};
use crate::widget::waveform_view::{Marker, WaveData};
use crate::widget::{Button, Collection, Container, Element, Row, WaveformViewer};
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
    Loaded(Result<SamplePack, (PathBuf, String)>),
    Progress(Option<f32>),
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
        selected: Option<usize>,
        samples: SamplePack,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct MediaSettings {
    pub volume: f32,
    pub play_on_selection: bool,
    pub enable_looping: bool,
}

impl Default for MediaSettings {
    fn default() -> Self {
        Self {
            volume: 1.0,
            play_on_selection: true,
            enable_looping: false,
        }
    }
}

/// Sample player instance
pub struct Instance {
    state: State,
    player: PlayerHandle,
    settings: MediaSettings,
    pub hovered: bool,
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

    pub fn update(&mut self, message: Message, entries: &mut Entries) -> Command<Message> {
        match message {
            Message::Select(index) => {
                if let State::Loaded { selected, .. } = &mut self.state {
                    *selected = Some(index);

                    match self.settings.play_on_selection {
                        true => return self.play_selected(),
                        false => self.player.stop(),
                    }
                }
            }
            Message::Play => return self.play_selected(),
            Message::Pause => self.player.pause(),
            Message::Stop => self.player.stop(),
            Message::SetPlayOnSelection(toggle) => self.settings.play_on_selection = toggle,
            Message::AddEntry(path) => entries.add(path),
            Message::Loaded(result) => {
                self.state = match result {
                    Ok(samples) => State::Loaded {
                        selected: None,
                        samples,
                    },
                    Err((path, reason)) => State::Failed { path, reason },
                }
            }
            Message::SetVolume(volume) => {
                self.player.set_volume(volume);
                self.settings.volume = volume;
            }
            Message::Progress(p) => self.progress = p,
        }
        Command::none()
    }

    pub fn view(&self, entries: &Entries) -> Element<Message> {
        let info = fill_container(self.view_sample_info())
            .padding(8)
            .style(theme::Container::Black);

        let top_left = column![info, self.media_buttons()]
            .spacing(5)
            .width(Length::Fill);

        let top_right_controls = {
            let add_path_button = self.loaded_path().and_then(|path| {
                let button =
                    || button("Add to Entries").on_press(Message::AddEntry(path.to_owned()));
                (!entries.contains(path)).then(button)
            });

            let no_button_spacing = add_path_button
                .is_none()
                .then_some(Space::with_height(Length::Fixed(27.0)));

            let play_on_selection_checkbox = checkbox(
                "Play on Selection",
                self.settings.play_on_selection,
                Message::SetPlayOnSelection,
            )
            .style(theme::CheckBox::Inverted);

            row![play_on_selection_checkbox, Space::with_width(Length::Fill)]
                .push_maybe(no_button_spacing)
                .push_maybe(add_path_button)
                .spacing(5)
                .align_items(Alignment::Center)
        };

        let sample_list = fill_container(self.view_samples())
            .padding(8)
            .style(theme::Container::Black);

        let top_right = column![sample_list, top_right_controls]
            .spacing(5)
            .width(Length::Fill);

        let waveform_viewer = self
            .view_waveform()
            .marker_maybe(self.progress.map(Marker))
            .width(Length::Fill)
            .height(Length::FillPortion(2));

        let progress = progress_bar(0.0..=1.0, self.progress.unwrap_or_default())
            .height(5.0)
            .style(theme::ProgressBar::Dark);

        let warning = warning(
            || false,
            "WARNING - This sample is most likely static noise.",
        );

        let top_half = row![top_left, top_right]
            .height(Length::FillPortion(3))
            .spacing(5);

        let main = column![top_half, waveform_viewer, progress]
            .push_maybe(warning)
            .spacing(5);

        fill_container(main)
            .style(theme::Container::Hovered(self.hovered))
            .padding(15)
            .into()
    }

    pub fn matches_path(&self, module_path: &Path) -> bool {
        match &self.state {
            State::Failed { path, .. } => path == module_path,
            State::Loaded { samples, .. } => samples.path() == module_path,
            _ => false,
        }
    }

    pub fn load_samples(&mut self, module_path: PathBuf) -> Command<Message> {
        let load = |state: &mut State, path: PathBuf| {
            *state = State::Loading;
            self.player.stop();
            load_samples(path)
        };

        match &self.state {
            State::None => load(&mut self.state, module_path),
            State::Loading => Command::none(),
            _ => match self.matches_path(&module_path) {
                true => Command::none(),
                false => load(&mut self.state, module_path),
            },
        }
    }

    pub fn title(&self) -> String {
        match &self.state {
            State::None => "No samples loaded!".into(),
            State::Loading => "Loading...".into(),
            State::Failed { path, .. } => {
                format!("Failed to open {}", path.display())
            }
            State::Loaded { samples, .. } => {
                format!("Loaded: \"{}\"", samples.name())
            }
        }
    }

    pub fn play_selected(&self) -> Command<Message> {
        match &self.state {
            State::Loaded {
                selected, samples, ..
            } => match selected.and_then(|index| samples.tracker_sample(index)) {
                Some(sample) => play_sample(&self.player, sample),
                None => Command::none(),
            },
            _ => Command::none(),
        }
    }

    pub fn loaded_path(&self) -> Option<&Path> {
        match &self.state {
            State::Loaded { samples, .. } => Some(samples.path()),
            _ => None,
        }
    }

    /// top left quadrant
    fn view_sample_info(&self) -> Element<Message> {
        match &self.state {
            State::None => centered_container("Nothing Loaded...").into(),
            State::Loading => centered_container("Loading...").into(),
            State::Failed { reason, .. } => centered_container(text(reason)).into(),
            State::Loaded {
                selected, samples, ..
            } => match selected {
                Some(index) => samples.view_sample_info(*index),
                None => centered_container("Nothing selected...").into(),
            },
        }
    }

    /// List out the samples
    fn view_samples(&self) -> Element<Message> {
        match &self.state {
            State::None => centered_container("Drag and drop a module to preview").into(),
            State::Loading => centered_container("Loading...").into(),
            State::Failed { .. } => centered_container("ERROR").into(),
            State::Loaded { samples, .. } => match samples.is_empty() {
                true => centered_container("This module doesn't have any samples! o_0").into(),
                false => scrollable(
                    column(
                        samples
                            .inner()
                            .iter()
                            .enumerate()
                            .map(|(index, result)| result.view_sample(index)),
                    )
                    .spacing(10)
                    .padding(4),
                )
                .into(),
            },
        }
    }

    fn view_waveform(&self) -> WaveformViewer<Message> {
        WaveformViewer::new_maybe(match &self.state {
            State::Loaded {
                selected, samples, ..
            } => selected.and_then(|index| samples.waveform(index)),
            _ => None,
        })
    }

    fn media_buttons(&self) -> Element<Message> {
        let media_controls = media_button([
            (icon::play().size(18), Message::Play),
            (icon::stop().size(18), Message::Stop),
            (icon::pause().size(18), Message::Pause),
            // (icon::repeat().size(18), Message::Stop),
        ]);

        let volume = text(format!(
            "Volume: {}%",
            (self.settings.volume * 100.0).round()
        ));
        let volume_slider = column![volume]
            .push(
                slider(
                    MIN_VOLUME..=MAX_VOLUME,
                    self.settings.volume,
                    Message::SetVolume,
                )
                .step(0.01),
            )
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
        let button = Button::new(label)
            .padding(8.0)
            .on_press(message)
            .style(style);
        media_row = media_row.push(button);
    }

    media_row.into()
}

const PLAY_CURSOR_FPS: f32 = 60.0;

fn play_sample(handle: &PlayerHandle, source: TrackerSample) -> Command<Message> {
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<f32>();
    handle.stop();
    handle.play_with_callback(
        source,
        move |sample: &TrackerSample, duration: &mut Instant| {
            let fps_interval =
                Duration::from_millis(((1.0 / PLAY_CURSOR_FPS) * 1000.0).round() as u64);

            if duration.elapsed() > fps_interval {
                *duration = Instant::now();
                let progress = sample.frame() as f32 / sample.buf.frames() as f32;

                let _ = sender.send(progress);
            }
        },
    );

    command::channel(256, |mut s| async move {
        while let Some(new_progress) = receiver.recv().await {
            let _ = s.try_send(Message::Progress(Some(new_progress)));
        }
        let _ = s.try_send(Message::Progress(None));
    })
}

fn load_samples(path: PathBuf) -> Command<Message> {
    use crate::logger::log_file_on_panic;
    use xmodits_lib::Error;

    Command::perform(
        async {
            let path_copy = path.clone();

            let task = move || {
                log_file_on_panic(&path, |path| {
                    const MAX_SIZE: u64 = 40 * 1024 * 1024;

                    if path.is_dir() {
                        return Err(Error::io_error("Path is a directory.").unwrap_err());
                    }

                    let mut file = std::fs::File::open(&path)?;

                    if file.metadata()?.len() > MAX_SIZE {
                        return Err(Error::io_error("File size exceeds 40 MB").unwrap_err());
                    }

                    let module = xmodits_lib::load_module(&mut file)?;
                    let sample_pack = audio_engine::SamplePack::build(&*module);
                    let name = sample_pack.name;

                    let samples = sample_pack
                        .samples
                        .into_iter()
                        .map(|result| match result {
                            Ok((metadata, buffer)) => {
                                let peaks = buffer.buf.peaks(Duration::from_millis(5));
                                let waveform = WaveData::from(peaks);
                                SampleResult::Valid {
                                    metadata,
                                    buffer,
                                    waveform,
                                }
                            }
                            Err(error) => SampleResult::Invalid(error.to_string()),
                        })
                        .collect();

                    Ok(SamplePack::new(name, path.to_owned(), samples))
                })
            };

            // TODO
            match tokio::task::spawn_blocking(task).await {
                Ok(Ok(samples)) => Ok(samples),
                Ok(Err(e)) => Err((path_copy, e.to_string())),
                Err(e) => Err((path_copy, e.to_string())),
            }
        },
        Message::Loaded,
    )
}
