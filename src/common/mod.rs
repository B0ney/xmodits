/*
* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::path::{Path, PathBuf};
use crate::{SampleNamerFunc, XmoditsError, load_module, load_from_ext};

pub fn mod_name<P>(mod_path: P) -> String 
where 
    P: AsRef<Path>
{
    mod_path
        .as_ref()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace('.', "_")
}

pub fn filename<P>(path: P) -> String 
where
    P: AsRef<Path>
{
    path
        .as_ref()
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_default()
}

pub fn folder<P, Q>(destination: P, path: Q, with_folder: bool) -> PathBuf
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    match with_folder {
        true => {
            let modname: String = mod_name(path);
            let new_folder: PathBuf = destination.as_ref().join(modname);

            new_folder
        }
        _ => destination.as_ref().to_path_buf(),
    }
}

pub fn total_size_megabytes<P>(paths: &[P]) -> f64 
where 
    P: AsRef<Path>
{
    paths
        .iter()
        .filter_map(|e| match e.as_ref().metadata().ok() {
            Some(meta) => Some(meta.len() as f64),
            _ => None
        })
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

    load_module(mod_path)?.dump(&folder, true)
}

pub fn dump_samples_advanced<T, U>(
    mod_path: T,
    dest_dir: U,
    sample_namer: &SampleNamerFunc,
    with_folder: bool,
    hint: &Option<String>,
    with_loop_points: bool,
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
        Some(hint) => load_from_ext(mod_path, hint)?,
        None => load_module(mod_path)?,
    };

    tracker.dump_advanced(&dest_dir, sample_namer, with_folder, with_loop_points)
}
