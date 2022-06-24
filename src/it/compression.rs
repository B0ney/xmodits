/// Impulse Tracker decompression
/// 
/// Reference:
/// https://github.com/nicolasgramlich/AndEngineMODPlayerExtension/blob/master/jni/loaders/itsex.c
/// https://wiki.multimedia.cx/index.php/Impulse_Tracker#IT214_sample_compression
/// https://github.com/schismtracker/schismtracker/blob/master/fmt/compression.c

use crate::utils::Error;
use crate::offset_u16;
use byteorder::{ByteOrder, LE};

pub fn decompress_sample(buf: &[u8], len: u32, smp_bits: u8, it215: bool) -> Result<Vec<u8>, Error> {
    use crate::utils::signed::SignedByte;
    match smp_bits {
        16 => decompress_16bit(buf, len, it215),
        _ => Ok(decompress_8bit(buf, len, it215)?.to_signed()), 
    }
}

/// My solution to C's awful shared states.
/// 
/// In the original C implementation, "read bits" and "read block"
/// share a lot of data. 
/// 
/// So why not combine them?
/// 
/// 
/// bit reading order:
/// ```
/// 0101_1100, 1011_0111,   ....._.....
/// |       |  |       |    |         |
/// \   <-<-|  \   <-<-|    \       <-|
///         |          |-------|      |-(3) Contiune until desired bits are met
///         |                  |
///         |-(1) Start here   |
///           (Right -> Left)  |
///                            |-(2) When it reaches MSB (Most Significant Bit)
///                               Move to next byte. Continue reading until it hits MSB.
///  
/// If I want to read 12 bits, the result would be:
/// 
/// 0111__0101_1100
///    |  |_______|
///    |          |-- From first Byte
///    |--------|
///             |---- From second Byte
/// 
/// You'd pad the Left Most bits like so:
/// 
/// 0000_0111__0101_1100
/// ```
struct BitReader<'a> {
    block_offset: usize,    // Location of next block
    bitnum: u8,             // Bits left. When it hits 0, it resets to 8 & "blk_index" increments by 1.  
    bitbuf: u32,            // Internal buffer for storing read bits
    buf: &'a [u8],          // IT Module buffer (read-only because reading data shouldn't modify anything)
    blk_data: Vec<u8>,      // Sections of "buf" are copied here for modification.
    blk_index: usize,       // Used to index blk_data.
}

impl <'a>BitReader<'a> { 
    fn new(buf: &'a [u8]) -> Self {
        Self { 
            bitnum: 0,
            bitbuf:0,
            buf: &buf,
            blk_data: Vec::new(),
            blk_index: 0,
            block_offset: 0x0000,
        }
    }

    fn read_next_block(&mut self) -> Result<(), Error> {
        // First 2 bytes combined to u16 (LE). Tells us size of compressed block. 
        let block_size = LE::read_u16(&self.buf[offset_u16!(self.block_offset)]);

        if block_size == 0 {
            return Err("block size is zero >:(".into());
        } 

        // copy section of buffer for mutation.
        self.blk_data = self._allocate(block_size as usize)?;
        // Set to 2 to skip length field
        self.blk_index = 2; 
        // Initialize bit buffer.
        // (Following the original by setting it to 0 caused a lot of headaches :D)
        self.bitbuf = self.blk_data[self.blk_index] as u32; 
        self.bitnum = 8;

        // Move to next block when called again
        // Add 2 since we need to skip previous length field
        self.block_offset += block_size as usize + 2;  
        Ok(())
    }   

    fn _allocate(&self, size: usize) -> Result<Vec<u8>, Error> {
        if self.buf.len() < self.block_offset + size + 2 {
            return Err("Cannot Allocate, buffer is too small".into());
        }
        Ok(self.buf[self.block_offset..].to_vec())
    } 

    fn read_bits_u16(&mut self, n: u8) -> Result<u16, Error> { 
        Ok(self.read_bits_u32(n)? as u16)
    }

