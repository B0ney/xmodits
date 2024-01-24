//! Provide information about the application

use crate::widget::Element;

#[cfg(feature = "built")]
mod build_info_inner {
    use std::{collections::HashMap, path::Path};

    use super::Element;
    use iced::widget::{column, container, scrollable, text};
    use once_cell::sync::Lazy;
    use tokio::{fs::File, io::AsyncWriteExt};

    pub mod info {
        include!(concat!(env!("OUT_DIR"), "/built.rs"));
    }

    static INFO: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
        HashMap::from_iter([
            ("build_time", info::BUILT_TIME_UTC),
            ("rustc", info::RUSTC_VERSION),
            ("git", info::GIT_COMMIT_HASH.unwrap_or("None")),
            ("git_short", info::GIT_COMMIT_HASH_SHORT.unwrap_or("None")),
            ("features", info::FEATURES_LOWERCASE_STR),
            ("license", info::PKG_LICENSE),
            ("architecture", info::TARGET),
        ])
    });

    pub fn info(verbose: bool) -> impl Iterator<Item = (&'static str, &'static str)> {
        let built = if verbose { "Date of build" } else { "Built" };
        let rustc = if verbose { "Rustc version" } else { "With" };
        let architecture = if verbose { "Architecture" } else { "Target" };
        let git = if verbose { "git" } else { "git_short" };
        let features = if verbose { "features" } else { "" };

        [
            (built, "build_time"),
            (rustc, "rustc"),
            (architecture, "architecture"),
            ("Git", git),
            ("Features", features),
            ("License", "license"),
        ]
        .into_iter()
        .filter_map(|(label, key)| Some((label, *INFO.get(key)?)))
    }

    pub fn view<'a, Message: 'a>() -> Option<Element<'a, Message>> {
        let elem = |(label, value)| text(format!("{label}: {value}")).size(12).into();
        let info = scrollable(column(info(false).map(elem)).spacing(4));
        Some(container(info).into())
    }

    pub async fn export_build(path: &Path) -> Result<(), String> {
        let mut file = File::create(path).await.map_err(|f| f.to_string())?;

        for (label, value) in info(true) {
            file.write_all(format!("{label}: {value}\n").as_bytes())
                .await
                .map_err(|f| f.to_string())?;
        }

        Ok(())
    }
}

#[cfg(feature = "built")]
pub use build_info_inner::*;

#[cfg(not(feature = "built"))]
pub fn view<'a, Message: 'a>() -> Option<Element<'a, Message>> {
    None
}

#[cfg(not(feature = "built"))]
pub async fn export_build(_path: &std::path::Path) -> Result<(), String> {
    Ok(())
}

#[cfg(not(feature = "built"))]
pub fn info(_verbose: bool) -> impl Iterator<Item = (&'static str, &'static str)> {
    std::iter::empty()
}