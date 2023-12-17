use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use audio_engine::{PlayerHandle, SamplePack};
use iced::widget::{button, column, row};
use iced::window::Id;
use iced::{Command, Length};

use crate::widget::helpers::warning;
use crate::widget::{Button, Collection, Container, Element, Row};
use crate::{icon, theme};

#[derive(Debug, Clone)]
pub enum Message {
    Play(usize),
    Pause,
    Stop,
    Volume(f32),
    Loaded(Arc<Result<SamplePack, xmodits_lib::Error>>),
    Load(PathBuf),
}

enum State {
    Play,
    Paused,
}

pub struct SamplePreviewWindow {
    player: PlayerHandle,
    sample_pack: Option<SamplePack>,
    id: Id,
    pub hovered: bool,
    state: State,
}

impl SamplePreviewWindow {
    pub fn create(id: Id, player: PlayerHandle) -> Self {
        Self {
            player,
            id,
            hovered: false,
            sample_pack: None,
            state: State::Play,
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::Play(index) => {
                let Some(sample_pack) = &self.sample_pack else {
                    return Command::none();
                };

                if let Ok((_, sample)) = &sample_pack.samples[index] {
                    self.player.stop();
                    self.player.play(sample.clone());
                };
            }
            Message::Pause => self.player.pause(),
            Message::Stop => self.player.stop(),
            Message::Volume(vol) => self.player.set_volume(vol),
            Message::Load(path) => {
                if self.sample_pack.as_ref().is_some_and(|f| !f.matches_path(&path)) {
                    return load_sample_pack(path);
                }
            }
            Message::Loaded(result) => match Arc::into_inner(result).unwrap() {
                Ok(sample_pack) => self.sample_pack = Some(sample_pack),
                Err(err) => tracing::error!("{}", err),
            },
        }
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let top_left = Container::new("Information about selected sample")
            .padding(8)
            .style(theme::Container::BlackHovered(self.hovered))
            .width(Length::Fill)
            .height(Length::Fill);

        let controls = media_button([
            (icon::play().size(18), Message::Play(0)),
            (icon::stop().size(18), Message::Stop),
            (icon::pause().size(18), Message::Pause),
            (icon::repeat().size(18), Message::Stop),
        ]);

        let control_panel = Container::new(controls)
            .padding(8)
            .style(theme::Container::BlackHovered(self.hovered))
            .width(Length::Fill)
            .height(Length::Shrink);

        let top_left = column![top_left, control_panel].spacing(5).width(Length::Fill);

        let top_right = Container::new("Sample list")
            .padding(8)
            .style(theme::Container::BlackHovered(self.hovered))
            .width(Length::Fill)
            .height(Length::Fill);

        let bottom = Container::new("Really cool looking waveform")
            .padding(8)
            .style(theme::Container::BlackHovered(self.hovered))
            .width(Length::Fill)
            .height(Length::FillPortion(2));

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

        Container::new(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .into()
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
                tracing::debug!("{:#?}", &sample_pack);

                Ok(sample_pack)
            })
            .await
            .map(Arc::new)
            .unwrap()
        },
        Message::Loaded,
    )
}