    fn read_bits_u32(&mut self, n: u8) -> Result<u32, Error> { 
        // prevent panic if user forgets to call before reading bits.
        // tbh this is more useful when unit testing than here... 
        if self.blk_data.is_empty() { self.read_next_block()?; }

        let mut value: u32 = 0;
        let i =  n;

        for _ in 0..i {
            if self.bitnum == 0 {
                self.blk_index += 1;
                self.bitbuf = self.blk_data[self.blk_index] as u32;
                self.bitnum = 8;
            }
            value >>= 1;
            value |= self.bitbuf << 31;
            self.bitbuf >>= 1;
            self.bitnum -= 1;
        }

        return Ok(value >> (32 - n)); 
    }   
}


/// Decompresses 8 bit sample from buffer.
/// 
/// At this stage, i'm not interested in optimisations
/// 
/// Think of this as a rustified version of itsex.c
/// 
/// The goal here is to achive simplicity.
/// 
/// TODO:
///     deompressing stereo samples may not work.
///     refer to line 137 in compression.c for ideas.
///     Add stereo boolean parameter.
///     
fn decompress_8bit(buf: &[u8], len: u32, it215: bool) -> Result<Vec<u8>, Error> {
    let mut len: u32 = len;     // Length of uncompressed sample. (copied for mutation)
    let mut blklen: u16;        // uncompressed block length. Usually 0x8000 for 8-Bit samples
    let mut blkpos: u16;        // block position
    let mut sample_value: i8;   // decompressed sample value             (Note i8 for 8 bit samples)
    let mut d1: i8 = 0;         // integrator buffer for IT2.14          (Note i8 for 8 bit samples)
    let mut d2: i8 = 0;         // second integrator buffer for IT2.15   (Note i8 for 8 bit samples)
    let mut width: u8;          // Bit width. (Starts at 9 For 8-Bit samples)
    let mut value: u16;         // Value read (Note u16 for 8-bit samples)
    let mut dest_buf: Vec<u8>       = Vec::new();               // Buffer to write decompressed data
    let mut bitreader: BitReader    = BitReader::new(&buf);    // solution to C's horrible global variables

    // Unpack data
    while len != 0 {
        // Read new block, reset variables
        bitreader.read_next_block()?;

        // Make sure block len won't exceed len.
        blklen = if len < 0x8000 { len as u16 } else { 0x8000 };
        blkpos = 0;
        width = 9;
        d1 = 0; 
        d2 = 0;

        while blkpos < blklen {

            if width > 9 {
                return Err(format!("Invalid Bit width. Why is it {}?", width).into());
            }

            value = bitreader.read_bits_u16(width)?;
            
            if width < 7 { // Method 1, 1-6 bits

                if value == (1 << (width - 1)) as u16
                {
                    value = bitreader.read_bits_u16(3)? + 1;

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

            } else {  // Method 3, 9 bits

                if (value & 0x100) >> 8 == 1 // is bit 8 set? 
                { 
                    width = ((value + 1) & 0xff) as u8;
                    continue;
                }
                
            }

            // sample values are encoded with "bit width"
            // expand them to be 8 bits
            // expand value to signed byte
            if width < 8 {
                let shift: u8 = 8 - width;
                sample_value = (value << shift) as i8 ;
                sample_value >>= shift as i8;

            } else {
                sample_value = value as i8;
            }

            // integrate
            // In the original C implementation, 
            // values will wrap implicitly if they overflow
            d1 = d1.wrapping_add(sample_value);
            d2 = d2.wrapping_add(d1);

            dest_buf.push(
                if it215 { d2 as u8 } else { d1 as u8 }
            );

            blkpos += 1;
        }

        len -= blklen as u32; 
    }

    Ok(dest_buf)
}

fn decompress_16bit(buf: &[u8], len: u32, it215: bool) -> Result<Vec<u8>, Error> {
    let mut len: u32 = len;         // Length of uncompressed sample. (copied for mutation)
    let mut blklen: u16;            // uncompressed block length. Usually 0x4000 for 16-Bit samples
    let mut blkpos: u16;            // block position
    let mut sample_value: i16;      // decompressed sample value             (Note i16 for 16 bit samples)
    let mut d1: i16 = 0;            // integrator buffer for IT2.14          (Note i16 for 16 bit samples)
    let mut d2: i16 = 0;            // second integrator buffer for IT2.15   (Note i16 for 16 bit samples)
    let mut width: u8;              // Bit width. (Starts at 17 For 16-Bit samples)
    let mut value: u32;             // Value read (Note u32 for 16 bit sample)
    let mut dest_buf: Vec<u8>       = Vec::new();               // Buffer to write decompressed data
    let mut bitreader: BitReader    = BitReader::new(&buf);    // solution to C's horrible global variables

    while len != 0 {
        // Read new block, reset variables
        bitreader.read_next_block()?;

        // Make sure block len won't exceed len.
        blklen = if len < 0x4000 { len as u16 } else { 0x4000 };
        blkpos = 0;
        width = 17;
        d1 = 0; 
        d2 = 0;
        
        while blkpos < blklen {

            if width > 17 {
                return Err(format!("Invalid Bit width. Why is it {}?", width).into());
            }

            value = bitreader.read_bits_u32(width)?;

            if width < 7 { // Method 1, 1-6 bits
                
                if value == (1 << (width - 1)) as u32
                {
                    value = bitreader.read_bits_u32(4)? + 1;

                    let val = value as u8;
                    width = if val < width { val } else { val + 1 };
                    continue;
                }
            
            } else if width < 17 { // Method 2, 7-16 bits
                let border: u32 = (0xffff >> (17 - width)) - 8;

                if value > border
                    && value <= (border + 16)
                    {
                        value -= border;

                        let val = value as u8;
                        width = if val < width { val } else { val + 1 };
                        continue;
                    }

            } else {  // Method 3, 17 bits
                if (value & 0x10000) >> 16 == 1 // is bit 16 set? 
                { 
                    width = ((value + 1) & 0xff) as u8;
                    continue;
                }
            }

            if width < 16 {
                let shift: u8 = 16 - width;
                sample_value = (value << shift) as i16 ;
                sample_value >>= shift as i16;
            } else {
                sample_value = value as i16;
            }

            d1 = d1.wrapping_add(sample_value);
            d2 = d2.wrapping_add(d1);

            {   // write i16 to u8 buffer
                let mut buf = vec![0u8; 2];
                LE::write_i16(&mut buf,
                    if it215 { d2 } else { d1 }
                );

                dest_buf.append(&mut buf);
            }

            blkpos += 1;
        }

        len -= blklen as u32; 
    }

    Ok(dest_buf)
}

/// Use this to make sure BitReader works properly
#[test]
fn readbit() {
    let buf: Vec<u8> = vec![
        0x1, 0x0, // block size header (LE) of 1 byte
        0b1111_1110, 0b1111_1111,   // group 1

        0b1010_1110,                // group 2
        
        0b1100_1100,0b1100_1111,    // group 3
        
        0b0011_1010,

        0b1010_1010, 0b1100_1100,
        0b1100_1100, 0b1010_1010,
        0b1010_1010, 0b1100_1100,
    ];
    let mut b = BitReader::new(&buf);
    b.read_next_block().unwrap(); // this must be called before reading bits. UPDATE: will automatically do

    // test group 1
    assert_eq!(b.read_bits_u16(8).unwrap(), 0b_1111_1110);
    assert_eq!(b.read_bits_u16(8).unwrap(), 0b_1111_1111);
    
    // test group 2
    assert_eq!(b.read_bits_u16(4).unwrap(), 0b_0000_1110);
    assert_eq!(b.read_bits_u16(4).unwrap(), 0b_0000_1010);

    // test group 3
    assert_eq!(b.read_bits_u16(16).unwrap(), 0b_1100_1111_1100_1100);
    assert_eq!(b.read_bits_u16(9).unwrap(), 0b_0_0011_1010);

    // println!("{:016b}", b.read_bits(8).unwrap());
}