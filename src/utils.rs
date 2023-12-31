//! Utilility functions used throughout the codebase.

use std::path::{Path, PathBuf};

/// Returns filename of path
pub fn filename(path: &Path) -> &str {
    path.file_name().and_then(|f| f.to_str()).unwrap_or_default()
}

/// Returns path file extension
pub fn extension(path: &Path) -> &str {
    path.extension().and_then(|f| f.to_str()).unwrap_or_default()
}

pub async fn folder_dialog() -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .pick_folder()
        .await
        .map(|f| f.path().to_owned())
}

pub async fn folders_dialog() -> Option<Vec<PathBuf>> {
    rfd::AsyncFileDialog::new().pick_folders().await.map(paths)
}

pub async fn files_dialog() -> Option<Vec<PathBuf>> {
    rfd::AsyncFileDialog::new().pick_files().await.map(paths)
}

fn paths(handles: Vec<rfd::FileHandle>) -> Vec<PathBuf> {
    handles.into_iter().map(|d| d.path().to_owned()).collect()
}

pub async fn create_file_dialog(filename: String) -> Option<PathBuf> {
    let file_dialog = rfd::AsyncFileDialog::new();

    #[cfg(windows)]
    // I find that Windows is the only platform where file filters work as intended.
    let file_dialog = file_dialog.add_filter("", &["txt"]);

    file_dialog
        .set_file_name(filename)
        .save_file()
        .await
        .map(|handle| handle.path().to_owned())
}
