// https://github.com/squidowl/halloy/blob/9d0562a4e0a2643daed7283e6737a4307f21b2c6/src/event.rs
// For reference

use iced::{self, event, keyboard::{self, key::Named}, window, Subscription};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Event {
    Clear,
    CloseRequested,
    Delete,
    FileDropped(window::Id, PathBuf),
    FileHovered(window::Id, PathBuf),
    FileHoveredLeft(window::Id),
    Closed(window::Id),
    Save,
    Start,
}

pub fn events() -> Subscription<Event> {
    iced::event::listen_with(filter)
}

pub fn filter(event: iced::Event, status: event::Status, id: window::Id) -> Option<Event> {
    // If the event has not been handled by any widget
    let ignored =
        |status: event::Status| -> bool { matches!(status, iced::event::Status::Ignored) };

    match event {
        iced::Event::Keyboard(keyboard::Event::KeyReleased {
            key,
            modifiers,
            ..
        }) => match key.as_ref() {
            keyboard::Key::Named(Named::Delete) if ignored(status) => {
                match modifiers.shift() {
                    true => Some(Event::Clear),   // SHIFT + Delete clears the entries
                    false => Some(Event::Delete), // Delete will only delete the selected entries
                }
            }
            // CTRL + S or ⌘ + S saves the current configuration
            keyboard::Key::Character("s") if modifiers.command() => Some(Event::Save),
            _ => None,
        },
        iced::Event::Window(event) => match event {
            window::Event::FileDropped(file) if ignored(status) => {
                Some(Event::FileDropped(id, file))
            }
            window::Event::CloseRequested => Some(Event::CloseRequested),
            window::Event::FileHovered(path) => Some(Event::FileHovered(id, path)),
            window::Event::FilesHoveredLeft => Some(Event::FileHoveredLeft(id)),
            window::Event::Closed => Some(Event::Closed(id)),
            _ => None,
        },
        _ => None,
    }
}
