use super::ColorPalette;
use iced::widget::text;
use iced::Color;

#[derive(Default, Clone, Copy)]
pub enum Text {
    #[default]
    Default,
    Error,
    Color(Color),
}

impl From<Color> for Text {
    fn from(color: Color) -> Self {
        Text::Color(color)
    }
}

impl text::StyleSheet for ColorPalette {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Default => Default::default(),
            Text::Error => text::Appearance {
                color: Some(self.bright.error),
            },
            Text::Color(c) => text::Appearance { color: Some(c) },
        }
    }
}
