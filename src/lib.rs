mod fmt;
#[allow(unused, dead_code)]
mod utils;
mod interface;
mod formatter;
mod error;

pub use interface::SampleNamerFunc;
pub use interface::TrackerSample;
pub use interface::TrackerModule;
pub use interface::TrackerDumper;
pub use error::XmoditsError;

pub use utils::Error;
pub use formatter::SampleNamer;

use std::path::Path;
use fmt::*;

pub mod tracker_formats {
    pub use crate::it::ITFile;
    pub use crate::xm::XMFile;
    pub use crate::amig_mod::MODFile;
    pub use crate::s3m::S3MFile;
    pub use crate::umx::UMXFile;
}

type ModLoaderFunc = fn(&Path) -> Result<TrackerModule, XmoditsError>;

lazy_static::lazy_static! {
    static ref LOADER: [(&'static str, ModLoaderFunc); 4] = {
        use tracker_formats::*;
        [
            ("it", |p| ITFile::load_module(&p)),
            ("xm", |p| XMFile::load_module(&p)),
            ("s3m", |p| S3MFile::load_module(&p)),
            ("mod", |p| MODFile::load_module(&p)),
            // ("umx", |p| UMXFile::load_module(&p)),
        ]
    };
}

/// A more robust method to load a module gven a path.
/// 
/// Load a module given a file extension.
/// 
/// If it fails, loop through other module loaders, return if one succeeds.
pub fn load_module<P>(path: P) -> Result<TrackerModule, XmoditsError> 
where P: AsRef<std::path::Path>
{
    use tracker_formats::*;

    let ext = file_extension(&path).to_lowercase();
    
    let a = match ext.as_str() {
        "it"    => ITFile::load_module(&path),
        "xm"    => XMFile::load_module(&path),
        "s3m"   => S3MFile::load_module(&path),
        "mod"   => MODFile::load_module(&path),
        f   => return Err(XmoditsError::UnsupportedFormat(
                format!("'{}' is not a supported format.", f)
            )
        ),
    };

    match a {
        Ok(a) => Ok(a),

        Err(original_err) => {
            for (_, backup_loader) in LOADER.iter().filter(|e| e.0 != ext.as_str() && e.0 != "mod") {
                if let Ok(tracker) = backup_loader(path.as_ref()) {
                    return Ok(tracker);
                } else {
                    continue
                }
            }
            Err(original_err)
        }
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
    let a= load_module("./tests/mods/xm/DEADLOCK.ooga");
    // let a= load_module("./tests/mods/xm/invalid.xm");

    if let Err(a) = a {
        dbg!(a);
    }
    dbg!();
}