mod error;
mod fmt;
mod interface;
mod utils;
mod loader;

pub use error::XmoditsError;
pub use interface::SampleNamerFunc;
pub use interface::TrackerDumper;
pub use interface::TrackerModule;
pub use interface::TrackerSample;
pub use utils::name::SampleNamer;
pub use utils::wav;
pub use utils::Error;
pub use loader::load_from_ext;
pub use loader::load_module;
pub use loader::LOADERS;

use fmt::*;
pub mod tracker_formats {
    pub use crate::amig_mod::MODFile;
    pub use crate::it::ITFile;
    pub use crate::s3m::S3MFile;
    pub use crate::umx::UMXFile;
    pub use crate::xm::XMFile;
}
