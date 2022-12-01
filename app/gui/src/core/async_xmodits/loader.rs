/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use xmodits_lib::{LOADERS, TrackerModule, XmoditsError};
use tokio::fs::{File, read, metadata};

const MAX_FILESIZE_MB: u64 = 1024 * 1024 * 64;

pub async fn load_module<P>(path: P) -> Result<TrackerModule, XmoditsError>
where
    P: AsRef<std::path::Path>,
{
    let ext = file_extension(&path).to_lowercase();
    load_from_ext(path, &ext).await
}

pub async fn load_from_ext<P>(path: P, ext: &str) -> Result<TrackerModule, XmoditsError>
where
    P: AsRef<std::path::Path>,
{
    let buf: Vec<u8> = load_to_buf(path).await?;

    let Some((validator, loader)) = LOADERS.get(ext) else {
        return Err(XmoditsError::UnsupportedFormat(format!(
            "'{}' is not a supported format.",
            ext
        )))
    };

    let Err(original_err) = validator(&buf) else {
        return loader(buf)
    };

    for (_, (validator_bak, loader_bak)) in
        LOADERS.entries().filter(|(k, _)| !["mod", ext].contains(k))
    {
        if validator_bak(&buf).is_ok() {
            return loader_bak(buf);
        }
    }

    Err(original_err)
}

pub async fn load_to_buf<P>(path: P) -> Result<Vec<u8>, XmoditsError>
where
    P: AsRef<std::path::Path>,
{
    if metadata(&path).await?.len() > MAX_FILESIZE_MB {
        return Err(XmoditsError::file(
            "File provided is larger than 64MB. No tracker module should ever be close to that",
        ));
    }

    Ok(read(&path).await?)
}

/// Function to get file extension from path.
fn file_extension<P: AsRef<std::path::Path>>(p: P) -> String {
    p.as_ref()
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

