use std::path::{Path, PathBuf};
use xmodits_lib::{Error, TrackerDumper, TrackerModule, tracker_formats::*,};

// Function to get file extension from path.
fn file_extension<P: AsRef<Path>>(p: P) -> String {
    (match p.as_ref().extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap_or(""),
    }).to_string()
}

pub fn dump_samples(mod_path: &PathBuf, dest_dir: &PathBuf) -> Result<(), Error> {
    let hint: String    = file_extension(&mod_path).to_lowercase();
    let modname: String = mod_path.file_name().unwrap().to_str().unwrap().replace(".", "_");
    let folder = dest_dir.join(modname);
    if folder.exists() {
        return Err(format!("'{}' Directory already exists", f).into());
    } else {
        std::fs::create_dir(&folder)
    }
    todo!("Reminder: We are no longer creating an independent folder");
    match hint.as_str() {
        "it"    => ITFile::load_module(mod_path),
        "xm"    => XMFile::load_module(mod_path),
        "s3m"   => S3MFile::load_module(mod_path),
        "mod"   => MODFile::load_module(mod_path),
        // "umx"   => UMXFile::load_module(mod_path),
        f       => return Err(format!("'{}' is not a supported format.", f).into()),
    }?.dump(&folder)
}