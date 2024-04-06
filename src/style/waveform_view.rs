//! TODO

use super::{helpers::border, Theme};

#[derive(Default)]
#[cfg(feature = "audio")]
pub enum WaveformView {
    #[default]
    Normal,
    Hovered(bool),
}

#[cfg(feature = "audio")]
impl crate::widget::waveform_view::StyleSheet for Theme {
    type Style = WaveformView;

    fn appearance(&self, style: &Self::Style) -> crate::widget::waveform_view::Appearance {
        let p = self.palette();

        let default = crate::widget::waveform_view::Appearance {
            background: p.background.into(),
            wave_color: p.waveform,
            cursor_color: p.text,
            border: border(p.border),
        };

        match style {
            WaveformView::Normal => default,
            WaveformView::Hovered(hovered) => crate::widget::waveform_view::Appearance {
                border: border(if *hovered { p.accent } else { p.border }),
                ..default
            },
        }
    }
}
