use iced::font::{Font, Family, Weight};

pub static JETBRAINS_MONO: Font = Font {
    family: Family::Monospace,
    weight: Weight::Medium,
    ..Font::with_name("JetBrains Mono")
};

pub static ICONS: Font = Font::with_name("icons");
