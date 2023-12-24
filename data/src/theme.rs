pub mod themes;

use iced_core::Color;
use serde::{Deserialize, Serialize};
pub use themes::Themes;

pub struct Theme {
    pub name: String,
    pub palette: Palette,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    pub background: Color,
    pub middleground: Color,
    pub foreground: Color,
    pub border: Color,
    pub text: Color,
    pub accent: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,
    pub waveform: Color,
}

impl Default for Palette {
    fn default() -> Self {
        themes::Themes::Dark.palette()
    }
}

fn hex_to_color(hex: &str) -> Option<Color> {
    if hex.len() == 7 {
        let hash = &hex[0..1];
        let r = u8::from_str_radix(&hex[1..3], 16);
        let g = u8::from_str_radix(&hex[3..5], 16);
        let b = u8::from_str_radix(&hex[5..7], 16);

        return match (hash, r, g, b) {
            ("#", Ok(r), Ok(g), Ok(b)) => Some(Color {
                r: r as f32 / 255.0,
                g: g as f32 / 255.0,
                b: b as f32 / 255.0,
                a: 1.0,
            }),
            _ => None,
        };
    }

    None
}

pub mod palette_serde {
    use super::{hex_to_color, Palette};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct ThemeHex {
        background: String,
        middleground: String,
        foreground: String,
        border: String,
        text: String,
        accent: String,
        error: String,
        warning: String,
        success: String,
        waveform: String,
    }

    impl Serialize for Palette {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            fn as_hex(color: iced_core::Color) -> String {
                format!(
                    "#{:02x}{:02x}{:02x}",
                    (255.0 * color.r).round() as u8,
                    (255.0 * color.g).round() as u8,
                    (255.0 * color.b).round() as u8
                )
            }

            ThemeHex {
                background: as_hex(self.background),
                middleground: as_hex(self.middleground),
                foreground: as_hex(self.foreground),
                border: as_hex(self.border),
                text: as_hex(self.text),
                accent: as_hex(self.accent),
                error: as_hex(self.error),
                warning: as_hex(self.warning),
                success: as_hex(self.success),
                waveform: as_hex(self.waveform),
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Palette {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let hex: ThemeHex = serde::Deserialize::deserialize(deserializer)?;

            let to_color =
                |hex: &str| hex_to_color(hex).ok_or_else(|| serde::de::Error::custom("Invalid hex"));

            Ok(Palette {
                background: to_color(&hex.text)?,
                middleground: to_color(&hex.middleground)?,
                foreground: to_color(&hex.foreground)?,
                border: to_color(&hex.border)?,
                text: to_color(&hex.text)?,
                accent: to_color(&hex.accent)?,
                error: to_color(&hex.error)?,
                warning: to_color(&hex.warning)?,
                success: to_color(&hex.success)?,
                waveform: to_color(&hex.waveform)?,
            })
        }
    }
}
