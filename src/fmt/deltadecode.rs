/*  XM samples are encoded in delta values
    https://github.com/milkytracker/MilkyTracker/blob/master/resources/reference/xm-form.txt#L303= 
*/
use byteorder::{ByteOrder, LE};
use crate::{word, TrackerSample};


#[inline]
pub fn delta_decode_u8_checked<'a>(buf: &'a mut [u8], smp: &TrackerSample) -> &'a [u8] { 
    let mut is_deltad = smp.is_readable.borrow_mut();

    if *is_deltad {
        &buf[smp.ptr_range()]
    } else {
        *is_deltad = true;
        delta_decode_u8(&mut buf[smp.ptr_range()])
    }
}

#[inline]
pub fn delta_decode_u16_checked<'a>(buf: &'a mut [u8], smp: &TrackerSample) -> &'a [u8] { 
    let mut is_deltad = smp.is_readable.borrow_mut();
    
    if *is_deltad {
        &buf[smp.ptr_range()]
    } else {
        *is_deltad = true;
        delta_decode_u16(&mut buf[smp.ptr_range()])
    }
}


#[inline]
pub fn delta_decode_u8(buf: &mut [u8]) -> &[u8] {
    let mut old: u8 = 0;
    let mut new: u8;

    for i in buf.iter_mut() {
        new = i.wrapping_add(old);
        *i = new.wrapping_sub(128); // convert to signed
        old = new;
    }

    buf
}

#[inline]
pub fn delta_decode_u16(buf: &mut [u8]) -> &[u8] {
    let mut old: i16 = 0;
    let mut new: i16;

    for i in 0..(buf.len() / 2) {
        let idx: usize = i * 2;
        new = LE::read_i16(&buf[word!(idx)]).wrapping_add(old);
        LE::write_i16(&mut buf[word!(idx)], new);
        old = new;
    }

    buf
}