// XM samples are encoded in delta values
// https://github.com/milkytracker/MilkyTracker/blob/master/resources/reference/xm-form.txt#L303=
use byteorder::{ByteOrder, LE};
use crate::dword;

pub fn delta_decode_u8(buf: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8>    = buf.to_owned();
    let mut old: u8         = 0;
    let mut new: u8         = 0;

    for i in 0..buf.len() {
        new = buf[i].wrapping_add(old);
        buf[i] = new;
        old = new;
    }

    buf
}

pub fn delta_decode_u16(buf: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8>    = buf.to_owned();
    let mut old: i16        = 0;
    let mut new: i16        = 0;

    for i in 0..((buf.len() / 2) - 1) {
        let idx: usize = i * 2;
        new = LE::read_i16(&buf[dword!(idx)]).wrapping_add(old);
        LE::write_i16(&mut buf[dword!(idx)], new);
        old = new;
    }

    buf
}