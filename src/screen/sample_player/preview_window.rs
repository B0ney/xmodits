mod wave_cache;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use audio_engine::{PlayerHandle, Sample, SamplePack, TrackerSample};
use iced::widget::{button, checkbox, column, row, scrollable, text, Space};
use iced::window::Id;
use iced::{Command, Length};

use crate::widget::helpers::{centered_container, centered_text, fill_container, warning};
use crate::widget::waveform_view::{WaveData, WaveformViewer};
use crate::widget::{Button, Collection, Container, Element, Row};
use crate::{icon, theme};

use self::wave_cache::WaveCache;

#[derive(Debug, Clone)]
pub enum Message {
    Play,
    Pause,
    Stop,
    Volume(f32),
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
        }
    }

    pub fn play(&self) {
        self.player.stop();
        let Some(sample_pack) = &self.sample_pack else {
            return;
        };

        let Some((index, _)) = self.selected else {
            return;
        };

        if let Ok((_, sample)) = &sample_pack.samples[index] {
            self.player.play(sample.clone());
        };
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::Play => self.play(),
            Message::Pause => self.player.pause(),
            Message::Stop => self.player.stop(),
            Message::Volume(vol) => self.player.set_volume(vol),
            Message::Load(path) => {
                return match &self.sample_pack {
                    Some(f) if !f.matches_path(&path) => {
                        self.sample_pack = None;
                        self.selected = None;

                        load_sample_pack(path)
                    }
                    _ => Command::none(),
                }
            }
            Message::Loaded(result) => match Arc::into_inner(result).unwrap() {
                Ok((sample_pack, wave_cache)) => {
                    self.sample_pack = Some(sample_pack);
                    self.wave_cache = wave_cache;
                }
                Err(err) => tracing::error!("{}", err),
            },
            Message::Info(smp) => {
                self.selected = Some(smp);

                if self.play_on_select {
                    self.play();
                }
            }
            Message::SetPlayOnSelect(play_on_select) => self.play_on_select = play_on_select,
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
            .height(Length::Shrink);

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

        let waveform_viewer = WaveformViewer::new_maybe(
            self.selected
                .as_ref()
                .map(|(idx, _)| self.wave_cache.cache.get(&idx))
                .flatten(),
        )
        .style(theme::WaveformView::Hovered(self.hovered));

        let bottom = fill_container(waveform_viewer).height(Length::FillPortion(2));

        let warning = warning(
            || true,
            "Whoops! This is a placeholder error in case something bad happens...",
        );

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

fn load_sample_pack(path: PathBuf) -> Command<Message> {
    Command::perform(
        async {
            tracing::info!("loading samples...");
            tokio::task::spawn_blocking(move || {
                let mut file = File::open(&path)?;
                let module = xmodits_lib::load_module(&mut file)?;
                let sample_pack = SamplePack::build(&*module).with_path(path);
                let wave_cache = WaveCache::from_sample_pack(&sample_pack);
                // tracing::debug!("{:#?}", &sample_pack);

                Ok((sample_pack, wave_cache))
            })
            .await
            .map(Arc::new)
            .unwrap()
        },
        Message::Loaded,
    )
}

#[derive(Debug, Clone)]
pub enum SampleInfo {
    Invalid { reason: String },
    Sample(Sample),
}

impl SampleInfo {
    pub fn title(&self) -> String {
        match &self {
            Self::Sample(smp) => smp.filename_pretty().to_string(),
            Self::Invalid { .. } => "ERROR".into(),
        }
    }
}

impl From<&Result<(Sample, TrackerSample), xmodits_lib::Error>> for SampleInfo {
    fn from(value: &Result<(Sample, TrackerSample), xmodits_lib::Error>) -> Self {
        match value {
            Ok((smp, _)) => Self::Sample(smp.to_owned()),
            Err(e) => Self::Invalid {
                reason: e.to_string(),
            },
        }
    }
}
fn view_sample_info(info: Option<&SampleInfo>) -> Element<Message> {
    match info {
        None => centered_container("Nothing selected...").into(),
        Some(info) => match info {
            SampleInfo::Invalid { reason } => centered_container(text(reason)).into(),
            SampleInfo::Sample(smp) => centered_container(text(smp.filename_pretty())).into(),
        },
    }
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
    let name = info.title();

    row![
        button(text(format!("{index} - {name}")))
            .width(Length::Fill)
            .on_press(Message::Info((index, info)))
            .style(theme::Button::Entry),
        Space::with_width(15)
    ]
    .into()
}
