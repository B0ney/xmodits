use std::{fs, path::PathBuf};

use anyhow::Result;
use dirs;
use serde::{Deserialize, Serialize};
use toml;

const APP_NAME: &str = "xmodits";

// User editable configuration
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    index_raw: bool,
    index_only: bool,
    index_padding: usize,
    upper: bool,
    lower: bool,
    no_folder: bool,
    default_destination: Option<String>,
}

pub fn config_dir() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap().join(APP_NAME);

    if !config_dir.exists() {
        fs::create_dir(&config_dir).unwrap()
    }

    config_dir
}

impl Config {
    pub fn load() -> Self {
        match fs::read_to_string(Config::path()) {
            Ok(j) => match toml::from_str::<Self>(&j) {
                Ok(s) => s,
                Err(_) => Self::default(),
            },
            Err(_) => Self::default(),
        }
    }

    pub fn path() -> PathBuf {
        config_dir().join(Config::filename())
    }

    pub fn filename() -> &'static str {
        "config.toml"
    }

    pub fn save(&self) -> Result<()> {
        use std::io::prelude::*;

        // println!("{}",);
        let mut a = fs::File::create(Config::path())?;
        a.write_all(&toml::to_vec(&self)?)?;
        Ok(())
    }
}

#[test]
fn s() {
    let _ = Config::load().save();
}
