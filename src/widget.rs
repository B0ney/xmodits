//! Custom widgets
//! 
//! Most it graciously yoinked from: https://github.com/squidowl/halloy/
//! 
//! I cannot express my gratitude enough. Those guys are awesome.

pub mod waveform;
pub mod collection;
pub mod context_menu;

pub use self::collection::Collection;

// use crate::theme::Theme;
use iced::Theme;  // TODO: use one above once theme::Theme has been implemented

pub type Renderer = iced::Renderer<Theme>;

pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
pub type Content<'a, Message> = iced::widget::pane_grid::Content<'a, Message, Renderer>;
pub type TitleBar<'a, Message> = iced::widget::pane_grid::TitleBar<'a, Message, Renderer>;
pub type Column<'a, Message> = iced::widget::Column<'a, Message, Renderer>;
pub type Row<'a, Message> = iced::widget::Row<'a, Message, Renderer>;
pub type Text<'a> = iced::widget::Text<'a, Renderer>;
pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
pub type Button<'a, Message> = iced::widget::Button<'a, Message>;