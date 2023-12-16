#[cfg(feature = "audio")]
mod sample_player_inner {
    use iced::window::Action;
    use iced::{window, Command, Length};

    use std::collections::HashMap;
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    use crate::app::application_icon;
    use crate::theme;
    use crate::widget::waveform;
    use crate::widget::Container;
    use crate::widget::Element;
    use audio_engine::{PlayerHandle, SamplePack, SamplePlayer};

    #[derive(Debug, Clone)]
    pub enum Message {
        Window(window::Id, PreviewWindowMessage),
    }

    #[derive(Default)]
    pub struct SamplePreview {
        audio_engine: SamplePlayer,
        windows: HashMap<window::Id, SamplePreviewWindow>,
        multi_instance: bool,
    }

    impl SamplePreview {
        pub fn update(&mut self, msg: Message) -> Command<Message> {
            match msg {
                Message::Window(id, msg) => self.windows.get_mut(&id).unwrap().update_with_id(id, msg),
            }
        }

        pub fn view(&self, id: window::Id) -> Element<Message> {
            self.windows
                .get(&id)
                .expect("View sample preview window")
                .view()
                .map(move |msg| Message::Window(id, msg))
        }

        pub fn close(&mut self, id: window::Id) {
            self.windows.remove_entry(&id);
        }

        // spawn new instance
        pub fn create_instance(&mut self, path: PathBuf) -> Command<Message> {
            if let Some(old_id) = self.find(&path) {
                return window::gain_focus(old_id);
            }

            let (id, spawn_window) = window::spawn(window::Settings {
                size: [640, 480].into(),
                min_size: Some([640, 480].into()),
                icon: Some(application_icon()),
                // platform_specific: todo!(),
                exit_on_close_request: true,
                ..Default::default()
            });

            self.windows.insert(
                id,
                SamplePreviewWindow::create(id, self.audio_engine.create_handle()),
            );

            Command::batch([spawn_window, self.load_samples(id, path)])
        }

        pub fn get_title(&self, id: window::Id) -> String {
            self.windows.get(&id).expect("View sample preview window").title()
        }

        pub fn set_hovered(&mut self, id: window::Id, hovered: bool) {
            self.windows.get_mut(&id).unwrap().hovered = hovered;
        }

        pub fn load_samples(&self, id: window::Id, path: PathBuf) -> Command<Message> {
            self.windows
                .get(&id)
                .unwrap()
                .load_sample_pack(path)
                .map(move |result| Message::Window(id, result))
        }

        pub fn find(&self, path: &Path) -> Option<window::Id> {
            self.windows
                .iter()
                .find_map(|(id, window)| window.matches_path(path).then_some(id))
                .copied()
        }
    }

    #[derive(Debug, Clone)]
    pub enum PreviewWindowMessage {
        Play(usize),
        Pause,
        Stop,
        Loaded(Arc<Result<SamplePack, xmodits_lib::Error>>),
        Load(PathBuf),
    }

    // #[derive(Default)]
    pub struct SamplePreviewWindow {
        player: PlayerHandle,
        sample_pack: Option<SamplePack>,
        id: window::Id,
        hovered: bool,
    }

    impl SamplePreviewWindow {
        pub fn create(id: window::Id, player: PlayerHandle) -> Self {
            Self {
                player,
                id,
                hovered: false,
                sample_pack: None,
            }
        }

        pub fn update_with_id(&mut self, id: window::Id, msg: PreviewWindowMessage) -> Command<Message> {
            self.update(msg).map(move |msg| Message::Window(id, msg))
        }

        pub fn update(&mut self, msg: PreviewWindowMessage) -> Command<PreviewWindowMessage> {
            match msg {
                PreviewWindowMessage::Play(index) => {
                    let Some(sample_pack) = &self.sample_pack else {
                        return Command::none();
                    };

                    if let Ok((_, sample)) = &sample_pack.samples[index] {
                        self.player.stop();
                        self.player.play(sample.clone());
                    };
                }
                PreviewWindowMessage::Pause => self.player.pause(),
                PreviewWindowMessage::Stop => self.player.stop(),
                PreviewWindowMessage::Load(path) => {
                    if self.sample_pack.as_ref().is_some_and(|f| !f.matches_path(&path)) {
                        return load_sample_pack(path);
                    }
                }
                PreviewWindowMessage::Loaded(result) => match Arc::into_inner(result).unwrap() {
                    Ok(sample_pack) => self.sample_pack = Some(sample_pack),
                    Err(err) => tracing::error!("{}", err),
                },
            }
            Command::none()
        }

        pub fn view(&self) -> Element<PreviewWindowMessage> {
            let main = Container::new("s")
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
                Some(pack) => format!("Loaded: {}", pack.name),
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

        pub fn load_sample_pack(&self, path: PathBuf) -> Command<PreviewWindowMessage> {
            match self.matches_path(&path) {
                true => Command::none(),
                false => load_sample_pack(path),
            }
        }
    }

    fn load_sample_pack(path: PathBuf) -> Command<PreviewWindowMessage> {
        let task = async {
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
        };

        Command::perform(task, PreviewWindowMessage::Loaded)
    }
}

#[cfg(feature = "audio")]
pub use sample_player_inner::*;

use iced::Command;

#[cfg(not(feature = "audio"))]
#[derive(Clone, Copy)]
pub struct Message;

#[cfg(not(feature = "audio"))]
#[derive(Default)]
pub struct SamplePreviewWindow;

#[cfg(not(feature = "audio"))]
impl SamplePreviewWindow {
    pub fn update(&mut self, _msg: PreviewWindowMessage) -> Command<PreviewWindowMessage> {
        Command::none()
    }

    pub fn view(&self) {
        unimplemented!("Attempt to view sample player without 'audio' feature")
    }
}
