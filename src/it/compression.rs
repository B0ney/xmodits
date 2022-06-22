/// Impulse Tracker decompression
/// Refer to:
/// 
/// https://github.com/schismtracker/schismtracker/blob/master/fmt/compression.c
/// https://wiki.multimedia.cx/index.php/Impulse_Tracker#IT214_sample_compression
/// 
/// GOLD
/// https://github.com/nicolasgramlich/AndEngineMODPlayerExtension/blob/master/jni/loaders/itsex.c
/// 
/// TODO:
/// 
/// Find an IT module that uses compressed samples
/// 
/// Decompress 8 bit samples
/// 
/// Decompress 16 bit samples
/// 

/*
* A sample is compressed if bit 3 in flag is set.

* Compressed sample is split into blocks, when uncompressed is 0x8000 bytes (0x4000 for 16 bit samples)
* Decompressed block (or failed to) is delta decoded 

* Each block starts with a 16-bit length field, followed by the bitstream (LE)

*/

/*
Observation from C code



*/

use crate::utils::Error;
use crate::offset_u16;
use byteorder::{ByteOrder, LE, BE};

struct BitReader<'a> {
    block_offset: usize,        // location of next block.
    bitnum: u8,
    bitlen: u32,
    buf: &'a [u8], // access the buffer without cloning or mutating anything
    blk_data: Vec<u8>,
    blk_index: usize, // internal value pointer?
}

impl <'a>BitReader<'a> { 
    fn new(buf: &'a [u8]) -> Result<Self, Error> {
        Ok(Self { 
            bitnum: 0,
            bitlen:0,
            buf: &buf,
            blk_data: Vec::new(),
            blk_index: 0,
            block_offset: 0x0000,
        })
    }

    fn read_next_block(&mut self) -> Result<(), Error> {
        let block_size = self._get_block_size();

        if block_size == 0 {
            return Err("block size is zero >:(".into());
        }

        // copy section of buffer for mutation.
        self.blk_data = self._allocate(block_size as usize)?;
        self.blk_index = 0; 

        self.bitnum = 8;
        self.bitlen = block_size as u32;
        
        // move to next block if called again
        self.block_offset += block_size as usize; 

        Ok(())
    }   

    fn _allocate(&self, size: usize) -> Result<Vec<u8>, Error> {
        if self.buf.len() < self.block_offset + size + 2 {
            return Err("Cannot Allocate, buffer is too small".into());
        }
        // copy contents of buffer to new vector.
        // make things easier for mutation.
        // We add 2 to skip the length field
        Ok(self.buf[(self.block_offset)..size as usize].to_vec())
    } 

    fn _get_block_size(&self) -> u16 {
        // Combine 2 bytes to u16 (Little Endian)
        // offset_u16!(0x0000) -> 0x0000..(0x0000 + 4)
        LE::read_u16(&self.buf[offset_u16!(self.block_offset)])
    }

    fn read_bits(&mut self, width: u8) -> Result<u16, Error> {
        let mut retval: u32 = 0;
        let mut offset: u32 = 0;
        let mut n = width;

        while n != 0 {
            let mut m = n; // set mask to copy of width

            if self.bitlen == 0 {
                return Ok(retval as u16);
                // return Err("ran out of buffer".into()); 
            }

            if m > self.bitnum {
                m = self.bitnum
            };
            // println!("{}",m);
            retval |= ( (self.blk_data[self.blk_index] & ((1 << (m-1)) - 1)) as u32 ) << offset;
            
            self.blk_data[self.blk_index] >>= (m-1);
            n -= m;
            offset += m as u32;

            self.bitnum -= m;
            if self.bitnum == 0 {
                self.bitlen -= 1;
                self.blk_index += 1;
                self.bitnum = 8;
            };
        } 
        Ok(retval as u16)
    }   
}



/// Decompresses 8 bit sample from buffer.
/// 
/// At this stage, i'm not interested in optimisations
/// 
/// Think of this as a rustified version of itsex.c
/// 
/// The goal here is to achive simplicity.
pub fn decompress_8bit(buf: &[u8], len: u32) -> Result<Vec<u8>, Error> {
    let mut len: u32 = len;     // Length of uncompressed sample. (copied for mutation)
    let mut blklen: u16;        // uncompressed block length. Usually 0x8000 for 8-Bit samples
    let mut blkpos: u16;        // block position
    let mut sample_value: i8;   // decompressed sample value             (Note i8 for 8 bit samples)
    let mut d1: i8 = 0;         // integrator buffer for IT2.14          (Note i8 for 8 bit samples)
    let mut d2: i8 = 0;         // second integrator buffer for IT2.15   (Note i8 for 8 bit samples)
    let mut width: u8;          // Bit width. (Starts at 9 For 8-Bit samples)
    let mut value: u16;         // Value read 
    let mut dest_buf: Vec<u8> = Vec::new(); // Buffer to write decompressed data
    
    let mut bitread = BitReader::new(&buf)?; // solution to C's horrible global variables
    
    // Unpack data
    while len != 0 {
        // Read new block, reset variables
        // {}
        bitread.read_next_block()?;

        // Make sure block len won't exceed len.
        blklen = if len < 0x8000 {len as u16} else {0x8000};
        blkpos = 0;
        width = 9;
        // Reset integrator buffers
        d1 = 0; 
        d2 = 0;

        while blkpos < blklen {
            value = bitread.read_bits(width)?;

            if width < 7 { // Method 1, 1-6 bits
                if value == (1 << (width - 1)) as u16
                {
                    value = bitread.read_bits(3)? + 1;

                    let val = value as u8;
                    width = if val < width { val } else { val + 1 };
                    continue;
                }
            
            } else if width < 9 { // Method 2, 7-8 bits
                let border: u16 = (0xff >> (9 - width)) - 4;

                if value > border
                    && value <= (border + 8)
                    {
                        value -= border;

                        let val = value as u8;
                        width = if val < width { val } else { val + 1 };
                        continue;
                    }

            } else if width == 9 {  // Method 3, 9 bits
                if value & 0x100 == 1 // is the 9th bit set?
                { 
                    width = (value + 1) as u8;
                    continue;
                }
                
            } else {
               return Err("Illegal width".into()); 
            }

            // expand value to signed byte
            if width < 8 {
                let shift: u8 = 8 - width;
                sample_value = (value as i8) << shift;
                sample_value >>= shift
            } else {
                sample_value = value as i8
            }

            // integrate
            // println!("sample_value: {}",sample_value);
            d1 = d1.wrapping_add(sample_value);
            d2 = d2.wrapping_add(d1);

            dest_buf.push(d2 as u8);
           
            blkpos += 1;
        }
        len -= blklen as u32;
    }
    Ok(dest_buf)
}
// 
// pub fn decompress_16bit(buf: &[u8], len: u32) -> Result<Vec<u8>, Error> {

    // Push to buffer
    // {
        // let mut buf = vec![0u8 ;2];
        // LE::write_i16(&mut buf, d1); // todo
        
    // }


// }
