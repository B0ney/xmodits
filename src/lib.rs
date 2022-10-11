mod fmt;
use std::{path::{Path, PathBuf}, collections::HashMap, hash::Hash};

use fmt::*;
#[allow(unused, dead_code)]
mod utils;
mod interface;
mod formatter;
mod error;

pub use interface::{TrackerDumper, TrackerModule, TrackerSample, SampleNamerFunc};
pub use error::XmoditsError;
use once_cell::sync::Lazy;
pub use utils::Error;
pub use formatter::SampleNamer;

pub mod tracker_formats {
    pub use crate::it::ITFile;
    pub use crate::xm::XMFile;
    pub use crate::amig_mod::MODFile;
    pub use crate::s3m::S3MFile;
    pub use crate::umx::UMXFile;
}

/// Load a tracker module based on file extension
pub fn load_module<P: AsRef<std::path::Path>>(path: P) -> Result<TrackerModule, XmoditsError> {
    use tracker_formats::*;
    match file_extension(&path).to_lowercase().as_str() {
        "it"    => ITFile::load_module(path),
        "xm"    => XMFile::load_module(path),
        "s3m"   => S3MFile::load_module(path),
        "mod"   => MODFile::load_module(path),
        f   => Err(
            XmoditsError::UnsupportedFormat(
                format!("'{}' is not a supported format.", f)
            )
        ),
    }
}

// https://stackoverflow.com/questions/65157092/how-to-construct-a-hashmap-with-boxed-fn-values

type ModLoaderFunc = fn(&Path) -> Result<TrackerModule, XmoditsError>;

static LOADER: Lazy<HashMap<&str, ModLoaderFunc>> = Lazy::new(|| {
    use tracker_formats::*;
    let b: [(&str, ModLoaderFunc); 4] = [
        ("it",|p| ITFile::load_module(&p)),
        ("xm", |p| XMFile::load_module(&p)),
        ("s3m", |p| S3MFile::load_module(&p)),
        ("mod", |p| MODFile::load_module(&p)),
    ];
    HashMap::from(b)
});

pub fn load_module_robust<P>(path: P) -> Result<TrackerModule, XmoditsError> 
where P: AsRef<std::path::Path>
{
    let ext = file_extension(&path).to_lowercase();
    let path = path.as_ref();

    match LOADER.get(ext.as_str()) {
        Some(mod_loader) => {
            if let Ok(tracker) = mod_loader(&path) {
                Ok(tracker)
            } else {
                for (extension, backup_loader) in LOADER.iter().filter(|k| k.0 != &ext.as_str()) {
                    if let Ok(tracker) = backup_loader(&path) {
                        println!("Detected: {}", extension);
                        return Ok(tracker);
                    } else {
                        continue
                    }
                }
                return Err(XmoditsError::UnsupportedFormat(
                    format!("Could not determine a valid format from {}", path.display() )
                ));
            }
        },
        None => return Err(XmoditsError::UnsupportedFormat(format!("'{}' is not a supported format.", ext)))
    }
}

// Function to get file extension from path.
fn file_extension<P: AsRef<std::path::Path>>(p: P) -> String {
    (match p.as_ref().extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap_or(""),
    }).to_string()
}

// Hashmaps are not sorted
#[test]
fn robust() {
    let a= load_module_robust("./tests/mods/xm/DEADLOCK.s3m").unwrap();

    dbg!();
}