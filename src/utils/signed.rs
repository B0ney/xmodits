#[inline]
pub fn make_signed_u8(buf: &mut [u8]) -> &[u8] {
    for i in buf.iter_mut() {
        *i = i.wrapping_sub(128)
    }

    buf
}

#[inline]
pub fn make_signed_u16(buf: &mut [u8]) -> &[u8] {
    use byteorder::{LE, ByteOrder, BE};
    use crate::word;

    for i in 0..(buf.len() / 2) {
        let idx: usize = i * 2;
        let new = LE::read_u16(&buf[word!(idx)]).wrapping_sub(32768);
        LE::write_u16(&mut buf[word!(idx)], new);
    }

    buf
}