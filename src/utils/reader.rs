/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::borrow::Cow;
use crate::{dword, slice, word};
use byteorder::{ByteOrder, LE};

/*
In this case, "#[inline]" sorta acts as a macro
in that during compilation,
the function call is replaced with the function body.

This is usually done automatically,
but I want to make sure this is the case
*/

#[inline]
/// Helper function to make word! more readable
pub fn read_u16_le(buf: &[u8], offset: usize) -> u16 {
    LE::read_u16(&buf[word!(offset)])
}

#[inline]
/// Helper function to make dword! more readable
pub fn read_u32_le(buf: &[u8], offset: usize) -> u32 {
    LE::read_u32(&buf[dword!(offset)])
}

#[inline]
/// Helper function to make chars! more readable
pub fn read_slice(buf: &[u8], offset: usize, len: usize) -> &[u8] {
    &buf[slice!(offset, len)]
}

#[inline]
/// Helper function to obtain String from ```&[u8]```
pub fn read_string(buf: &[u8], offset: usize, len: usize) -> String {
    use super::ascii::string_from_buf;
    string_from_buf(read_slice(buf, offset, len))
}

#[inline]
/// Helper function to obtain Cow String from ```&[u8]```
pub fn read_str<'a>(buf: &'a [u8], offset: usize, len: usize) -> Cow<'a, str>{
    String::from_utf8_lossy(read_slice(buf, offset, len))
}