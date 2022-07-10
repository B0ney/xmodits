use crate::{word, chars, dword};

use byteorder::{ByteOrder, LE};

/*
In this case, "#[inline]" sorta acts as a macro
in that during compilation, 
the function call is replaced with the function body.

This is usually done automatically, 
but I want to make sure this is the case
*/

#[inline]
pub fn read_u16_le(buf: &[u8], offset: usize) -> u16{
    LE::read_u16(&buf[word!(offset)])
}

#[inline]
pub fn read_u32_le(buf: &[u8], offset: usize) -> u32 {
    LE::read_u32(&buf[dword!(offset)])
}

#[inline]
pub fn read_chars(buf: &[u8], offset: usize, len: usize) -> &[u8] {
    &buf[chars!(offset, len)]
}