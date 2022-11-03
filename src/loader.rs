use crate::tracker_formats::*;
use crate::TrackerDumper;
use crate::XmoditsError;
use crate::TrackerModule;

type ModLoaderFunc = fn(Vec<u8>) -> Result<TrackerModule, XmoditsError>;
type ModValidatorFunc = fn(&[u8]) -> Result<(), XmoditsError>;

use phf::phf_map;

const MAX_FILESIZE_MB: u64 = 1024 * 1024 * 64;

pub static LOADERS: phf::Map<&str, (ModValidatorFunc, ModLoaderFunc)> = phf_map! {
    "it" => (|p| ITFile::validate(p), ITFile::load_from_buf_unchecked),
    "xm" => (|p| XMFile::validate(p), XMFile::load_from_buf_unchecked),
    "s3m" => (|p| S3MFile::validate(p), S3MFile::load_from_buf_unchecked),
    "umx" => (|p| UMXFile::validate(p), UMXFile::load_from_buf_unchecked),
    "mod" => (|p| MODFile::validate(p), MODFile::load_from_buf_unchecked),
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

pub fn load_from_ext<P>(path: P, ext: &str) -> Result<TrackerModule, XmoditsError>
where
    P: AsRef<std::path::Path>,
{
    let buf: Vec<u8> = load_to_buf(path)?;
    
    match LOADERS.get(ext) {
        Some((validator, loader)) => {
            if let Err(original_err) = validator(&buf) {
                for (_, (validator_bak, loader_bak)) in
                    LOADERS.entries().filter(|(k, _)| !["mod", ext].contains(k))
                {
                    if validator_bak(&buf).is_ok() {
                        return loader_bak(buf);
                    }
                }
                Err(original_err)
            } else {
                loader(buf)
            }
        },
        None => Err(XmoditsError::UnsupportedFormat(format!(
            "'{}' is not a supported format.",
            ext
        ))),
    }

}

fn load_to_buf<P>(path: P) -> Result<Vec<u8>, XmoditsError> 
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

// Function to get file extension from path.
fn file_extension<P: AsRef<std::path::Path>>(p: P) -> String {
    (match p.as_ref().extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap_or(""),
    })
    .to_string()
}
