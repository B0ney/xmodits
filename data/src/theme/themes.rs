use iced_core::color;
use serde::{Deserialize, Serialize};

use super::{BaseColors, BrightColors, NormalColors, Palette};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Themes {
    #[default]
    Dark,
    Dracula,
    Catppuccin,
    Nord,
    LMMS,
    OneShot,
}

impl Themes {
    pub const ALL: [Self; 6] = [
        Self::Dark,
        Self::Dracula,
        Self::Catppuccin,
        Self::Nord,
        Self::LMMS,
        Self::OneShot,
    ];

    pub fn palette(&self) -> Palette {
        match self {
            Self::Dark => Palette {
                base: BaseColors {
                    background: color!(0x272727),
                    foreground: color!(0x353535),
                    dark: color!(0x151515),
                    border: color!(0x474747),
                },
                normal: NormalColors {
                    primary: color!(0x5E4266),
                    secondary: color!(0x386e50),
                    surface: color!(0x828282),
                    error: color!(0xff5555),
                },
                bright: BrightColors {
                    primary: color!(0xBA84FC),
                    secondary: color!(0x49eb7a),
                    surface: color!(0xE0E0E0),
                    error: color!(0xff5555),
                },
            },
            Self::Dracula => Palette {
                base: BaseColors {
                    background: color!(0x282a36),
                    foreground: color!(0x44475a),
                    dark: color!(0x1D1E26),
                    border: color!(0x4f5263),
                },
                normal: NormalColors {
                    primary: color!(0xff79c6),
                    secondary: color!(0x50fa7b),
                    surface: color!(0xf8f8f2),
                    error: color!(0xff5555),
                },
                bright: BrightColors {
                    primary: color!(0xff79c6),
                    secondary: color!(0x50fa7b),
                    surface: color!(0xf8f8f2),
                    error: color!(0xff5555),
                },
            },
            Self::LMMS => Palette {
                base: BaseColors {
                    background: color!(0x26_2B_30),
                    foreground: color!(0x3B424A), //3B424A
                    dark: color!(0x11_13_14),
                    border: color!(0x4C5864),
                },
                normal: NormalColors {
                    primary: color!(0x309655),
                    secondary: color!(0x309655),
                    surface: color!(0xe5e9f0),
                    error: color!(0xff5555),
                },
                bright: BrightColors {
                    primary: color!(0x0BD556),
                    secondary: color!(0x0BD556),
                    surface: color!(0xe5e9f0),
                    error: color!(0xff5555),
                },
            },
            Self::Nord => Palette {
                base: BaseColors {
                    background: color!(0x2e3440),
                    foreground: color!(0x3b4252),
                    dark: color!(0x21252d),
                    border: color!(0x50586d),
                },
                normal: NormalColors {
                    primary: color!(0x88c0d0),
                    secondary: color!(0xa3be8c),
                    surface: color!(0xe5e9f0),
                    error: color!(0xbf616a),
                },
                bright: BrightColors {
                    primary: color!(0x88c0d0),
                    secondary: color!(0xa3be8c),
                    surface: color!(0xe5e9f0),
                    error: color!(0xbf616a),
                },
            },
            Self::OneShot => Palette {
                base: BaseColors {
                    background: color!(0x1A0B1D),
                    foreground: color!(0x2B0D1A),
                    dark: color!(0x100213),
                    border: color!(0xFBCD5D),
                },
                normal: NormalColors {
                    primary: color!(0xF48550),
                    secondary: color!(0x80FF80),
                    surface: color!(0xFEFECD),
                    error: color!(0xff5555),
                },
                bright: BrightColors {
                    primary: color!(0xFBCD5D),
                    secondary: color!(0x80FF80),
                    surface: color!(0xFEFECD),
                    error: color!(0xff5555),
                },
            },
            Self::Catppuccin => Palette {
                base: BaseColors {
                    background: color!(0x1E1E28),
                    foreground: color!(0x332E41),
                    dark: color!(0x1B1923),
                    border: color!(0x6E6C7E),
                },
                normal: NormalColors {
                    primary: color!(0xC6AAE8),
                    secondary: color!(0xB1E3AD),
                    surface: color!(0xD7DAE0),
                    error: color!(0xE38C8F),
                },
                bright: BrightColors {
                    primary: color!(0xC6AAE8),
                    secondary: color!(0xB1E3AD),
                    surface: color!(0xFEFECD),
                    error: color!(0xE38C8F),
                },
            },
        }
    }
}

impl std::fmt::Display for Themes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Themes::Dark => "Dark",
                Themes::Dracula => "Dracula",
                Themes::Nord => "Nord",
                Themes::LMMS => "LMMS",
                Themes::OneShot => "OneShot",
                Themes::Catppuccin => "Catppuccin",
                // Themes::Custom() => "Custom Theme"
            }
        )
    }
}