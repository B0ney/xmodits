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

use std::collections::HashMap;
use std::path::Path;
pub use error::XmoditsError;

pub use utils::Error;
pub use formatter::SampleNamer;

use fmt::*;
use once_cell::sync::Lazy;

pub mod tracker_formats {
    pub use crate::it::ITFile;
    pub use crate::xm::XMFile;
    pub use crate::amig_mod::MODFile;
    pub use crate::s3m::S3MFile;
    pub use crate::umx::UMXFile;
}

type ModLoaderFunc = fn(&Path) -> Result<TrackerModule, XmoditsError>;

// static LOADER: Lazy<HashMap<&str, ModLoaderFunc>> = Lazy::new(|| {
static LOADER: Lazy<[(&str, ModLoaderFunc);4]> = Lazy::new(|| {
    use tracker_formats::*;
    let b: [(&str, ModLoaderFunc);4] = [
        ("it",|p| ITFile::load_module(&p)),
        ("xm", |p| XMFile::load_module(&p)),
        ("s3m", |p| S3MFile::load_module(&p)),
        ("mod", |p| MODFile::load_module(&p)),
        // ("umx", |p| UMXFile::load_module(&p)),
    ];
    // HashMap::from(b)
    b
});

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
    let path = path.as_ref();
    
    let a = match ext.as_str() {
        "it"    => ITFile::load_module(path),
        "xm"    => XMFile::load_module(path),
        "s3m"   => S3MFile::load_module(path),
        "mod"   => MODFile::load_module(path),
        f   => return Err(XmoditsError::UnsupportedFormat(
                format!("'{}' is not a supported format.", f)
            )
        ),
    };

    match a {
        Ok(a) => Ok(a),

        Err(original_err) => {
            for (_, backup_loader) in LOADER.iter().filter(|e| e.0 != ext.as_str()) {
                if let Ok(tracker) = backup_loader(path) {
                    return Ok(tracker);
                } else {
                    continue
                }
            }
            Err(original_err)
        }
    }
}
    
//     // Slower
//     match LOADER.get(ext.as_str()) {
//         Some(mod_loader) => match mod_loader(path) {
//             Ok(tracker) => Ok(tracker),
//             Err(original_err) => {
//                 for (_, backup_loader) in LOADER.iter().filter(|k| k.0 != &ext.as_str() && k.0 != &"mod") {
//                     if let Ok(tracker) = backup_loader(path) {
//                         return Ok(tracker);
//                     } else {
//                         continue
//                     }
//                 }
//                 Err(original_err)
//             }
//         },
//         None => Err(XmoditsError::UnsupportedFormat(format!("'{}' is not a supported format.", ext)))
//     }
// }

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
    // let a= load_module("./tests/mods/xm/DEADLOCK.s3m");
    let a= load_module("./tests/mods/xm/invalid.xm");

    if let Err(a) = a {
        dbg!(a);
    }
    dbg!();
}