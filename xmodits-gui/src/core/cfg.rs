use anyhow::Result;
use dirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use toml;
use xmodits_lib::{exporter::AudioFormat, SampleNamer, SampleNamerTrait};

const APP_NAME: &str = "xmodits";
const CONFIG_NAME: &str = "config.toml";

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .expect("There should be a config directory")
        .join(APP_NAME)
}

pub fn create_config_dir() -> Result<()> {
    Ok(fs::create_dir(config_dir())?)
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
                folder_max_depth: 1,
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
        a.write_all(toml::to_string_pretty(&self)?.as_bytes())?;
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SampleRippingConfig {
    pub destination: PathBuf,
    pub self_contained: bool,
    pub folder_max_depth: u8,
    pub strict: bool,
    pub exported_format: AudioFormat,
    // must be placed at the bottom
    pub naming: SampleNameConfig,
}

impl Default for SampleRippingConfig {
    fn default() -> Self {
        Self {
            destination: dirs::download_dir().expect("Expected Downloads folder"),
            self_contained: true,
            folder_max_depth: 1,
            strict: true,
            exported_format: Default::default(),
            naming: SampleNameConfig {
                index_padding: 2,
                prefer_filename: true,
                ..Default::default()
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct SampleNameConfig {
    pub index_raw: bool,
    pub index_only: bool,
    pub index_padding: u8,
    pub upper: bool,
    pub lower: bool,
    pub prefix: bool,
    pub prefer_filename: bool,
}

impl SampleNameConfig {
    pub fn build_func(&self) -> Box<dyn SampleNamerTrait> {
        SampleNamer {
            index_only: self.index_only,
            index_padding: self.index_padding,
            index_raw: self.index_raw,
            lower: self.lower,
            upper: self.upper,
            prefix_source: self.prefix,
            prefer_filename: self.prefer_filename,
            ..Default::default()
        }
        .into()
    }
}
