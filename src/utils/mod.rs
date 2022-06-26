pub mod macros;
pub mod signed;
pub mod array;
pub mod wav;
pub type Error = Box<dyn std::error::Error>;

/// Import useful helper functions & macros
/// and common imports
/// 
/// **Common Imports:**
/// 
/// ```rust
/// std::fs::{self, File};
/// std::io::Write;
/// std::path::Path;
/// ```
/// 
/// **Helper funcion modules**:
/// 
/// ```rust
/// crate::utils::Error;
/// crate::utils::wav;
/// crate::utils::signed::SignedByte;
/// crate::utils::array::load_to_array;
/// ```
/// 
/// **Macros:**
/// 
/// ```rust
/// crate::offset_u16;
/// crate::offset_u32;
/// crate::offset_chars;
/// ```
/// 
pub mod prelude {
    // Bulk common imports
    pub use std::fs::{self, File};
    pub use std::io::Write;
    pub use std::path::Path;

    // Import helper functions
    pub use super::wav;
    pub use super::signed::SignedByte;
    pub use super::array::load_to_array;
    pub use super::Error;

    // Import macros
    pub use crate::offset_u16;
    pub use crate::offset_u32;
    pub use crate::offset_chars;
}