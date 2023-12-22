//! Provide information about the application

use crate::widget::Element;

#[cfg(feature = "built")]
mod build_info_inner {
    use std::{collections::HashMap, path::PathBuf};

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
        ])
    });

    pub fn info(verbose: bool) -> impl Iterator<Item = (&'static str, &'static str)> {
        let rustc = if verbose { "Rustc version" } else { "With" };
        let git = if verbose { "git" } else { "git_short" };
        let features = if verbose { "features" } else { "" };

        [
            ("Built", "build_time"),
            (rustc, "rustc"),
            ("Git", git),
            ("Features", features),
            ("License", "license"),
        ]
        .into_iter()
        .filter_map(|(label, key)| Some((label, *INFO.get(key)?)))
    }

    pub fn view<'a, Message: 'a>() -> Option<Element<'a, Message>> {
        let information = info(false)
            .fold(column![].spacing(4), |col, (label, value)| {
                col.push(text(format!("{label}: {value}")).size(12))
            });

        Some(container(scrollable(information)).into())
    }

    pub async fn export_build(path: PathBuf) -> Result<(), String> {
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
pub async fn export_build(_path: std::path::PathBuf) -> Result<(), String> {
    Ok(())
}
