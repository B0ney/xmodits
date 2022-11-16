use iced::Font;

pub const JETBRAINS_MONO: Font = Font::External {
    name: "Jetbrains Mono",
    bytes: include_bytes!("../../res/font/JetBrainsMono-Regular.ttf"),
};

pub const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../res/font/material_design_iconic_font.ttf"),
};
