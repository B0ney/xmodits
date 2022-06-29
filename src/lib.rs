mod it;
mod modtrk;
mod xm;
mod s3m;
mod umx;
mod utils;
mod interface;

pub use interface::{TrackerDumper, TrackerModule};
pub use utils::Error;

pub mod tracker_formats {
    pub use crate::it::ITFile;
    pub use crate::modtrk::MODFile;
    pub use crate::s3m::S3MFile;
    pub use crate::xm::XMFile;
    pub use crate::umx::UMXFile;
}

// pub mod exotic_formats {

// }

