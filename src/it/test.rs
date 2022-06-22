use std::{fs::DirEntry, vec};

use crate::offset_u16;

use super::it::*;

#[test]
fn test1() {
    let a = ItFile::load("samples/NYCStreets_Music.it").unwrap();
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
    let a = ItFile::load("samples/comp/before_the_explozion.it").unwrap();
    let _ = a.export(format!("./test/test_decomp1"), 15).unwrap();
    // for (i,f) in a.samples_meta.iter().enumerate() {
    //     // let _ = a.export(format!("./test/{i}.wav"), i);

    //     println!("{}",i);
    //     println!("index: 0x{:04X}\nlength:{}\nrate:{}\nbits smp: {}\ncompressed: {}\n\n",
    //         f.smp_ptr,
    //         f.smp_len,
    //         f.smp_rate,
    //         f.smp_bits,
    //         f.smp_comp,
    //     );

    // }
}




#[test]
fn test_decomp() {
    use byteorder::{ByteOrder, BE, LE};
    let mut comp_data: Vec<u8> = std::fs::read("./test/raw/explosion_comp_smp_6").unwrap();
    let mut len = comp_data.len(); //length of sample data
    /*
    length: 7411
    bits smp 16
    */
    // let src_buff:
    let smp_bits: u16 = 16;
    /*
    change to:
    if 0x4000 < len {0x4000} else {len}
    */
    let block_len: u16 = 0x4000; // length of compressed data block in samples. 0x4000 because sample is 16 bit, otherwise it's 0x8000, 
    let mut block_pos: u16 = 0;

    let mut width: u8 = 17; // 17 because sample is 16 bit, otherwise start with 9

    let mut value: u32; // value read from data to be processed

    let mut len = comp_data.len(); //length of sample data
    let mut dest_buf: Vec<u8> = vec![0; len];
    let mut dest_pos: usize = 0;

    let mut bitnum: u32 = 0;
    let mut bitbuf: u32 = 0;
    
    println!("0x{:04X}", LE::read_u16(&comp_data[offset_u16!(0x0000)]));
    // read block
    /* 
    block structure:
    size: [u16]
    
    

    */

    // set value to ???

    // reset integrator buffers
    let mut d1 = 0;
    let mut d2 = 0;
    //  current position in source buffer
    let mut srcbuf: usize = 0;// we'd set this to the sample pointer, but in this case set to 0
    let mut filebuf: usize = 0;
    

    while (len != 0) {
        // read new block, reset vars
        d1 = 0;
        d2 = 0;

        if srcbuf + 2 > filebuf + len
            || srcbuf + 2 /*+? */ > filebuf + len{

            break;
        };

        // test decompress for 16 bit
        while block_pos < block_len {
            if width > 17 {
                println!("Illegal bit width for 16 bit sample");
                return;
            } 

            value = (it_readbits(
                4,
                &mut bitbuf,
                &mut bitnum,
                &comp_data,
                srcbuf
            ) + 1) as u32;

            if width < 7 {
                // Type A: Bit widths 1-6.
                if value == 1 << (width - 1) as u32 {
                    // read 4 bits for calculating new bit width
                    value = (it_readbits(
                        4,
                        &mut bitbuf,
                        &mut bitnum,
                        &comp_data,
                        srcbuf
                    ) + 1);
                    // expand

                    width = if (value < width as u32) { value  as u8} else { (value + 1) as u8}; // check
                    continue
                } 

            } else if width < 17 {
                // Type B: Bit widths 7-16. (16 will be 8 if 8-bit )
                let border: u32 = (0xffff >> (17 - width)) as u32 - 8;
                
                if value > border
                    && value <= (border + 16) as u32
                {
                    value -= border as u32; // convert width to 1-8
                    width = if (value < width as u32) { value as u8} else { (value + 1) as u8}; // check
                    continue
                }

            } else {
                // Type C:
                // If top bit is set i.e 1, width = lower 8 bits + 1
                // 0b00001_0000_0000_0000_0000
                if value & 0x10000 ==1 {
                    width = ((value+ 1) & 0x00ff) as u8;
                    continue
                }
            }
            // expand value to signed word:
            let mut v: u16;
            if width < 16 {
                let shift = 16 - width;
                v = (value as u16) << (shift as u16);
                v >>= shift;
            } else {
                v = value as u16; // idk if this is good, we want to truncate 32 bit value to fit.
            }
            // "integrate upon the sample values"
            d1 += v;
            d2 += d1;


            LE::write_u16(&mut dest_buf[offset_u16!(dest_pos)], d1);
            // dest_buf.push(d1); // or d2?
            dest_pos += 1; // only 1 channel
            block_pos += 1;

            // then store to buffer
        }
        len -= block_len as usize
    }

    println!("{}", dest_buf.len()); 
    // instead of returning slice, why not return new vector containing decompressed sample?
}

