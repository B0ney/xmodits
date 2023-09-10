use std::path::{Path, PathBuf};

/// Returns filename of path
pub fn filename(path: &Path) -> &str {
    path.file_name()
        .and_then(|f| f.to_str())
        .unwrap_or_default()
}

/// Returns path file extension
pub fn extension(path: &Path) -> &str {
    path.extension()
        .and_then(|f| f.to_str())
        .unwrap_or_default()
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

pub async fn create_file() -> Option<PathBuf> {
    let file_dialog = rfd::AsyncFileDialog::new();

    #[cfg(windows)]
    // I find that Windows is the only platform where file filters work as intended.
    let file_dialog = file_dialog.add_filter("", &["txt"]);

    file_dialog
        .save_file()
        .await
        .map(|handle| handle.path().to_owned())
}

fn paths(h: Option<Vec<rfd::FileHandle>>) -> Option<Vec<PathBuf>> {
    h.map(|filehandles| {
        filehandles
            .into_iter()
            .map(|d| d.path().to_owned())
            .collect()
    })
}
