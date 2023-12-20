//! Custom widgets
//!
//! Most it graciously yoinked from: https://github.com/squidowl/halloy/
//!
//! I cannot express my gratitude enough. Those guys are awesome.

pub mod animation;
pub mod collection;
pub mod helpers;

#[cfg(feature = "audio")]
pub mod waveform_view;

pub use self::collection::Collection;

use crate::theme::Theme;
// use iced::Theme;

pub type Renderer = iced::Renderer<Theme>;

pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
pub type Content<'a, Message> = iced::widget::pane_grid::Content<'a, Message, Renderer>;
pub type TitleBar<'a, Message> = iced::widget::pane_grid::TitleBar<'a, Message, Renderer>;
pub type Column<'a, Message> = iced::widget::Column<'a, Message, Renderer>;
pub type Row<'a, Message> = iced::widget::Row<'a, Message, Renderer>;
pub type Text<'a> = iced::widget::Text<'a, Renderer>;
pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
pub type PickList<'a, Message, T> = iced::widget::PickList<'a, T, Message, Renderer>;