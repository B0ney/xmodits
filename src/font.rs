use iced::font::{self, Error};
use iced::Command;

// pub static JETBRAINS_MONO: Font = Font::new();

// pub static ICONS: Font = Font::External {
//     name: "Icons",
//     bytes: include_bytes!("../assets/font/material_design_iconic_font.ttf"),
// };

pub static ICONS: iced::Font = iced::Font {
    monospaced: true,
    ..iced::Font::with_name("MaterialDesign Iconic")
};

pub fn load() -> Command<Result<(), Error>> {
    Command::batch(vec![
        font::load(include_bytes!("../assets/font/material_design_iconic_font.ttf").as_slice()),
        font::load(include_bytes!("../assets/font/JetBrainsMono-Regular.ttf").as_slice()),
    ])
}
