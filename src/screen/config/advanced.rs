//! Advanced features of xmodits
pub mod custom_filters;
pub mod custom_rules;

use crate::widget::Element;

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Debug, Default)]
pub struct AdvancedConfig;

impl AdvancedConfig {
    pub fn view(&self) -> Element<Message> {
        todo!()
    }
    pub fn update(&mut self, message: Message) {}
}