// given width, and
// fn it_readbits(
//     width: u8,

//     bit_buffer: &mut u32, 
//     bit_num: &mut u32,

//     buf: &[u8],
// ) -> u32 {
//     let mut value: u32 = 0;
//     let i = width;

//     for _ in i..0 {
//         if (!bit_num) {
//             // bit_buffer = ;
//             bit_num = 8;
//         } 
//         value >>= 1;
//         value |= bit_buffer << 31;
//         bit_buffer >> 1;
//         bit_num -= 1;
//     }
//     return value >> (32 - n); 

// }


/* 
understaning code implementation

in a nutshell:
    * decompress data
    * delta decode data

delta decoding:
A bit transition from 0->1 / 1->0 will be a 1
no bit transition = 0

1001_0110_0010_1001_0010 
becomes:
0101_1101_0011_1001_1011



compressed block starts with 16 bit length field (2 bytes), followed by LE bitstream.

BUF = buffer containing compressed sample

Index = index for our buffer, this is equivalent to the "srcbuf" pointer in C
"srcbuf" -> current position in source buffer.

BUF[Index] = "*srcbuf" in C, provides u8.



in loop:

first 2 bytes are skipped, (Index += 2)

block pos set to 0 (relative to 2nd skipped byte from above)






dest_buf = Vec<u8> storing decompressed data

dest_index = "destpos" in C

dest_buf[dest_index] = "*destpos" in C

"*destpos = it215 ? d2 : d1;" can be:
if it215?
    LE::write_u16(dest_buf[offset_u16!(index)], d2)
else: 
    LE::write_u16(dest_buf[offset_u16!(index)], d2)

"destpos += channels;" -> dest_index += channels (mostly 1);


*/

// #[test]
// fn find_compressed_sample() {
//     // let f: Vec<DirEntry> = std::fs::read_dir("../samples/")
//     //     .unwrap()
//     //     .filter_map(|f| f.ok())
//     //     .collect();

//     let a = ItFile::load("samples/xerxes_kandu.it").unwrap();
//     let cm: Vec<&ItSample> = a 
//         .samples_meta
//         .iter()
//         .filter(|e| e.smp_comp)
//         .collect();

//     if cm.is_empty() {
//         println!("\nModule has no compressed samples :(\n");
//     } else {
//         println!{"{:#?}", &cm};
//     }
// }

#[test]
fn read_bit_test() {
    // 00000000000000000_010_1010_1100_1100
    let buffer: Vec<u8> = vec![0b0010_1010, 0b1100_1100,0b0010_1010, 0b1100_1100];
    let index: usize = 0;
    let mut bitnum: u32 = 0;
    let mut bitbuf: u32 = 0;
    let bits_to_read: i8 = 15; 

    let bits = it_readbits(
        bits_to_read,
        &mut bitbuf,
        &mut bitnum,
        &buffer,
        index,
    );


    let bits_to_read: i8 = 16; 
    let index: usize = 1;

    let bits = it_readbits(
        bits_to_read,
        &mut bitbuf,
        &mut bitnum,
        &buffer,
        index,
    );


    println!("{:032b}", bits);

}
// todo, make improved version as this is a rust mplementation of the c code
// reads bits given buffer,
// LE
fn it_readbits(
    n: i8,
    bitbuf: &mut u32,
    bitnum: &mut u32,
    buf: &[u8],
    index: usize,
) -> u32 {
    let mut index = index;
    let mut value: u32 = 0;
    let mut i: u32 = n as u32;
    while i > 0 {
        if (*bitnum) == 0 {
            index += 1;
            *bitbuf = buf[index] as u32;
            *bitnum = 8;
        }
        value >>= 1;
        value |= ( *bitbuf ) << 31;
        (*bitbuf) >>= 1;
        *bitnum -= 1;

        i -= 1;
    }
    println!("current number:   {:032b}", bitnum);
    println!("current bitbuffer: {:032b}", bitbuf);

    return value >> (32 -n);

}

