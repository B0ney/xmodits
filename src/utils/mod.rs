pub mod macros;
pub mod signed;
// pub mod array;
pub mod wav;
pub mod ascii;
pub mod name;
pub mod reader;

pub type Error = Box<dyn std::error::Error>;
/*
/ Import useful helper functions & macros
/ and common imports
/ 
/ **Common Imports:**
/ 
/ ```rust
/ std::fs::{self, File};
/ std::io::Write;
/ std::path::Path;
/ std::path::PathBuf;
/ ```
/ 
/ **Helper funcion modules**:
/ 
/ ```rust
/ crate::utils::Error;
/ crate::utils::wav;
/ crate::utils::signed::SignedByte;
/ crate::utils::array::load_to_array;
/ ```
/ 
/ **Macros:**
/ 
/ ```rust
/ crate::offset_u16;
/ crate::offset_u32;
/ crate::offset_chars;
/ ```
*/
#[allow(unused)]
pub mod prelude {
    // Bulk common imports
    pub use std::fs::{self, File};
    pub use std::io::Write;
    pub use std::path::Path;
    pub use std::path::PathBuf;

    // Import helper functions
    // pub use super::wav;
    pub use super::wav::WAV;

    pub use super::signed::SignedByte;
    pub use super::name::name_sample;
    pub use super::reader::read_chars;
    pub use super::reader::read_u16_le;
    pub use super::reader::read_u32_le;
    pub use super::ascii::string_from_chars;
    pub use super::Error;

    // Import macros
    // pub use crate::word;
    // pub use crate::dword;
    pub use crate::chars;
}

