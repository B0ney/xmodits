use iced::font::{self, Error};
use iced::Command;

pub static JETBRAINS_MONO: iced::Font = iced::Font {
    monospaced: true,
    ..iced::Font::with_name("JetBrains Mono")
};

pub static ICONS: iced::Font = iced::Font {
    monospaced: true,
    ..iced::Font::with_name("bootstrap-icons")

};

pub fn load() -> Command<Result<(), Error>> {
    Command::batch([
        font::load(include_bytes!("../assets/font/icons.ttf").as_slice()),
        font::load(include_bytes!("../assets/font/JetBrainsMono-Regular.ttf").as_slice()),
    ])
}
