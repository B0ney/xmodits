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

// struct ModLoader<'a>{
//     loaders: HashMap<&'a str, ModLoaderFunc>
// }

// impl <'a>ModLoader<'a> {
//     fn add_user_function(&mut self, ext: &'a str, func: ModLoaderFunc) {
//         self.loaders.insert(ext, func);
//     }

//     fn load<P: AsRef<std::path::Path>>(&self, path: P) -> Result<TrackerModule, XmoditsError> {
//         let ext = file_extension(&path).to_lowercase();

//         match self.loaders.get(ext.as_str()) {
//             Some(mod_loader) => {
//                 if let Ok(tracker) = mod_loader(path.as_ref()) {
//                     Ok(tracker)
//                 } else {
//                     for (_, backup_loader) in self.loaders.iter().filter(|k| k.0 != &ext.as_str()) {
//                         if let Ok(tracker) = backup_loader(&path.as_ref()) {
//                             return Ok(tracker);
//                         } else {
//                             continue
//                         }
//                     }
//                     return Err(XmoditsError::UnsupportedFormat(
//                         format!("Could not determine a valid format from {}", path.as_ref().display())
//                     ));
//                 }
//             },
//             None => return Err(XmoditsError::UnsupportedFormat(format!("'{}' is not a supported format.", ext)))
//         }
// }
// }

// https://stackoverflow.com/questions/65157092/how-to-construct-a-hashmap-with-boxed-fn-values

pub fn identify_and_load<P>(path: P) -> Result<TrackerModule, XmoditsError> 
where P: AsRef<std::path::Path>
{
    use tracker_formats::*;
    use std::collections::HashMap;

    type ModLoaderFunc= Box<dyn Fn(&Path) -> Result<TrackerModule, XmoditsError>>;

    let b: [(&str,ModLoaderFunc); 4]  = [
        ("it", Box::new(ITFile::load_module)),
        ("xm", Box::new(XMFile::load_module)),
        ("s3m", Box::new(S3MFile::load_module)),
        ("mod", Box::new(MODFile::load_module)),
    ];

    let loaders = HashMap::from(b);

    let ext = file_extension(&path).to_lowercase();

    match loaders.get(ext.as_str()) {
        Some(mod_loader) => {
            if let Ok(tracker) = mod_loader(path.as_ref()) {
                Ok(tracker)
            } else {
                for (_, backup_loader) in loaders.iter().filter(|k| k.0 != &ext.as_str()) {
                    if let Ok(tracker) = backup_loader(&path.as_ref()) {
                        return Ok(tracker);
                    } else {
                        continue
                    }
                }
                return Err(XmoditsError::UnsupportedFormat(
                    format!("Could not determine a valid format from {}", path.as_ref().display())
                ));
            }
        },
        None => return Err(XmoditsError::UnsupportedFormat(format!("'{}' is not a supported format.", ext)))
    }

    
}


    // let mut a = ModLoader{ loaders: HashMap::new() };
    // a.add_user_function("it", Box::new(ITFile::load_module) as ModLoaderFunc);


    // todo!()
    // a.load(path)

// }


// Function to get file extension from path.
fn file_extension<P: AsRef<std::path::Path>>(p: P) -> String {
    (match p.as_ref().extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap_or(""),
    }).to_string()
}