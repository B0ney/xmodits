//! The main display panel

pub mod entry;

use data::entries::Entries;
use iced::Element;

use super::tracker_info;

pub enum Message {

}

#[derive(Default, Debug, Clone)]
pub enum State {
    #[default]
    Idle,
    Ripping {
        message: Option<String>,
        progress: f32,
        total_errors: u64,
    },
    Finished(CompleteState),
}

#[derive(Default, Debug, Clone, Copy)]
pub enum CompleteState {
    #[default]
    NoError,
}

pub fn view() {}

pub struct TrackerView {
    pub state: State,
    pub entries: Entries,
}

impl TrackerView {
    pub fn view(&mut self) -> Element<Message> {
        match &self.state {
            State::Idle => match self.entries.is_empty() {
                true => {
                    // Show "Drag and Drop"
                    todo!()
                },
                false => {
                    // Display entries
                    todo!()
                },
            },
            State::Ripping {
                message,
                progress,
                total_errors,
            } => {
                todo!()
            },
            State::Finished(ref complete_state) => self.view_finished(complete_state),
        }
    }

    fn view_finished(&self, complete_state: &CompleteState) -> Element<Message> {
        match complete_state {
            CompleteState::NoError => todo!(),
        }
    }

    // needs to also update 
    pub fn update(&mut self) {}

}
