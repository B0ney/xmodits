use anyhow::Result;
use dirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use toml;
use xmodits_lib::{SampleNamer, SampleNamerFunc};

const APP_NAME: &str = "xmodits";
const CONFIG_NAME: &str = "config.toml";

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .expect("There should be a config directory")
        .join(APP_NAME)
}

pub fn create_config_dir() -> Result<()> {
    Ok(fs::create_dir(&config_dir())?)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub general: GeneralConfig,
    pub ripping: SampleRippingConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: Default::default(),
            ripping: SampleRippingConfig {
                destination: dirs::download_dir().expect("Expected Downloads folder"),
                folder_recursion_depth: 1,
                naming: SampleNameConfig {
                    index_padding: 2,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
impl Config {
    pub fn load() -> Self {
        let Ok(toml) = fs::read_to_string(Self::path()) else {
            return Self::default();
        };

        let Ok(config) = toml::from_str::<Self>(&toml) else {
            return Self::default();
        };

        config
    }

    pub fn save(&self) -> Result<()> {
        if !config_dir().exists() {
            create_config_dir()?;
        };
        use std::io::prelude::*;
        let mut a = fs::File::create(Self::path())?;
        a.write_all(&toml::to_vec(&self)?)?;
        Ok(())
    }

    pub fn filename() -> &'static str {
        CONFIG_NAME
    }

    pub fn path() -> PathBuf {
        config_dir().join(Self::filename())
    }

    // pub fn exists() -> bool {
    //     Self::path().exists()
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GeneralConfig {
    // pub sfx: bool,
    // pub folder_recursion_depth: u8,
    pub logging_path: Option<PathBuf>,
    // pub quiet_output: bool,
}

// Warning, due to the limitations of the toml format,
// the order of these properties matter.
// Structs are treated as tables, and so must be placed at the bottom.
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct SampleRippingConfig {
    pub destination: PathBuf,
    pub hint: FormatHint,
    pub no_folder: bool,
    pub embed_loop_points: bool,
    pub folder_recursion_depth: u8,
    pub naming: SampleNameConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SampleNameConfig {
    pub index_raw: bool,
    pub index_only: bool,
    pub index_padding: u8,
    pub upper: bool,
    pub lower: bool,
}

impl SampleNameConfig {
    pub fn build_func(&self) -> Box<SampleNamerFunc> {
        SampleNamer::build_func(
            self.index_only,
            Some(self.index_padding.into()),
            self.index_raw,
            self.lower,
            self.upper,
        )
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormatHint {
    #[default]
    None,
    IT,
    XM,
    S3M,
    MOD,
    UMX,
}

impl FormatHint {
    pub const ALL: [FormatHint; 6] = [
        FormatHint::None,
        FormatHint::IT,
        FormatHint::XM,
        FormatHint::S3M,
        FormatHint::MOD,
        FormatHint::UMX,
    ];
}

impl Into<Option<String>> for FormatHint {
    fn into(self) -> Option<String> {
        match self {
            FormatHint::None => None,
            hint => Some(hint.to_string().to_lowercase()),
        }
    }
}

// impl Into<Option<String>> for &FormatHint {
//     fn into(self) -> Option<String> {
//         match self {
//             FormatHint::None => None,
//             hint => Some(hint.to_string().to_lowercase()),
//         }
//     }
// }

impl std::fmt::Display for FormatHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FormatHint::None => "None",
                FormatHint::IT => "IT",
                FormatHint::XM => "XM",
                FormatHint::S3M => "S3M",
                FormatHint::MOD => "MOD",
                FormatHint::UMX => "UMX",
            }
        )
    }
}
