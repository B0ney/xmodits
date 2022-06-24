/// Extends Vec<u8> with a new method called "to_signed"
/// 
/// This converts the values to a signed value while retaining the u8 type
pub trait SignedByte {
    fn to_signed(&self) -> Vec<u8>; 
}

impl SignedByte for Vec<u8> {
    fn to_signed(&self) -> Vec<u8> {
        self.iter().map(|e| e.wrapping_sub(128)).collect::<Vec<u8>>()
    }
}

impl SignedByte for &[u8] {
    fn to_signed(&self) -> Vec<u8> {
        self.iter().map(|e| e.wrapping_sub(128)).collect::<Vec<u8>>()
    }
}