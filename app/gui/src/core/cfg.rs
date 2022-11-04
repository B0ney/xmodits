use std::{fs, path::PathBuf};

use anyhow::Result;
use dirs;
use serde::{Deserialize, Serialize};
use toml;

const APP_NAME: &str = "xmodits";

// User editable configuration
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub index_raw: bool,
    pub index_only: bool,
    pub index_padding: usize,
    pub upper: bool,
    pub lower: bool,
    pub no_folder: bool,
    pub destination: String,
}

pub fn config_dir() -> PathBuf {
    let config_dir = dirs::config_dir()
        .expect("There should be a config directory")
        .join(APP_NAME);

    if !config_dir.exists() {
        fs::create_dir(&config_dir)
            .expect("Couldn't create a config folder for xmodits") // TODO: replace with log warn
    }

    config_dir
}

impl Config {
    pub fn load() -> Self {
        let default_and_save = || {
            let a = Self::default();
            // let _ = a.save();
            a
        };
        default_and_save()

        // match fs::read_to_string(Config::path()) {
        //     Ok(j) => match toml::from_str::<Self>(&j) {
        //         Ok(s) => s,
        //         Err(_) => default_and_save(),
        //     },
        //     Err(_) => default_and_save(),
        // }
    }

    pub fn path() -> PathBuf {
        config_dir().join(Config::filename())
    }

    pub fn filename() -> &'static str {
        "config.toml"
    }

    pub fn save(&self) -> Result<()> {
        use std::io::prelude::*;
        let mut a = fs::File::create(Config::path())?;
        a.write_all(&toml::to_vec(&self)?)?;
        Ok(())
    }
}

#[test]
fn s() {
    let _ = Config::load().save();
}
