use anyhow::Result;
use dirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tokio::io::AsyncWriteExt;
use toml;
use tracing::{error, info, warn};
use xmodits_lib::{exporter::AudioFormat, SampleNamer, SampleNamerTrait};

use crate::gui::style::{Themes, Theme};

const APP_NAME: &str = "xmodits";
const CONFIG_NAME: &str = "config.toml";

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .expect("There should be a config directory")
        .join(APP_NAME)
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub general: GeneralConfig,
    pub ripping: SampleRippingConfig,
}

impl Config {
    pub fn load() -> Self {
        let Ok(toml) = fs::read_to_string(Self::path()) else {
            info!("Generating Default config file. Note that this won't be saved.");
            return Self::default();
        };

        let Ok(config) = toml::from_str(&toml) else {
            warn!("Could not parse config file. Perhaps an older version was loaded...");
            return Self::default();
        };

        config
    }

    pub async fn save(&self) -> Result<()> {
        if !config_dir().exists() {
            info!("Creating config directory: {}", config_dir().display());
            tokio::fs::create_dir(config_dir()).await?;
        };

        let file = tokio::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(Self::path())
            .await;

        if let Err(e) = &file {
            error!("{}", e);
        }

        let result = file?
            .write_all(toml::to_string_pretty(&self)?.as_bytes())
            .await;

        if let Err(e) = &result {
            error!("{}", e)
        } else {
            info!("Saved Configuration!");
        }

        Ok(result?)
    }
    pub fn filename() -> &'static str {
        CONFIG_NAME
    }

    pub fn path() -> PathBuf {
        config_dir().join(Self::filename())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GeneralConfig {
    /// Theme of the application
    pub theme: Themes,
    // pub sfx: bool,
    // pub folder_recursion_depth: u8,
    pub logging_path: Option<PathBuf>,
    pub worker_threads: usize,
    pub non_gui_quiet_output: bool,
    /// Use the current working directory for extraction
    pub non_gui_use_cwd: bool,
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
    pub worker_threads: usize,
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
            worker_threads: 0,
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
