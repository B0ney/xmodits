#[cfg(feature = "audio")]
mod sample_player_inner {
    use iced::Command;
    use std::path::PathBuf;
    use std::fs::File;
    use std::sync::Arc;

    use audio_engine::{SamplePack, SamplePlayer};
    use crate::widget::Element;
    use crate::widget::waveform;

    #[derive(Debug, Clone)]
    pub enum Message {
        Play(usize),
        Pause,
        Stop,
        Loaded(Arc<Result<SamplePack, xmodits_lib::Error>>),
        Load(PathBuf),
    }

    #[derive(Default)]
    pub struct SamplePreviewWindow {
        player: SamplePlayer,
        sample_pack: Option<SamplePack>,
    }

    impl SamplePreviewWindow {
        pub fn update(&mut self, msg: Message) -> Command<Message> {
            match msg {
                Message::Play(index) => {
                    let Some(sample_pack) = &self.sample_pack else {
                        return Command::none();
                    };

                    if let Ok((_, sample)) = &sample_pack.samples[index]  {
                        self.player.stop();
                        self.player.play(sample.clone());
                    };
                }
                Message::Pause => self.player.pause(),
                Message::Stop => self.player.stop(),
                Message::Load(path) => {
                    if self.sample_pack.as_ref().is_some_and(|f| !f.matches_path(&path)) {
                        return Command::perform(load_sample_pack(path), Message::Loaded);
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
            todo!()
        }
    }

    pub async fn load_sample_pack(path: PathBuf) -> Arc<Result<SamplePack, xmodits_lib::Error>> {
        tokio::task::spawn_blocking(move || {
            let mut file = File::open(&path)?;
            let module = xmodits_lib::load_module(&mut file)?;

            Ok(SamplePack::build(&*module).with_path(path))
        })
        .await
        .map(Arc::new)
        .unwrap()
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
    pub fn update(&mut self, _msg: Message) -> Command<Message> {
        Command::none()
    }

    pub fn view(&self) {
        unimplemented!("Attempt to view sample player without 'audio' feature")
    }
}
