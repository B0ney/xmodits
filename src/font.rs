use iced::font::{self, Font};
use iced::Task;

pub static JETBRAINS_MONO: Font = Font::with_name("JetBrains Mono NL");
pub static ICONS: Font = Font::with_name("icons");

pub mod bytes {
    pub static JETBRAINS_MONO: &[u8] = include_bytes!("../assets/font/JetBrainsMonoNL-Regular.ttf");
    pub static ICONS: &[u8] = include_bytes!("../assets/font/icons.ttf");
}

pub fn load() -> Task<Result<(), font::Error>> {
    Task::batch([font::load(bytes::JETBRAINS_MONO), font::load(bytes::ICONS)])
}
