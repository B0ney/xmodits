// https://github.com/squidowl/halloy/blob/9d0562a4e0a2643daed7283e6737a4307f21b2c6/src/event.rs
// For reference

use iced::{self, event, keyboard::{self, Modifiers}, window, Subscription};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Event {
    Clear,
    CloseRequested,
    Delete,
    FileDropped(PathBuf),
    Save,
    Start,
}

pub fn events() -> Subscription<Event> {
    iced::event::listen_with(filter)
}

pub fn filter(event: iced::Event, status: event::Status) -> Option<Event> {
    // If the event has not been handled by any widget
    let ignored =
        |status: event::Status| -> bool { matches!(status, iced::event::Status::Ignored) };

    match event {
        // Delete will only delete the selected entries
        iced::Event::Keyboard(keyboard::Event::KeyPressed {
            key_code: keyboard::KeyCode::Delete,
            ..
        }) if ignored(status) => Some(Event::Delete),

        // SHIFT + Delete clears the entries
        iced::Event::Keyboard(keyboard::Event::KeyPressed {
            key_code: keyboard::KeyCode::Delete,
            modifiers,
        }) if modifiers.shift() => Some(Event::Clear),

        // CTRL + S or ⌘ + S saves the current configuration
        iced::Event::Keyboard(keyboard::Event::KeyReleased {
            key_code: keyboard::KeyCode::S,
            modifiers,
        }) if modifiers.command() => Some(Event::Save),

        iced::Event::Window(window::Event::FileDropped(file)) if ignored(status) => {
            Some(Event::FileDropped(file))
        }
        iced::Event::Window(window::Event::CloseRequested) => Some(Event::CloseRequested),
        _ => None,
    }
}
