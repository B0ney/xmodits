use anyhow::Result;
use dirs;
use serde::{Deserialize, Serialize};
use xmodits_lib::{SampleNamer, SampleNamerFunc};
use std::{fs, path::PathBuf};
use toml;
use tracing::{info, warn, Level};

const APP_NAME: &str = "xmodits";
const CONFIG_NAME: &str = "config.toml";

pub fn config_dir() -> PathBuf {
    let config_dir = dirs::config_dir()
        .expect("There should be a config directory")
        .join(APP_NAME);

    config_dir
}

pub fn create_config_dir() -> Result<()> {
    Ok(fs::create_dir(&config_dir())?)
}

// TODO: rename to config
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub general: GeneralConfig,
    pub ripping: SampleRippingConfig,
}

impl Config {
    pub fn load() -> Self {
        let Ok(toml) = fs::read_to_string(Self::path()) else {
            warn!("cant load");
            return Self::default();
        };

        let Ok(config) = toml::from_str::<Self>(&toml) else {
            warn!("cant convert");
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

    pub fn exists() -> bool {
        Self::path().exists()
    }

    pub fn name_cfg(&self) -> &SampleNameConfig {
        &self.ripping.naming
    }
    
    pub fn name_cfg_mut(&mut self) -> &mut SampleNameConfig {
        &mut self.ripping.naming
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GeneralConfig {
    pub sfx: bool,
    pub folder_recursion_depth: u8,
    pub logging_path: PathBuf,
    pub quiet_output: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SampleRippingConfig {
    pub destination: PathBuf,
    pub hint: Option<String>,
    pub no_folder: bool,
    pub embed_loop_points: bool,
    pub naming: SampleNameConfig,
}

impl Default for SampleRippingConfig {
    fn default() -> Self {
        Self { 
            destination: dirs::download_dir()
                .expect("Expected Downloads folder"),
            hint: None,
            no_folder: false,
            embed_loop_points: false,
            naming: SampleNameConfig::default(),
        }
    }
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
            self.upper
        )
    }

    pub fn set_index_only(set: bool) {

    }
}
