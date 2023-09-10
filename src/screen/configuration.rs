//! Responsible for customising the behaviour of XMODITS' ripping routine

pub mod advanced;
pub mod sample_naming;
pub mod sample_ripping;

use data::config::{SampleNameConfig, SampleRippingConfig};
use data::name_preview;

use iced::{Command, Element};

use self::advanced::AdvancedConfiguration;

#[derive(Debug, Clone)]
pub enum Message {
    Ripping(sample_ripping::Message),
    Naming(sample_naming::Message),
    Advanced(advanced::Message),
}

/// Sample extraction configuration manager
///
#[derive(Debug, Default)]
pub struct SampleConfigManager {
    ripping: SampleRippingConfig,
    naming: SampleNameConfig,
    advanced: AdvancedConfiguration,
}

impl SampleConfigManager {
    pub fn load() {}
    pub fn save() {}

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Ripping(msg) => {
                return sample_ripping::update(&mut self.ripping, msg).map(Message::Ripping)
            }
            Message::Naming(msg) => sample_naming::update(&mut self.naming, msg),
            Message::Advanced(msg) => self.advanced.update(msg),
        }
        Command::none()
    }

    pub fn view_ripping_config(&self) -> Element<Message> {
        sample_ripping::view(&self.ripping).map(Message::Ripping)
    }

    pub fn view_naming_config(&self) -> Element<Message> {
        sample_naming::view(&self.naming, &name_preview::preview_sample_name).map(Message::Naming)
    }
}
