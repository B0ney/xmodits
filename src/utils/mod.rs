pub mod macros;
pub mod signed;
pub mod array;
pub mod wav;

/// Import useful helper functions & macros
pub mod prelude {
    pub use super::wav;
    pub use super::signed::SignedByte;
    pub use super::array::load_to_array;
    pub use super::Error;
    // Import useful macros
    pub use crate::offset_u16;
    pub use crate::offset_u32;
    pub use crate::offset_chars;
}

pub type Error = Box<dyn std::error::Error>;