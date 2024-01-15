use iced::font::{self, Family, Font, Weight};
use iced::Command;

pub static JETBRAINS_MONO: Font = Font::with_name("JetBrains Mono NL");
pub static ICONS: Font = Font::with_name("icons");

pub mod bytes {
    pub static JETBRAINS_MONO: &[u8] = include_bytes!("../assets/font/JetBrainsMonoNL-Regular.ttf");
    pub static ICONS: &[u8] = include_bytes!("../assets/font/icons.ttf");
    // pub static ICED_AW_ICONS: &[u8] = iced_aw::graphics::icons::ICON_FONT_BYTES;
}

pub fn load() -> Command<Result<(), font::Error>> {
    Command::batch([
        font::load(bytes::JETBRAINS_MONO), 
        font::load(bytes::ICONS)
    ])
}
