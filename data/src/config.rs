use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

pub mod general;
pub mod name_params;
pub mod sample_naming;
pub mod sample_ripping;
// pub mod filters;

pub use general::GeneralConfig;
pub use name_params::SampleNameParams;
pub use sample_naming::SampleNameConfig;
pub use sample_ripping::SampleRippingConfig;

use anyhow::Result;
use tokio::io::AsyncWriteExt;
use tracing::{error, info, warn};

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
    pub naming: SampleNameConfig,
}

impl Config {
    pub fn load() -> Self {
        let Ok(toml) = fs::read_to_string(Self::path()) else {
            info!("Generating Default config file. Note that this won't be saved.");
            return Self::default();
        };

        Self::load_str(&toml)
    }

    pub fn load_str(input: &str) -> Self {
        toml::from_str(input).unwrap_or_else(|e| {
            warn!(
                "Could not parse existing configuration file. Perhaps an older version was loaded?"
            );
            error!("{}", e);
            Self::default()
        })
    }

    pub fn save_str(&self) -> anyhow::Result<String> {
        Ok(toml::to_string_pretty(&self)?)
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
