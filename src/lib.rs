#[allow(dead_code)]
mod it;
mod modtrk;
mod xm;
#[allow(dead_code)]
mod s3m;
mod umx;
#[allow(unused, dead_code)]
mod utils;
mod interface;
mod error;
pub use error::XmoditsError;
pub use interface::{TrackerDumper, TrackerModule, TrackerSample};
pub use utils::Error;


pub mod tracker_formats {
    pub use crate::it::ITFile;
    pub use crate::xm::XMFile;
    pub use crate::modtrk::MODFile;
    pub use crate::s3m::S3MFile;
    pub use crate::umx::UMXFile;
}