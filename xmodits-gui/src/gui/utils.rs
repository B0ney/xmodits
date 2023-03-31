use std::path::{PathBuf, Path};

use super::Info;

pub fn file_name(path: impl AsRef<Path>) -> String {
    path
        .as_ref()
        .file_name()
        .map(|f| f.to_string_lossy())
        .unwrap_or_default()
        .into()
}

pub async fn folder_dialog() -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .pick_folder()
        .await
        .map(|f| f.path().to_owned())
}

pub async fn folders_dialog() -> Option<Vec<PathBuf>> {
    paths(rfd::AsyncFileDialog::new().pick_folders().await)
}

pub async fn files_dialog() -> Option<Vec<PathBuf>> {
    paths(rfd::AsyncFileDialog::new().pick_files().await)
}

fn paths(h: Option<Vec<rfd::FileHandle>>) -> Option<Vec<PathBuf>> {
    h.map(|filehandles| {
        filehandles
            .into_iter()
            .map(|d| d.path().to_owned())
            .collect()
    })
}

pub async fn tracker_info(path: PathBuf) -> Option<Info> {
    let path2 = path.clone();

    let (tracker_result) = tokio::task::spawn_blocking(|| {
        let mut file = std::fs::File::open(path2)?;
        xmodits_lib::fmt::loader::load_module(&mut file)
    })
    .await
    .ok()?;

    match tracker_result {
        Ok(tracker) => Some(Info::valid(tracker, path)),
        Err(error) => Some(Info::invalid(error.to_string(), path)),
    }
}