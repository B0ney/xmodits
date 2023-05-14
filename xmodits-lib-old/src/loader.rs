/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::tracker_formats::*;
use crate::TrackerDumper;
use crate::TrackerModule;
use crate::XmoditsError;
use phf::phf_ordered_map;

const MAX_FILESIZE_MB: u64 = 1024 * 1024 * 64;

type ModLoaderFunc = fn(Vec<u8>) -> Result<TrackerModule, XmoditsError>;
type ModValidatorFunc = fn(&[u8]) -> Result<(), XmoditsError>;

fn validate<T: TrackerDumper>(buf: &[u8]) -> Result<(), XmoditsError> {
    T::validate(buf)
}

fn load<T: TrackerDumper>(buf: Vec<u8>) -> Result<TrackerModule, XmoditsError> {
    T::load_from_buf_unchecked(buf)
}

/// An ordered hashmap 
/// 
/// Key: format file extension
/// 
/// Value: (validator, loader)
pub static LOADERS: phf::OrderedMap<&str, (ModValidatorFunc, ModLoaderFunc)> = phf_ordered_map! {
    "it" => (validate::<ITFile>, load::<ITFile>),
    "xm" => (validate::<XMFile>, load::<XMFile>),
    "s3m" => (validate::<S3MFile>, load::<S3MFile>),
    "umx" => (validate::<UMXFile>, load::<UMXFile>),
    "mptm" => (validate::<ITFile>, load::<ITFile>),
    // MOD has the least validations, so we put this last.
    // This is why we made it an ordered hashmap.
    "mod" => (validate::<MODFile>, load::<MODFile>),
};

/// A more robust method to load a module gven a path.
///
/// Load a module given a file extension.
///
/// If it fails, loop through other module loaders, return if one succeeds.
pub fn load_module<P>(path: P) -> Result<TrackerModule, XmoditsError>
where
    P: AsRef<std::path::Path>,
{
    let ext = file_extension(&path).to_lowercase();
    load_from_ext(path, &ext)
}

/// Load a module given a path and file extension.
/// 
/// 
pub fn load_from_ext<P>(path: P, ext: &str) -> Result<TrackerModule, XmoditsError>
where
    P: AsRef<std::path::Path>,
{
    // Check if file extension exists in hashmap.
    //
    // If it exists, we obtain two function objects:
    // 1) function to validate a particular format.
    // 2) function to load the file. (Takes ownership of buffer)
    let Some((validator, loader)) = LOADERS.get(ext) else {
        return Err(XmoditsError::UnsupportedFormat(format!(
            "'{}' is not a supported format.",
            ext
        )))
    };

    // After obtaining the function objects,
    // load the file into memory
    let buf: Vec<u8> = load_to_buf(path)?;

    // Validate the loaded file.
    // If validation succeeds, early return the loaded module.
    let Err(original_err) = validator(&buf) else {
        return loader(buf)
    };

    // If validation fails, hold the original error so that we may return it if all else fails.
    // Iterate through all the (validator, loaders) in hashmap and attempt to load it.
    // 
    // Exclude the current extension to prevent an infinite loop.
    // Exclude "mod" because it has little to no validation. Might be removed in future.
    // Exclude "mptm" becaues it's identical to "it".
    for (_, (validator_bak, loader_bak)) in
        LOADERS.entries().filter(|(k, _)| !["mptm", "mod", ext].contains(k))
    {
        if validator_bak(&buf).is_ok() {
            return loader_bak(buf);
        }
    }

    // Return original error if we cannot find a suitable format.
    Err(original_err)
}

/// Read file contents into a ```Vec<u8>```
/// 
/// Traditionally, we'd use file objects + BufReader,
/// but this is fine for two reasons:
///
/// 1) Tracker Modules are very small.
/// 2) 90% of its size are raw samples that need to be processed anyway.
///  
pub fn load_to_buf<P>(path: P) -> Result<Vec<u8>, XmoditsError>
where
    P: AsRef<std::path::Path>,
{
    if std::fs::metadata(&path)?.len() > MAX_FILESIZE_MB {
        return Err(XmoditsError::file(
            "File provided is larger than 64MB. No tracker module should ever be close to that",
        ));
    }

    Ok(std::fs::read(&path)?)
}

/// Function to get file extension from path.
pub fn file_extension<P: AsRef<std::path::Path>>(p: P) -> String {
    p.as_ref()
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}
