use iced::Font;

pub static JETBRAINS_MONO: Font = Font::External {
    name: "Jetbrains Mono",
    bytes: include_bytes!("../assets/font/JetBrainsMono-Regular.ttf"),
};

pub static ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../assets/font/material_design_iconic_font.ttf"),
};
