/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::{PathBuf, Path};
use xmodits_lib::{SampleNamerFunc, XmoditsError};

/// Checks if the last element in paths is a folder that exists.
/// 
/// If not, it will create that folder.
/// 
/// If the last element is a file, the destination directory is the
/// current working directory.
pub fn destination_dir(paths: &mut Vec<PathBuf>) -> Result<PathBuf, String> {
    let cwd = || Ok(std::env::current_dir().expect("xmodits needs a current working directory. (>_<)"));
    
    let Some(path) = paths.last() else {
        return cwd();
    };

    // Make sure path is NOT a file, and the length is over 1
    if path.is_file() || paths.len() <= 1 {
        return cwd();
    }

    let folder = paths.pop().unwrap();

    if !folder.is_dir() {
        if let Err(e) = std::fs::create_dir(&folder) {
            return Err(format!(
                "Error: Could not create destination folder \"{}\": {}",
                folder.display(),
                e
            ));
        };
    }

    Ok(folder)
}


pub fn mod_name<P: AsRef<Path>>(mod_path: P) -> String {
    mod_path
        .as_ref()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace('.', "_")
}

pub fn folder<P: AsRef<Path>>(destination: P, path: P, with_folder: bool) -> PathBuf {
    match with_folder {
        true => {
            let modname: String = mod_name(path);
            let new_folder: PathBuf = destination.as_ref().join(modname);

            new_folder
        }
        _ => destination.as_ref().to_path_buf(),
    }
}

pub fn total_size_megabytes(paths: &[PathBuf]) -> f64 {
    paths
        .iter()
        .map(|e| 
            match e.metadata() {
                Ok(m) => m.len() as f64,
                _ => 0.0
            }
        )
        .sum::<f64>()
        / (1024.0 * 1024.0)
}

pub fn dump_samples<T, U>(mod_path: T, dest_dir: U) -> Result<(), XmoditsError>
where
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    let folder = folder(dest_dir.as_ref(), mod_path.as_ref(), true);

    if folder.exists() {
        return Err(XmoditsError::FileError(format!(
            "Folder already exists: {}",
            &folder.display()
        )));
    }

    xmodits_lib::load_module(mod_path)?.dump(&folder, true)
}

pub fn dump_samples_advanced<T, U>(
    mod_path: T,
    dest_dir: U,
    sample_namer: &SampleNamerFunc,
    with_folder: bool,
    hint: &Option<String>,
) -> Result<(), XmoditsError>
where
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    if dest_dir.as_ref().is_dir() && with_folder {
        return Err(XmoditsError::FileError(format!(
            "Folder already exists: {}",
            &dest_dir.as_ref().display()
        )));
    }

    let mut tracker = match hint {
        Some(hint) => xmodits_lib::load_from_ext(mod_path, hint)?,
        None => xmodits_lib::load_module(mod_path)?,
    };

    tracker.dump_advanced(&dest_dir, sample_namer, with_folder)
}
