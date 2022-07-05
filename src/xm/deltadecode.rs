// XM samples are encoded in delta values
// https://github.com/milkytracker/MilkyTracker/blob/master/resources/reference/xm-form.txt#L303=
use byteorder::{ByteOrder, LE};

// NOTICE, sample data is stored as signed values.
fn delta_decode_u8(buf: &mut [u8]){
    let mut old = 0;
    let mut new = 0;

    for i in 0..buf.len() {
        new = buf[i].wrapping_add(old);
        buf[i] = new;
        old = new;
    }
}

fn delta_decode_u16(buf: &mut [u8]){
    let mut old: i16 = 0;
    let mut new: i16 = 0;

    for i in 0..buf.len() {
        new = LE::read_i16(&buf[i..i+1]).wrapping_add(old);
        LE::write_i16(&mut buf[i..i+1], new);
        old = new;
    }
}


