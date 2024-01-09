mod sample_info;
mod wave_cache;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use audio_engine::{PlayerHandle, Sample, SamplePack, TrackerSample};
use iced::widget::{
    button, checkbox, column, horizontal_rule, progress_bar, row, scrollable, slider, text, Space,
};
use iced::window::Id;
use iced::{command, Alignment, Command, Length};
use tokio::sync::mpsc;

use crate::screen::main_panel::Entries;
use crate::widget::helpers::{centered_container, fill_container, warning};
use crate::widget::waveform_view::{Marker, WaveData, WaveformViewer};
use crate::widget::{Button, Collection, Container, Element, Row};
use crate::{icon, theme};

use sample_info::SampleInfo;
use wave_cache::WaveCache;

const MAX_VOLUME: f32 = 1.25;
const MIN_VOLUME: f32 = 0.0;

#[derive(Debug, Clone)]
pub enum Message {
    Play,
    Pause,
    Stop,
    Volume(f32),
    Progress(Option<f32>),
    Loaded(Arc<Result<(SamplePack, WaveCache), xmodits_lib::Error>>),
    Load(PathBuf),
    Info((usize, SampleInfo)),
    SetPlayOnSelect(bool),
    AddEntry(PathBuf),
}

#[derive(Default)]
enum State {
    #[default]
    None,
    Loading,
    Failed {
        reason: String,
        path: PathBuf,
    },
    Loaded {
        // path: PathBuf,
        sample_pack: SamplePack,
        wave_data: WaveCache,
        selected: SampleInfo,
        play_progress: Option<f32>,
    },
}

impl State {
    fn wave_cache(&self) -> Option<&WaveData> {
        match &self {
            State::Loaded { selected, .. } => selected.waveform(),
            _ => None,
        }
    }
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

pub struct SamplePreviewWindow {
    state: State,
    player: PlayerHandle,
    settings: MediaSettings,
    pub hovered: bool,
}

impl SamplePreviewWindow {
    pub fn create(id: Id, player: PlayerHandle) -> Self {
        Self {
            player,
            hovered: false,
            state: State::None,
            settings: Default::default(),
        }
    }

    pub fn can_load(&self, new_path: &Path) -> bool {
        match &self.state {
            State::None => true,
            State::Loading => false,
            State::Failed { path, .. } => path != new_path,
            State::Loaded { sample_pack, .. } => !sample_pack.matches_path(new_path),
        }
    }

    pub fn play(&mut self) -> Command<Message> {
        self.player.stop();

        match self.get_selected() {
            Some(sample) => play_sample(&self.player, sample),
            None => Command::none(),
        }
    }

    pub fn update(&mut self, msg: Message, entries: &mut Entries) -> Command<Message> {
        match msg {
            Message::Play => return self.play(),
            Message::Pause => self.player.pause(),
            Message::Stop => self.player.stop(),
            Message::Volume(vol) => {
                self.settings.volume = vol.clamp(MIN_VOLUME, MAX_VOLUME);
                self.player.set_volume(self.settings.volume)
            }
            Message::Load(path) => {
                return match self.can_load(&path) {
                    true => {
                        self.state = State::Loading;
                        tracing::info!("loading");
                        load_sample_pack(path)
                    }
                    false => Command::none(),
                };
            }
            Message::Loaded(result) => match Arc::into_inner(result).unwrap() {
                Ok((sample_pack, wave_data)) => {
                    self.player.stop();
                    self.state = State::Loaded {
                        sample_pack,
                        wave_data,
                        selected: SampleInfo::None,
                        play_progress: None,
                    }
                }
                Err(err) => tracing::error!("{}", err),
            },
            Message::Info(smp) => {
                if let State::Loaded { selected, .. } = &mut self.state {
                    *selected = smp.1;

                    match self.settings.play_on_selection {
                        true => return self.play(),
                        false => self.player.stop(),
                    }
                };
            }
            Message::SetPlayOnSelect(play_on_select) => self.settings.play_on_selection = play_on_select,
            Message::Progress(progress) => {
                if let State::Loaded { play_progress, .. } = &mut self.state {
                    *play_progress = progress
                }
            }
            Message::AddEntry(path) => entries.add(path),
        }
        Command::none()
    }

