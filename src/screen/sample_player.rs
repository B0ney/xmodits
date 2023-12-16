#[cfg(feature = "audio")]
mod preview_window;

#[cfg(feature = "audio")]
mod sample_player_inner {
    use iced::window::{self, Id};
    use iced::{Command, Size};

    use std::collections::HashMap;
    use std::path::{Path, PathBuf};

    use crate::{app::application_icon, widget::Element};
    use audio_engine::SamplePlayer;

    use super::preview_window::{self, SamplePreviewWindow};

    const WINDOW_SIZE: Size = Size::new(640.0, 480.0);

    #[derive(Debug, Clone)]
    pub enum Message {
        ResetEngine,
        Window(Id, preview_window::Message),
    }

    ///
    #[derive(Default)]
    pub struct SamplePreview {
        audio_engine: SamplePlayer,
        windows: HashMap<Id, SamplePreviewWindow>,
        singleton: bool,
    }

    impl SamplePreview {
        pub fn update(&mut self, msg: Message) -> Command<Message> {
            match msg {
                Message::Window(id, msg) => self.update_window(id, msg),
                Message::ResetEngine => todo!(),
            }
        }

        pub fn update_window(&mut self, id: Id, msg: preview_window::Message) -> Command<Message> {
            self.get_window_mut(id)
                .update(msg)
                .map(move |msg| Message::Window(id, msg))
        }

        pub fn view(&self, id: Id) -> Element<Message> {
            self.get_window(id)
                .view()
                .map(move |msg| Message::Window(id, msg))
        }

        pub fn close(&mut self, id: Id) {
            self.windows.remove_entry(&id);
        }

        // spawn new instance
        pub fn create_instance(&mut self, path: PathBuf) -> Command<Message> {
            if let Some(old_id) = self.find(&path) {
                return window::gain_focus(old_id);
            }

            let (id, spawn_window) = window::spawn(window::Settings {
                size: WINDOW_SIZE,
                min_size: Some(WINDOW_SIZE),
                icon: Some(application_icon()),
                exit_on_close_request: true,
                ..Default::default()
            });

            self.windows.insert(
                id,
                SamplePreviewWindow::create(id, self.audio_engine.create_handle()),
            );

            Command::batch([spawn_window, self.load_samples(id, path)])
        }

        pub fn get_title(&self, id: Id) -> String {
            self.get_window(id).title()
        }

        pub fn set_hovered(&mut self, id: Id, hovered: bool) {
            self.get_window_mut(id).hovered = hovered;
        }

        pub fn load_samples(&self, id: Id, path: PathBuf) -> Command<Message> {
            self.get_window(id)
                .load_sample_pack(path)
                .map(move |result| Message::Window(id, result))
        }

        // find a window that already has a tracker loaded
        pub fn find(&self, path: &Path) -> Option<Id> {
            self.windows
                .iter()
                .find_map(|(id, window)| window.matches_path(path).then_some(id))
                .copied()
        }

        pub fn get_window(&self, id: Id) -> &SamplePreviewWindow {
            self.windows.get(&id).expect("View sample preview window")
        }

        pub fn get_window_mut(&mut self, id: Id) -> &mut SamplePreviewWindow {
            self.windows.get_mut(&id).expect("View sample preview window")
        }
    }
}

use std::path::PathBuf;

#[cfg(feature = "audio")]
pub use sample_player_inner::*;

use crate::widget::Element;
use iced::{window::Id, Command};

#[cfg(not(feature = "audio"))]
#[derive(Clone, Copy, Debug)]
pub struct Message;

#[cfg(not(feature = "audio"))]
#[derive(Default)]
pub struct SamplePreview;

#[cfg(not(feature = "audio"))]
impl SamplePreview {
    pub fn update(&mut self, _msg: Message) -> Command<Message> {
        Command::none()
    }
    pub fn view(&self, _id: Id) -> Element<Message> {
        unimplemented!("Attempt to view sample player without 'audio' feature")
    }
    pub fn load_samples(&self, _id: Id, _path: PathBuf) -> Command<Message> {
        Command::none()
    }
    pub fn create_instance(&mut self, _path: PathBuf) -> Command<Message> {
        Command::none()
    }
    pub fn set_hovered(&mut self, _id: Id, _hovered: bool) {}
    pub fn close(&mut self, _id: Id) {}
    pub fn get_title(&self, _id: Id) -> String {
        unimplemented!("Attempt to view sample player without 'audio' feature")
    }
}
