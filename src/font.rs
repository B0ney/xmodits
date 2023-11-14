use iced::font::{Font, Family, Weight};

pub static JETBRAINS_MONO: Font = Font {
    family: Family::Monospace,
    weight: Weight::Medium,
    ..Font::with_name("JetBrainsMonoNL-Medium")
};

pub static ICONS: Font = Font::with_name("icons");

pub mod bytes {
    pub static JETBRAINS_MONO: &[u8] = include_bytes!("../assets/font/JetBrainsMonoNL-Medium.ttf");
    pub static ICONS: &[u8] = include_bytes!("../assets/font/icons.ttf");
    pub static ICED_AW_ICONS: &[u8] = iced_aw::graphics::icons::ICON_FONT_BYTES;
}