    pub fn view(&self, entries: &Entries) -> Element<Message> {
        let media_controls = media_button([
            (icon::play().size(18), Message::Play),
            (icon::stop().size(18), Message::Stop),
            (icon::pause().size(18), Message::Pause),
            // (icon::repeat().size(18), Message::Stop),
        ]);

        let volume_slider = column![
            text(format!("Volume: {}%", (self.settings.volume * 100.0).round())),
            slider(MIN_VOLUME..=MAX_VOLUME, self.settings.volume, Message::Volume).step(0.01)
        ]
        .align_items(Alignment::Start);

        let control_panel = Container::new(row![media_controls, volume_slider].spacing(8))
            .padding(8)
            .style(theme::Container::Black)
            .width(Length::Fill)
            .height(Length::Shrink)
            .center_x();

        let top_left = column![
            fill_container(view_sample_info(self.get_selected_info()))
                .padding(8)
                .style(theme::Container::Black),
            control_panel
        ]
        .spacing(5)
        .width(Length::Fill);

        let sample_list = view_sample_list(&self.state);

        let add_path_button = self.path().and_then(|path| {
            let button = || button("Add to Entries").on_press(Message::AddEntry(path.to_owned()));
            (!entries.contains(path)).then(button)
        });

        let no_button_spacing = add_path_button
            .is_none()
            .then_some(Space::with_height(Length::Fixed(27.0)));

        let top_right = column![
            fill_container(sample_list)
                .padding(8)
                .style(theme::Container::Black),
            row![
                checkbox(
                    "Play on Selection",
                    self.settings.play_on_selection,
                    Message::SetPlayOnSelect
                )
                .style(theme::CheckBox::Inverted),
                Space::with_width(Length::Fill)
            ]
            .push_maybe(no_button_spacing)
            .push_maybe(add_path_button)
            .spacing(5)
            .align_items(Alignment::Center)
        ]
        .spacing(5)
        .width(Length::Fill);

        let waveform_viewer = WaveformViewer::new_maybe(self.state.wave_cache())
            .marker_maybe(self.progress().map(Marker))
            .width(Length::Fill)
            .height(Length::FillPortion(2));

        let warning = warning(|| false, "WARNING - This sample is most likely static noise.");

        let main = column![
            row![top_left, top_right]
                .height(Length::FillPortion(3))
                .spacing(5),
            waveform_viewer,
            progress_bar(0.0..=1.0, self.progress().unwrap_or_default())
                .height(5.0)
                .style(theme::ProgressBar::Dark)
        ]
        .push_maybe(warning)
        .spacing(5);

        fill_container(main)
            .style(theme::Container::Hovered(self.hovered))
            .padding(15)
            .into()
    }

    pub fn progress(&self) -> Option<f32> {
        match &self.state {
            State::Loaded { play_progress, .. } => *play_progress,
            _ => None,
        }
    }

    pub fn title(&self) -> String {
        match &self.state {
            State::None => "No samples loaded!".into(),
            State::Loading => "Loading...".into(),
            State::Failed { reason, path } => "Failed to open...".into(),
            State::Loaded { sample_pack, .. } => format!("Loaded: \"{}\"", sample_pack.name),
        }
    }

    pub fn matches_path(&self, path: &Path) -> bool {
        self.path().is_some_and(|s| s == path)
    }

    pub fn path(&self) -> Option<&Path> {
        match &self.state {
            State::Loaded { sample_pack, .. } => sample_pack.path.as_deref(),
            _ => None,
        }
    }

    pub fn load_sample_pack(&self, path: PathBuf) -> Command<Message> {
        match self.can_load(&path) {
            true => load_sample_pack(path),
            false => Command::none(),
        }
    }

    fn get_selected(&self) -> Option<TrackerSample> {
        let State::Loaded { selected, .. } = &self.state else {
            return None;
        };

        match selected {
            SampleInfo::Sample { data, .. } => Some(data.clone()),
            _ => None,
        }
    }

