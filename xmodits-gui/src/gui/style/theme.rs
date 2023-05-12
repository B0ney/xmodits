use iced::{color, Color};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Theme {
    #[default]
    Dark,
    Dracula,
    Catppuccin,
    Nord,
    LMMS,
    OneShot,
    // Custom(&'static std::path::Path),
}

#[derive(Debug, Clone, Copy)]
pub struct BaseColors {
    pub background: Color,
    pub foreground: Color,
    pub dark: Color,   // TODO: sort
    pub border: Color, // TODO: sort
}

#[derive(Debug, Clone, Copy)]
pub struct NormalColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct BrightColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    pub base: BaseColors,
    pub normal: NormalColors,
    pub bright: BrightColors,
}

impl Theme {
    pub const ALL: [Self; 6] = [
        Self::Dark,
        Self::Dracula,
        Self::Catppuccin,
        Self::Nord, 
        Self::LMMS, 
        Self::OneShot
    ];

    pub fn palette(&self) -> ColorPalette {
        match self {
            Self::Dark => ColorPalette {
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
            Self::Dracula => ColorPalette {
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
            Self::LMMS => ColorPalette {
                base: BaseColors {
                    background: color!(0x26_2B_30),
                    foreground: color!(0x3B424A), //3B424A
                    dark: color!(0x11_13_14),
                    border: color!(0x4C5864),
                },
                normal: NormalColors {
                    primary: color!(0x309655),
                    secondary: color!(0xa3be8c),
                    surface: color!(0xe5e9f0),
                    error: color!(0xff5555),
                },
                bright: BrightColors {
                    primary: color!(0x0BD556),
                    secondary: color!(0xa3be8c),
                    surface: color!(0xe5e9f0),
                    error: color!(0xff5555),
                },
            },
            Self::Nord => ColorPalette {
                base: BaseColors {
                    background: color!(0x3b4252),
                    foreground: color!(0x434c5e),
                    dark: color!(0x2e3440),
                    border: color!(0x4c566a),
                },
                normal: NormalColors {
                    primary: color!(0x88c0d0),
                    secondary: color!(0xa3be8c),
                    surface: color!(0xe5e9f0),
                    error: color!(0xff5555),
                },
                bright: BrightColors {
                    primary: color!(0x88c0d0),
                    secondary: color!(0xa3be8c),
                    surface: color!(0xe5e9f0),
                    error: color!(0xff5555),
                },
            },
            Self::OneShot => ColorPalette {
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
            Self::Catppuccin => ColorPalette {
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

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Theme::Dark => "Dark",
                Theme::Dracula => "Dracula",
                Theme::Nord => "Nord",
                Theme::LMMS => "LMMS",
                Theme::OneShot => "OneShot",
                Theme::Catppuccin => "Catppuccin",
            }
        )
    }
}
