mod sample_info;
mod wave_cache;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use audio_engine::{PlayerHandle, Sample, SamplePack, TrackerSample};
use iced::alignment::Horizontal;
use iced::widget::scrollable::{Direction, Properties};
use iced::widget::{button, checkbox, column, horizontal_rule, row, scrollable, text, Space};
use iced::window::Id;
use iced::{command, Alignment, Command, Length};
use tokio::sync::mpsc::{self, Receiver, UnboundedReceiver};

use crate::widget::helpers::{centered_container, centered_text, fill_container, warning};
use crate::widget::waveform_view::{Marker, WaveData, WaveformViewer};
use crate::widget::{Button, Collection, Container, Element, Row};
use crate::{icon, theme};

use sample_info::SampleInfo;
use wave_cache::WaveCache;

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
}

enum State {
    Play,
    Paused,
}

pub struct SamplePreviewWindow {
    id: Id,
    state: State,
    player: PlayerHandle,
    sample_pack: Option<SamplePack>,
    selected: Option<(usize, SampleInfo)>,
    pub hovered: bool,
    play_on_select: bool,
    wave_cache: WaveCache,
    progress: Option<f32>,
}

impl SamplePreviewWindow {
    pub fn create(id: Id, player: PlayerHandle) -> Self {
        Self {
            player,
            id,
            hovered: false,
            sample_pack: None,
            state: State::Play,
            selected: None,
            play_on_select: true,
            wave_cache: WaveCache::default(),
            progress: None,
        }
    }

    pub fn play(&mut self) -> Command<Message> {
        self.player.stop();
        let Some(sample_pack) = &self.sample_pack else {
            return Command::none();
        };

        let Some((index, _)) = self.selected else {
            return Command::none();
        };

        match &sample_pack.samples[index] {
            Ok((_, sample)) => play_sample(&self.player, sample.clone()),
            _ => return Command::none(),
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::Play => return self.play(),
            Message::Pause => self.player.pause(),
            Message::Stop => self.player.stop(),
            Message::Volume(vol) => self.player.set_volume(vol),
            Message::Load(path) => {
                return match &self.sample_pack {
                    Some(f) if !f.matches_path(&path) => load_sample_pack(path),
                    _ => Command::none(),
                }
            }
            Message::Loaded(result) => match Arc::into_inner(result).unwrap() {
                Ok((sample_pack, wave_cache)) => {
                    self.selected = None;
                    self.sample_pack = Some(sample_pack);
                    self.wave_cache = wave_cache;
                }
                Err(err) => tracing::error!("{}", err),
            },
            Message::Info(smp) => {
                self.selected = Some(smp);

                if self.play_on_select {
                    return self.play();
                }
            }
            Message::SetPlayOnSelect(play_on_select) => self.play_on_select = play_on_select,
            Message::Progress(progress) => self.progress = progress,
        }
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let top_left = fill_container(view_sample_info(self.selected.as_ref().map(|(_, smp)| smp)))
            .padding(8)
            .style(theme::Container::BlackHovered(self.hovered));

        let controls = media_button([
            (icon::play().size(18), Message::Play),
            (icon::stop().size(18), Message::Stop),
            (icon::pause().size(18), Message::Pause),
            (icon::repeat().size(18), Message::Stop),
        ]);

        let control_panel = Container::new(controls)
            .padding(8)
            .style(theme::Container::BlackHovered(self.hovered))
            .width(Length::Fill)
            .height(Length::Shrink)
            .center_x();

        let sample_list = match &self.sample_pack {
            Some(pack) => view_samples(&pack.samples),
            None => centered_container("Loading...").into(),
        };

        let top_left = column![top_left, control_panel].spacing(5).width(Length::Fill);

        let play_on_select = checkbox("Play on Selection", self.play_on_select, Message::SetPlayOnSelect);

        let top_right = fill_container(sample_list)
            .padding(8)
            .style(theme::Container::BlackHovered(self.hovered));

        let top_right = column![top_right, play_on_select].spacing(5).width(Length::Fill);

        let waveform_viewer = WaveformViewer::new_maybe(self.wave_cache())
            .marker_maybe(self.progress.map(Marker))
            .style(theme::WaveformView::Hovered(self.hovered));

        let bottom = fill_container(waveform_viewer).height(Length::FillPortion(2));

        let warning = warning(|| false, "WARNING - This sample is most likely static noise.");

        let main = column![
            row![top_left, top_right]
                .height(Length::FillPortion(3))
                .spacing(5),
            bottom
        ]
        .push_maybe(warning)
        .spacing(5);

        fill_container(main).padding(15).into()
    }

    pub fn title(&self) -> String {
        match &self.sample_pack {
            Some(pack) => format!("Loaded: \"{}\"", pack.name),
            None => "No samples loaded!".into(),
        }
    }

    pub fn matches_path(&self, path: &Path) -> bool {
        self.sample_pack
            .as_ref()
            .map(|s| s.path.as_ref())
            .flatten()
            .is_some_and(|s| s == path)
    }

    pub fn load_sample_pack(&self, path: PathBuf) -> Command<Message> {
        match self.matches_path(&path) {
            true => Command::none(),
            false => load_sample_pack(path),
        }
    }

    fn wave_cache(&self) -> Option<&WaveData> {
        self.selected
            .as_ref()
            .map(|(idx, _)| self.wave_cache.cache.get(&idx))
            .flatten()
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

const PLAY_CURSOR_FPS: f32 = 60.0;

fn play_sample(handle: &PlayerHandle, source: TrackerSample) -> Command<Message> {
    let (sender, mut receiver) = mpsc::unbounded_channel::<f32>();

    let callback = move |sample: &TrackerSample, duration: &mut Instant| {
        let fps_interval = Duration::from_millis(((1.0 / PLAY_CURSOR_FPS) * 1000.0).round() as u64);

        if duration.elapsed().as_millis() > fps_interval.as_millis() {
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
                    return Err(xmodits_lib::Error::io_error("File size is exceeds 40 MB").unwrap_err());
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
    scrollable(
        column(a.into_iter().enumerate().map(view_sample).collect())
            .spacing(10)
            .padding(4),
    )
    .into()
}

fn view_sample(
    (index, result): (usize, &Result<(Sample, TrackerSample), xmodits_lib::Error>),
) -> Element<Message> {
    let info = SampleInfo::from(result);

    let error_icon = || {
        row![]
            .push(Space::with_width(Length::Fill))
            .push(icon::warning())
            .align_items(iced::Alignment::Center)
    };

    let title = row![]
        .push(text(format!("{} - {}", index + 1, info.title())))
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
            SampleInfo::Invalid { reason } => centered_container(text(reason)).into(),
            SampleInfo::Sample(smp) => {
                let sample_name =
                    (!smp.name.trim().is_empty()).then_some(text(format!("Name: {}", smp.name.trim())));

                let sample_filename = smp
                    .filename
                    .as_ref()
                    // .filter(|f| !f.is_empty())
                    .map(|n| text(format!("File Name: {}", n.trim())));

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
