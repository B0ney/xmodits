/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{dword, slice, word};
use byteorder::{ByteOrder, LE};
use std::borrow::Cow;
use crate::Error;
/*
In this case, "#[inline]" sorta acts as a macro
in that during compilation,
the function call is replaced with the function body.

This is usually done automatically,
but I want to make sure this is the case
*/

#[inline]
/// Helper function to make word! more readable
pub fn read_u16_le(buf: &[u8], offset: usize) -> Result<u16, Error> {
    Ok(LE::read_u16(&buf.get(word!(offset)).ok_or_else(Error::out_of_bounds)?))
}

#[inline]
/// Helper function to make dword! more readable
pub fn read_u32_le(buf: &[u8], offset: usize) -> Result<u32, Error> {
    Ok(LE::read_u32(&buf.get(dword!(offset)).ok_or_else(Error::out_of_bounds)?))
}

#[inline]
/// Helper function to make chars! more readable
pub fn read_slice(buf: &[u8], offset: usize, len: usize) -> Result<&[u8], Error> {
    Ok(&buf.get(slice!(offset, len)).ok_or_else(Error::out_of_bounds)?)
}

#[inline]
/// Helper function to obtain String from ```&[u8]```
pub fn read_string(buf: &[u8], offset: usize, len: usize) -> Result<String, Error> {
    use super::ascii::string_from_buf;
    Ok(string_from_buf(read_slice(buf, offset, len)?))
}

#[inline]
/// Helper function to obtain Cow String from ```&[u8]```
pub fn read_str(buf: &[u8], offset: usize, len: usize) -> Result<Cow<str>, Error> {
    Ok(String::from_utf8_lossy(read_slice(buf, offset, len)?))
}
