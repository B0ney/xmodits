// pub fn interleave(vec: &mut [u8], bits: usize) {
//     let len = vec.len();
//     assert!((len & 1) == 0);

//     let len_half = len / 2;

//     let is_half_even = ((len_half % 2) == 0) as usize;
//     // let is_half_even = 0;
//     dbg!(is_half_even);

//     let bytes = (bits/8) ;
//     // dbg!(!1_u8);
//     // (1..(len_half+1))
//     //     .filter(|x| (x % (bytes * 2)) == 0)
//     //     .for_each(|x| {
//     //         vec.swap(x, len_half + x + is_half_even);
//     //         if bytes == 2 {
//     //             vec.swap(x+1, len_half + x + 1 + is_half_even);

//     //         }
//     //         println!("{}", x-1);
//     //     });
//     if bytes == 1 {
//         for i in 0..len_half {
//             if (i & 1) == 0 {
//                 vec.swap(i, len_half + i + is_half_even);
//             }
//         }
//     } else {
//         for i in 0..len_half {
//             if (i & 4) == 0 {
//                 vec.swap(i, len_half + i);
//                 vec.swap(i+ 1, len_half + i + 1);

//             }
//         }
//     }
//     // for i in 0..len_half {
       
//     //     // if (i & 1) == 0 {
//     //     if ((i+bytes) % (bytes*2)) == 0 {
//     //         //  println!("{i}");
//     //         vec.swap(i, len_half + i + is_half_even);
//     //         if bytes == 2 {
//     //             vec.swap(i+1, len_half + i + 1 + is_half_even);

//     //         }
//     //     }
//     // } 
// }


// #[test]
// fn g() {
//     let mut a = [1,1,3,3,1,1,3,3,1,1,0,0,4,4,0,0,4,4,0,0];
//     // let mut a = [1,1,1,0,0,0];

//     interleave(&mut a , 16);
//     dbg!(a);

//     // let a = 4_u8;
//     // let b = 2_u8;
//     // let d = !(5_u8 & 3);
//     // println!("{:08b}", d);
//     // dbg!((! & 3));
// }