    fn get_selected_info(&self) -> Option<&SampleInfo> {
        match &self.state {
            State::Loaded { selected, .. } => Some(selected),
            _ => None,
        }
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

fn view_sample_list(state: &State) -> Element<Message> {
    match state {
        State::None => centered_container("Nothing Loaded").into(),
        State::Loading => centered_container("Loading...").into(),
        State::Failed { reason, path } => centered_container("").into(),
        State::Loaded { sample_pack, .. } => view_samples(&sample_pack.samples),
    }
}

const PLAY_CURSOR_FPS: f32 = 60.0;

fn play_sample(handle: &PlayerHandle, source: TrackerSample) -> Command<Message> {
    let (sender, mut receiver) = mpsc::unbounded_channel::<f32>();

    let callback = move |sample: &TrackerSample, duration: &mut Instant| {
        let fps_interval = Duration::from_millis(((1.0 / PLAY_CURSOR_FPS) * 1000.0).round() as u64);

        if duration.elapsed() > fps_interval {
            *duration = Instant::now();
            let progress = sample.frame() as f32 / sample.buf.frames() as f32;
            let _ = sender.send(progress);
        }
    };

    handle.play_with_callback(source, callback);

    command::channel(256, |mut s| async move {
        while let Some(new_progress) = receiver.recv().await {
            let _ = s.try_send(Message::Progress(Some(new_progress)));
        }
        let _ = s.try_send(Message::Progress(None));
    })
}

fn load_sample_pack(path: PathBuf) -> Command<Message> {
    Command::perform(
        async {
            tokio::task::spawn_blocking(move || {
                const MAX_SIZE: u64 = 40 * 1024 * 1024;

                let mut file = File::open(&path)?;

                if file.metadata()?.len() > MAX_SIZE {
                    return Err(xmodits_lib::Error::io_error("File size exceeds 40 MB").unwrap_err());
                }

                let module = xmodits_lib::load_module(&mut file)?;
                let sample_pack = SamplePack::build(&*module).with_path(path);
                let wave_cache = WaveCache::from_sample_pack(&sample_pack);

                Ok((sample_pack, wave_cache))
            })
            .await
            .map(Arc::new)
            .unwrap()
        },
        Message::Loaded,
    )
}

fn view_samples(a: &[Result<(Sample, TrackerSample), xmodits_lib::Error>]) -> Element<Message> {
    let samples = a.iter().enumerate().map(view_sample).collect();
    scrollable(column(samples).spacing(10).padding(4)).into()
}

fn view_sample(
    (index, result): (usize, &Result<(Sample, TrackerSample), xmodits_lib::Error>),
) -> Element<Message> {
    let info = SampleInfo::new(index, result);

    let error_icon = || {
        row![]
            .push(Space::with_width(Length::Fill))
            .push(icon::warning())
            .align_items(iced::Alignment::Center)
    };

    let title = row![]
        .push(text(match info.title() {
            title if title.is_empty() => format!("{}", index + 1),
            title => format!("{} - {}", index + 1, title),
        }))
        .push_maybe(info.is_error().then_some(error_icon()))
        .spacing(5);

    let theme = match info.is_error() {
        true => theme::Button::EntryError,
        false => theme::Button::Entry,
    };

    row![
        button(title)
            .width(Length::Fill)
            .style(theme)
            .on_press(Message::Info((index, info))),
        Space::with_width(15)
    ]
    .into()
}

fn view_sample_info(info: Option<&SampleInfo>) -> Element<Message> {
    match info {
        None => centered_container("Nothing selected...").into(),
        Some(info) => match info {
            SampleInfo::None => centered_container("Nothing selected...").into(),
            SampleInfo::Invalid { reason, .. } => centered_container(text(reason)).into(),
            SampleInfo::Sample { metadata, .. } => {
                let smp = metadata;

                let sample_name =
                    (!smp.name.trim().is_empty()).then_some(text(format!("Name: {}", smp.name.trim())));

                let sample_filename = smp
                    .filename
                    .as_ref()
                    .map(|s| s.trim())
                    .and_then(|s| (!s.is_empty()).then_some(text(format!("File Name: {}", s))));

                let metadata = text(format!(
                    "{} Hz, {}-bit ({}), {}",
                    smp.rate,
                    smp.bits(),
                    if smp.is_signed() { "Signed" } else { "Unsigned" },
                    if smp.is_stereo() { "Stereo" } else { "Mono" },
                ));

                let round_100th = |x: f32| (x * 100.0).round() / 100.0;

                let duration = Duration::from_micros(
                    ((smp.length_frames() as f64 / smp.rate as f64) * 1_000_000.0) as u64,
                );
                let duration_secs = round_100th(duration.as_secs_f32());
                let plural = if duration_secs == 1.0 { "" } else { "s" };
                let duration = text(format!("Duration: {} sec{plural}", duration_secs));

                let size = match smp.length {
                    l if l < 1000 => format!("{} bytes", l),
                    l if l < 1_000_000 => format!("{} KB", round_100th(l as f32 / 1000.0)),
                    l => format!("{} MB", round_100th(l as f32 / 1_000_000.0)),
                };

                let info = column![]
                    .push_maybe(sample_name)
                    .push_maybe(sample_filename)
                    .push(duration)
                    .push(text(format!("Size: {}", size)))
                    .push(text(format!("Loop type: {:#?}", smp.looping.kind())))
                    .push(text(format!("Internal Index: {}", smp.index_raw())))
                    .push(horizontal_rule(1))
                    .push(metadata)
                    .push(horizontal_rule(1))
                    .spacing(5)
                    .align_items(Alignment::Center);
                centered_container(info).into()
            }
        },
    }
}
