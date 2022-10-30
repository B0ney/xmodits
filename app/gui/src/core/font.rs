use iced::Font;

pub const JETBRAINS_MONO: Font = Font::External { 
    name: "Jetbrains Mono",
    bytes: include_bytes!("../../res/font/JetBrainsMono-Regular.ttf"), 
};