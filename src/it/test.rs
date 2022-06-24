use std::{fs::DirEntry, vec};
use crate::offset_u16;

use super::it::*;

#[test]
fn test1() {
    let a = ITFile::load("samples/NYCStreets_Music.it").unwrap();
    for i in 0..a.sample_number {
        let _ = a.export(format!("./test/{}.wav", i), i as usize);
    }
}

#[test]
fn test_flag_set() {
    const MASK_SMP_BITS: u8 = 0b0000_0010;
    let test_func = |b:u8| {8 * (((b & MASK_SMP_BITS) >> 1)  + 1)};
    let f1_8    = 0b010100_0_1;    // should be 8 
    let f2_16   = 0b000000_1_1;   // should be 16

    assert_eq!(test_func(f1_8), 8);
    assert_eq!(test_func(f2_16), 16);
}

#[test]
fn test_flag_set_2() {
    const MASK_SMP_BITS: u8 = 0b0000_1000;
    let test_func = |b:u8| {((b & MASK_SMP_BITS) >> 3) == 1};
    let f1_false    = 0b0101_0_001;    // should be false
    let f2_true     = 0b0000_1_011;   // should be true

    assert_eq!(test_func(f1_false), false);
    assert_eq!(test_func(f2_true), true);
}
#[test]
fn test_dump() {
    let a = ITFile::load("samples/comp/before_the_explozion.it").unwrap();
    // let _ = a.export(format!("./test/bingbong.wav"), 21).unwrap();
    // let _ = a.export(format!("./test/bingbon2g.wav"), 21).unwrap();
    // let _ = a.export(format!("./test/testdeus.wav"), 12).unwrap();



    // let f = &a.samples_meta[12];
    // println!("index: 0x{:04X}\nlength:{}\nrate:{}\nbits smp: {}\ncompressed: {}\n\n",
    //     // String::from_iter(f.filename),
    //     f.smp_ptr,
    //     f.smp_len,
    //     f.smp_rate,
    //     f.smp_bits,
    //     f.smp_comp,
    // );
    for (i,f) in a.samples_meta.iter().enumerate() {
        // if f.smp_bits == 16 && f.smp_comp {
            println!("{:}, {:08b}", f.smp_stereo as u8, f.smp_flag);
            println!("dumping: {}...",i + 1);


            if let Err(e) = a.export(

                format!("./test/{}.wav",
                i + 1,
                // c,
                ),i
                ) {
                    println!("{}", e);
                };
        // }
    }
        

        // println!("{}",i);
        // println!("name:{}\nindex: 0x{:04X}\nlength:{}\nrate:{}\nbits smp: {}\ncompressed: {}\n\n",
        //     String::from_iter(f.filename),
        //     f.smp_ptr,
        //     f.smp_len,
        //     f.smp_rate,
        //     f.smp_bits,
        //     f.smp_comp,
        // );

    // }
}
#[test]
fn test69() {
    let a = ITFile::load("samples/comp/worldies.it").unwrap();
    println!("{:04X}\n{:04X}", a.version, a.compat_version);
}