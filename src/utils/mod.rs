/* Did you mean amig_mod.rs? */

pub mod macros;
pub mod signed;
// pub mod array;
pub mod ascii;
pub mod name;
pub mod reader;
pub mod wav;

pub type Error = crate::XmoditsError;

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
    pub use super::wav::Wav;

    pub use super::name::name_sample;
    pub use super::reader::read_slice;
    pub use super::reader::read_string;
    pub use super::reader::read_u16_le;
    pub use super::reader::read_u32_le;
    pub use super::signed::make_signed_u16;
    pub use super::signed::make_signed_u8;
    pub use super::Error;

    // Import macros
    // pub use crate::word;
    // pub use crate::dword;
    pub use crate::slice;
}
