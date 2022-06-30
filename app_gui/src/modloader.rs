use xmodits_lib::{Error,TrackerDumper, TrackerModule, tracker_formats::*};
use std::path::Path;

// duplicate function 
// Function to get file extension from path.
fn file_extension<P:AsRef<Path>>(p: P) -> String {
    (match p.as_ref().extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap_or(""),
    }).to_string()
}


pub fn load_module<P:AsRef<Path>>(mod_path: P) -> Result<TrackerModule, Error> {
    let hint: String = file_extension(&mod_path).to_lowercase();

    match hint.as_str() {
        "it"    => ITFile::load_module(mod_path),
        "s3m"   => S3MFile::load_module(mod_path),
        "mod"   => MODFile::load_module(mod_path),
        // "umx"   => UMXFile::load_module(mod_path),
        // "xm"    => XMFile::load_module(mod_path),
        f       => return Err(format!("'{}' is not a supported format.", f).into()),
    }
}
