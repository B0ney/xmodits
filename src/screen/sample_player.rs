#[cfg(feature = "audio")]
mod instance;
#[cfg(feature = "audio")]
mod preview_manager;

#[cfg(feature = "audio")]
pub use preview_manager::*;

#[cfg(not(feature = "audio"))]
pub mod preview_manager_dummy {
    use crate::screen::entry::Entries;

    use iced::{window::Id, Command};
    use std::path::PathBuf;

    #[derive(Clone, Copy, Debug)]
    pub struct Message;

    #[derive(Default)]
    pub struct SamplePreview;

    impl SamplePreview {
        pub fn update(&mut self, _msg: Message, _entries: &mut Entries) -> Command<Message> {
            Command::none()
        }
        pub fn load_samples(&self, _id: Id, _path: PathBuf) -> Command<Message> {
            Command::none()
        }
        pub fn create_instance(&mut self, _path: PathBuf) -> Command<Message> {
            Command::none()
        }
        pub fn close_all(&self) -> Command<Message> {
            Command::none()
        }
        pub fn remove_instance(&self, _id: Id) {}
        pub fn set_hovered(&mut self, _id: Id, _hovered: bool) {}
        pub fn close(&mut self, _id: Id) {}
        pub fn get_title(&self, _id: Id) -> String {
            unimplemented!("Attempt to view sample player without 'audio' feature")
        }
    }
}

#[cfg(not(feature = "audio"))]
pub use preview_manager_dummy::*;
