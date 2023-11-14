//! Displays animated GIFs
//! uses iced_gif

use super::Collection;
use super::Element;

use data::config::config_dir;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;
use tracing::{error, info};

#[cfg(feature = "iced_gif")]
use iced_gif::gif;

pub static GIF: Lazy<Animation> = Lazy::new(|| Animation::new());

pub struct Animation {
    #[cfg(feature = "iced_gif")]
    pub gifs: HashMap<&'static str, iced_gif::Frames>,
}

#[cfg(feature = "iced_gif")]
impl Animation {
    pub fn init_lazy(&self) {}

    pub fn ripping(&self) -> Option<iced_gif::Gif> {
        Some(iced_gif::gif(self.gifs.get("ripping").unwrap()))
    }

    pub fn idle(&self) -> Option<iced_gif::Gif> {
        Some(iced_gif::gif(self.gifs.get("idle").unwrap()))
    }

    /// Allow loading custom animations
    fn new() -> Self {
        let idle_gif = config_dir().join("idle.gif");
        let ripping_gif = config_dir().join("ripping.gif");

        let idle = Self::load(idle_gif).unwrap_or_else(|_| Self::default_idle());

        let ripping = Self::load(ripping_gif).unwrap_or_else(|_| Self::default_ripping());

        Self { gifs: HashMap::from([
            ("idle", idle),
            ("ripping", ripping),
        ]) }
    }   

    fn load(path: impl AsRef<Path>) -> anyhow::Result<iced_gif::Frames> {
        const MAX_SIZE: u64 = 2 * 1024 * 1024;

        if std::fs::metadata(path.as_ref())?.len() > MAX_SIZE {
            error!("Custom animation is over 2MB");
            todo!();
        }

        let result = gif::Frames::from_bytes(std::fs::read(path.as_ref())?);

        match result.as_ref().err() {
            None => info!("Loaded custom animation!"),
            Some(e) => error!("Failed to load custom animation: {}", e),
        };

        Ok(result?)
    }

    fn default_idle() -> iced_gif::Frames {
        gif::Frames::from_bytes(include_bytes!("../../assets/img/gif/white_lie_8fps.gif").to_vec())
            .unwrap()
    }

    fn default_ripping() -> iced_gif::Frames {
        gif::Frames::from_bytes(include_bytes!("../../assets/img/gif/white_walk_8fps.gif").to_vec())
            .unwrap()
    }
}

#[cfg(not(feature = "iced_gif"))]
impl Animation {
    pub fn idle<Message>(&self) -> Option<Element<Message>> {
        None
    }

    pub fn ripping<Message>(&self) -> Option<Element<Message>> {
        None
    }

    pub fn new() -> Self {
        Self {}
    }
}