// mod app;
use std::path::{Path, PathBuf};
use xmodits::{Error, TrackerDumper, TrackerModule, tracker_formats::*,};
use clap::{Command, arg};

fn main() -> Result<(), Error> {
    let matches = Command::new("Xmodits")
        .about("Sample dumping tool for tracker modules")
        .version("0.1.0")
        .author("B0ney")
        .arg(
            arg!([module] "Path to tracker module")
                .required(true)
        )
        .arg(
            arg!(<out_dir> "Destination folder for dumped samples")
                .required(false)
        )
        .get_matches();
        
    let mod_path = match matches.get_one::<String>("module"){
        Some(path) => {
            PathBuf::new().join(path)
        },
        None => unimplemented!(),
    };

    let dest_dir = match matches.get_one::<String>("out_dir") {
        Some(dest) => {PathBuf::new().join(dest)},
        None => {std::env::current_dir()?}
    };

    if !mod_path.is_file() {
        return Err("Path provided either doesn't exist or is not a file".into());
    }

    let hint: String = file_extension(&mod_path);

    let module: TrackerModule = match hint.as_str() {
        "it"    => ITFile::load_module(mod_path),
        "s3m"   => S3MFile::load_module(mod_path),
        "mod"   => MODFile::load_module(mod_path),
        // "umx"   => UMXFile::load_module(mod_path),
        // "xm"    => XMFile::load_module(mod_path),
        f       => return Err(format!("'{}' is not a supported format.", f).into()),
    }?;

    module.dump(&dest_dir)?;

    Ok(())
} 

// Function to get file extension from path.
fn file_extension<P:AsRef<Path>>(p: P) -> String {
    (match p.as_ref().extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap_or(""),
    }).to_string()
}