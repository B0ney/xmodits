// use crate::s3m::{S3MFile, TrackerDumper};

// #[test]
// fn test1() {
//     let a = S3MFile::load_module("samples/s3m/city_on_a_stick.s3m").unwrap();
//     println!("{}", a.number_of_samples());
//     for i in 0..a.number_of_samples() {
//         if let Err(e) = a.export(&format!("test/s3m/"), i) {
//             println!("{:?}", e);
//         }
//     }
// }

// #[test]
// fn generate_offsets() {
//     let data = [
//         (1* 28),
//         1, 1,
//         2,2, 2,2, 2,2, 2,
//         (1 * 4),
//         1,1,1, 1,1,1,
//         (1 * 8),
//         2,
//         (1 * 32),
//         1

//     ];
//     let mut sum = 0;

//     for i in data {
//         println!("0x{:04X} => ", sum);
//         sum += i;
//     }
//     println!("\n0x{:04X} => ", sum);
// }

// #[test]
// fn generate_offsets_2() {
//     let data = [
//         1, 2,
//         4,4,4,
//         1,1,1,1,
//         4,
//         (1 * 12),
//         (1 * 28), 
//         (1 * 4),
//     ];
//     let mut sum = 0;

//     for i in data {
//         println!("0x{:04X} => ", sum);
//         sum += i;
//     }
//     println!("\n0x{:04X} => ", sum);
// }


