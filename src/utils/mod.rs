/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod ascii;
pub mod macros;
pub mod name;
pub mod reader;
pub mod signed;
pub mod wav;
// pub mod interleave;
pub type Error = crate::XmoditsError;

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
    pub use super::name::SampleNamer;
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
