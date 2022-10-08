use crate::{word, utils::prelude::read_u16_le};

/// Extends Vec<u8> with a new method called "to_signed"
/// 
/// This converts the values to a signed value while retaining the u8 type
pub trait SignedByte {
    fn to_signed(&self) -> Vec<u8>; 
    fn to_signed_u16(&self) -> Vec<u8>;
    fn to_signed_mut(self) -> Vec<u8>;
}

impl SignedByte for Vec<u8> {
    fn to_signed(&self) -> Vec<u8> {
        write_u8(self)
    }

    fn to_signed_mut(self) -> Vec<u8> {
        write_u8_mut(self)
    }

    fn to_signed_u16(&self) -> Vec<u8> {
        write_u16(self)
    }
}

impl SignedByte for &[u8] {
    fn to_signed(&self) -> Vec<u8> {
        write_u8(self)
    }

    fn to_signed_mut(self) -> Vec<u8> {
        self.to_signed()
    }

    fn to_signed_u16(&self) -> Vec<u8> {
        write_u16(self)
    }
}

#[inline]
fn write_u8(pcm: &[u8]) -> Vec<u8> {
    pcm.iter().map(|e| e.wrapping_sub(128)).collect::<Vec<u8>>()
}

#[inline]
fn write_u8_mut(mut pcm: Vec<u8>) -> Vec<u8> {
    pcm.iter_mut().for_each(|e| { *e=e.wrapping_sub(128); });
    pcm
}

#[inline]
fn write_u16(pcm: &[u8]) -> Vec<u8>{
    use byteorder::{LE, ByteOrder, BE};
    let mut out: Vec<u8> = Vec::with_capacity(pcm.len());

    for i in 0..(pcm.len() / 2) {
        let mut buf: Vec<u8> = vec![0u8; 2];
        let i = i * 2;
        let val = LE::read_u16(&pcm[word!(i)]).wrapping_sub(32768);
        LE::write_u16_into(&[val], &mut buf);
        out.append(&mut buf);
    }

    out
}

pub fn make_signed_u8(buf: &mut [u8]) -> &[u8] {
    for i in buf.iter_mut() {
        *i = i.wrapping_sub(128)
    }
    buf
}