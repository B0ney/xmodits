//! Responsible for customising the behaviour of XMODITS' ripping routine

pub mod advanced;
pub mod sample_naming;
pub mod sample_ripping;
pub mod name_preview;

use data::config::{SampleNameConfig, SampleRippingConfig};
// use data::name_preview;

use crate::widget::Element;
use iced::Command;

use crate::utils::filename;

use self::advanced::AdvancedConfig;

// #[derive(Debug, Clone)]
// pub enum Message {
//     Ripping(sample_ripping::Message),
//     Naming(sample_naming::Message),
//     Advanced(advanced::Message),
// }

// /// Sample extraction configuration manager
// ///
// #[derive(Debug, Default)]
// pub struct SampleConfigManager {
//     pub ripping: SampleRippingConfig,
//     pub naming: SampleNameConfig,
//     pub advanced: AdvancedConfiguration,
// }

// impl SampleConfigManager {
//     pub fn load() {}
//     pub fn save() {}

//     pub fn update(&mut self, message: Message) -> Command<Message> {
//         tracing::info!("{:?}", &message);
        
//         match message {
//             Message::Ripping(msg) => {
//                 return sample_ripping::update(&mut self.ripping, msg).map(Message::Ripping)
//             }
//             Message::Naming(msg) => sample_naming::update(&mut self.naming, msg),
//             Message::Advanced(msg) => self.advanced.update(msg),
//         }
//         Command::none()
//     }

//     pub fn view_ripping_config(&self) -> Element<Message> {
//         sample_ripping::view(&self.ripping).map(Message::Ripping)
//     }

//     pub fn view_naming_config(&self) -> Element<Message> {
//         // TODO: make name previewer its own helper function
//         let export_format = &self.ripping.exported_format;
//         sample_naming::view(
//             &self.naming,
//             // export_format,
//             // &name_preview::preview_sample_name,
//         )
//         .map(Message::Naming)
//     }

//     // TODO
//     pub fn view_destination(&self) -> Element<Message> {
//         let destination = &self.ripping.destination;

//         let filename_only = false;

//         let destination = match filename_only {
//             true => filename(destination),
//             false => destination.to_str().unwrap_or_default(),
//         };

//         // sample_ripping::view_destination_bar(destination).map(Message::Ripping)
//         todo!()
//     }
// }
