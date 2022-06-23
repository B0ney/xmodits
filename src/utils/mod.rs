// use std::convert::From;
pub mod macros;
pub mod signed;

pub type Error = Box<dyn std::error::Error>;
pub use signed::SignedByte;
// pub fn load_to_array<T, U>(array: &mut [T], data: &[U])
// where U: From<<T as U>::Output> + Sized,
//     T: Sized,
// {
//     assert!(array.len() <= data.len());

//     for i in 0..array.len() {
//         array[i] = T::from(data[i]);
//     }
// }
