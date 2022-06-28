use std::path::{Path, PathBuf};
use xmodits::{Error, TrackerDumper, DumperObject, tracker_formats::*,};

fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let p = "samples/s3m/city_on_a_stick.s3m";
    let a =PathBuf::new().join(p);

    if !a.is_file() {
        return Err("Path provided is not a file".into());
    }

    let hint: String = file_extension(&a);

    let module: DumperObject = match hint.as_str() {
        "it"    => ITFile::load_module(p),
        "s3m"   => S3MFile::load_module(p),
        "mod"   => MODFile::load_module(p),
        "umx"   => UMXFile::load_module(p),
        "xm"    => XMFile::load_module(p),
        _       => return Err("Could not determine format.".into()),
    }?;

    module.export(&"./test/", 0)?;
    println!("dumped!");
    Ok(())
} 

// Function to get file extension from path.
// 
fn file_extension<P:AsRef<Path>>(p: P) -> String {
    (match p.as_ref().extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap_or(""),
    }).to_string()
}