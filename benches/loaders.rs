use xmodits_lib::*;
use once_cell::sync::Lazy;
use std::path::Path;

use tracker_formats::*;

type ModLoaderFunc = fn(&Path) -> Result<TrackerModule, XmoditsError>;

static LOADER: Lazy<[(&'static str, ModLoaderFunc); 4]> = Lazy::new (|| {
    [
        ("it", |p| ITFile::load_module(&p)),
        ("xm", |p| XMFile::load_module(&p)),
        ("s3m", |p| S3MFile::load_module(&p)),
        ("mod", |p| MODFile::load_module(&p)),
    ]
});

use std::collections::HashMap;
static LOADER_HASH: Lazy<HashMap<&'static str, ModLoaderFunc>> = Lazy::new (|| {
    let b: [(&str, ModLoaderFunc);4] = [
        ("it", |p| ITFile::load_module(&p)),
        ("xm", |p| XMFile::load_module(&p)),
        ("s3m", |p| S3MFile::load_module(&p)),
        ("mod", |p| MODFile::load_module(&p)),
        // ("umx", |p| UMXFile::load_module(&p)),
    ];
    HashMap::from(b)
});

use phf::phf_map;
static LOADER_ORDERED_MAP: phf::Map<&str, ModLoaderFunc> = phf_map! {
    "it" => |p| ITFile::load_module(&p),
    "xm" => |p| XMFile::load_module(&p),
    "s3m" => |p| S3MFile::load_module(&p),
    "mod" => |p| MODFile::load_module(&p),
};

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
            for (_, backup_loader) in LOADER.iter().filter(|e| e.0 != ext.as_str()&& e.0 != "mod") {
                if let Ok(tracker) = backup_loader(path.as_ref()) {
                    return Ok(tracker);
                }
            }
            Err(original_err)
        }
    }
}


pub fn load_module_test<P>(path: P) -> Result<TrackerModule, XmoditsError> 
where P: AsRef<std::path::Path>
{
    let path = path.as_ref();
    let ext = file_extension(&path).to_lowercase();

    match LOADER_ORDERED_MAP.get(ext.as_str()) {
        Some(mod_loader) => match mod_loader(path) {
            Ok(tracker) => Ok(tracker),
            Err(original_err) => {
                for (_, backup_loader) in LOADER_ORDERED_MAP.entries().filter(|k| k.0 != &ext.as_str()&& k.0 != &"mod") {
                    if let Ok(tracker) = backup_loader(path) {
                        return Ok(tracker);
                    }
                }
                Err(original_err)
            }
        },
        None => Err(XmoditsError::UnsupportedFormat(format!("'{}' is not a supported format.", ext)))
    }
}

pub fn load_module_test_hash<P>(path: P) -> Result<TrackerModule, XmoditsError> 
where P: AsRef<std::path::Path>
{
    let path = path.as_ref();
    let ext = file_extension(&path).to_lowercase();

    match LOADER_HASH.get(ext.as_str()) {
        Some(mod_loader) => match mod_loader(path) {
            Ok(tracker) => Ok(tracker),
            Err(original_err) => {
                for (_, backup_loader) in LOADER_HASH.iter().filter(|k| k.0 != &ext.as_str() && k.0 != &"mod") {
                    if let Ok(tracker) = backup_loader(path) {
                        return Ok(tracker);
                    }
                }
                Err(original_err)
            }
        },
        None => Err(XmoditsError::UnsupportedFormat(format!("'{}' is not a supported format.", ext)))
    }
}

// Function to get file extension from path.
fn file_extension<P: AsRef<std::path::Path>>(p: P) -> String {
    (match p.as_ref().extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap_or(""),
    }).to_string()
}