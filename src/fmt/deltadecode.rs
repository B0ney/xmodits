/*  XM samples are encoded in delta values
    https://github.com/milkytracker/MilkyTracker/blob/master/resources/reference/xm-form.txt#L303= 
*/
use byteorder::{ByteOrder, LE};
use crate::word;

#[inline]
pub fn delta_decode_u8(buf: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8>    = buf.to_owned();
    let mut old: u8         = 0;
    let mut new: u8;

    for i in &mut buf{
        new = i.wrapping_add(old);
        *i = new.wrapping_sub(128); // convert to signed
        old = new;
    }

    buf
}

#[inline]
pub fn delta_decode_u16(buf: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8>    = buf.to_owned();
    let mut old: i16        = 0;
    let mut new: i16;

    for i in 0..(buf.len() / 2) {
        let idx: usize = i * 2;
        new = LE::read_i16(&buf[word!(idx)]).wrapping_add(old);
        LE::write_i16(&mut buf[word!(idx)], new);
        old = new;
    }

    buf
}