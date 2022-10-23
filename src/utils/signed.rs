use crate::TrackerSample;

#[inline]
pub fn make_signed_u8_checked<'a>(buf: &'a mut [u8], smp: &mut TrackerSample) -> &'a [u8] {
    let mut is_signed = &mut smp.is_readable;

    if *is_signed {
        &buf[smp.ptr_range()]
    } else {
        *is_signed = true;
        make_signed_u8(&mut buf[smp.ptr_range()])
    }
}

#[inline]
pub fn make_signed_u16_checked<'a>(buf: &'a mut [u8], smp: &mut TrackerSample) -> &'a [u8] {
    let mut is_signed = &mut smp.is_readable;

    if *is_signed {
        &buf[smp.ptr_range()]
    } else {
        *is_signed = true;
        make_signed_u16(&mut buf[smp.ptr_range()])
    }
}

#[inline]
pub fn make_signed_u8(buf: &mut [u8]) -> &[u8] {
    for i in buf.iter_mut() {
        *i = i.wrapping_sub(128)
    }

    buf
}

#[inline]
pub fn make_signed_u16(buf: &mut [u8]) -> &[u8] {
    use crate::word;
    use byteorder::{ByteOrder, BE, LE};

    for i in 0..(buf.len() / 2) {
        let idx: usize = i * 2;
        let new = LE::read_u16(&buf[word!(idx)]).wrapping_sub(32768);
        LE::write_u16(&mut buf[word!(idx)], new);
    }

    buf
}
