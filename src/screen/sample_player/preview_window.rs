use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use audio_engine::{PlayerHandle, SamplePack};

use iced::widget::{button, row};
use iced::window::Id;
use iced::{Command, Length};

use crate::theme;
use crate::widget::{Container, Element};

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
        let play = button("PLAY").on_press(Message::Play(0));
        let pause = button("PAUSE").on_press(Message::Pause);
        let stop = button("STOP").on_press(Message::Stop);

        let controls = row![play, pause, stop];
        let main = Container::new(controls)
            .padding(5)
            .style(theme::Container::BlackHovered(self.hovered))
            .width(Length::Fill)
            .height(Length::Fill);

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
