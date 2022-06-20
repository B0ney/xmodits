// https://fileformats.fandom.com/wiki/Impulse_tracker
// rememer that data is in little endian
// we're only interested in dumping every sample
use byteorder::{BigEndian, ByteOrder, LittleEndian, LE, BE};
use core::num;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

type Error = Box<dyn std::error::Error>;

use crate::offset_chars;
use crate::offset_u16;
use crate::offset_u32;

const IT_HEADER_ID: u32 = 0x49_4D_50_4D; // IMPM
const IT_SAMPLE_ID: u32 = 0x49_4D_50_53; // IMPS

const IT_HEADER_LEN: usize = 192;
const IT_SAMPLE_LEN: usize = 80;

const IT_INSTRU_ID_OLD: u32 = 0x494D5049; // IMPI

mod macros {
    #[macro_export]
    macro_rules! offset_u16 {
        ($i:expr) => {
            $i..$i + 2
        };
    }

    #[macro_export]
    macro_rules! offset_u32 {
        ($i:expr ) => {
            $i..$i + 4
        };
    }

    #[macro_export]
    macro_rules! offset_u64 {
        ($i:expr) => {
            $i..$i + 8
        };
    }
    #[macro_export]
    macro_rules! offset_chars  {
        ($i:expr, $e:expr) => {$i..$i + $e};
    }
}

/*
foo->bar is equivalent to (*foo).bar,
i.e. it gets the member called bar from the struct that foo points to.

Samples can be found by locating
IT_SAMPLE_ID


refer to its.c , void save_its_header() {}
Flags:
0000_0001 ?? (r)
0000_0010
0000_0100
0000_1000
0001_0000
0010_0000
0100_0000
1000_0000

*/

/*
Idea:

1) locate "IMPS" header and obtain:
    * sample name / filename
    * sample length
    * sample pointer
    * format ???

2) jump to the sample pointer,
3) obtain bytes specified by sample length

we don't care about the other infomation,
but it's important to know them so that we can identify their offsets

*/

#[derive(Debug)]
pub struct ItFile {
    buffer: Vec<u8>,
    sample_number: u16,
    samples_meta: Vec<ItSample>,
}

impl ItFile {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let buffer: Vec<u8> = fs::read(path)?;

        if buffer.len() < IT_HEADER_LEN
            || BE::read_u32(&buffer[offset_u32!(0x0000)]) != IT_HEADER_ID
        {
            return Err("File is not a valid Impulse Tracker Module".into());
        };

        let sample_number = LE::read_u16(&buffer[offset_u16!(0x0024)]);
        let samples_meta = build_samples(&buffer, sample_number)?;
        
        Ok(Self {
            buffer,
            sample_number,
            samples_meta,
        })
    }
    fn export<P: AsRef<Path>>(&self, path: P, index: u16) -> Result<(), Error> {
        let index = if index > self.sample_number { 
            self.sample_number - 1 } else {index};

        let header = build_wav_header(&self.samples_meta[index as usize]);

        let sample = &self.samples_meta[index as usize];
        let bits_sample: u16 = match sample.flags & MASK_BITS_SAMPLE {
            0b11 => 16, // 16 bit samples
            0b01 => 8, // 8- bit samples
            _ => 16,
        };
        println!(
            "sample length: {}\nsample pointer {:04X}\nsample speed: {}\nsample flags: {:08b}\n\n",
            &sample.length,
            &sample.sample_pointer,
            &sample.sample_rate,
            &sample.flags,
        );

        println!("{}", bits_sample);
        let start_ptr = sample.sample_pointer as usize;
        let end_ptr = (start_ptr + sample.length as usize) * (bits_sample/8) as usize;
        println!("{}", end_ptr-start_ptr);
        // println!("{}", );

        let raw = &self.buffer[sample.sample_pointer as usize..((sample.sample_pointer + sample.length))  as usize ];
        // We need to convert raw u8 buffer to i8 
        
        // yuck!

        // convert raw [u8] to [i8] for 8-bit samples and 16 bit samples
        //
        let a = raw.iter().map(|e| e.wrapping_sub(127)).collect::<Vec<u8>>();
        // println!("{}", raw.len());
        let mut file = File::create(path)?;
        file.write_all(&header)?;
        file.write_all(&a)?;
        Ok(())

    }
}

fn build_samples(it_data: &Vec<u8>, num_samples: u16) -> Result<Vec<ItSample>, Error> {
    let mut ins_start_index: usize = 0;
    let mut smp_meta: Vec<ItSample> = Vec::new();

    if num_samples == 0 {
        return Err("IT module doesn't contain any samples.".into());
    }

    for i in 0..(it_data.len() - 4) { // 4 is the amount of bytes a u32 takes up. Prevents panic.
        if BE::read_u32(&it_data[offset_u32!(i)]) == IT_SAMPLE_ID {
            ins_start_index = i;

            break;
        }
    }

    if ins_start_index == 0 {
        return Err(format!(
            "IT module doesn't contain any samples. Despite showing that it has \"{}\" samples",
            num_samples
        )
        .into());
    }

    for i in 0..num_samples {
        let offset = ins_start_index + (i * 0x50) as usize; // 0x50 = size of sample metadata (80 bytes)
        let mut filename:   [char; 12] = [' '; 12];
        let mut name:       [char; 26] = [' '; 26];

        load_to_array(&mut filename, &it_data[offset_chars!(0x0004 + offset, 12)]);
        load_to_array(&mut name, &it_data[offset_chars!(0x0014 + offset, 26)]);

        smp_meta.push(ItSample {
            filename,
            name,
            length:             LE::read_u32(&it_data[offset_u32!(0x0030 + offset)]),
            sample_pointer:     LE::read_u32(&it_data[offset_u32!(0x0048 + offset)]),
            sample_rate:        LE::read_u32(&it_data[offset_u32!(0x003C + offset)]),
            flags:              it_data[0x012 + offset],
        })
    }

    Ok(smp_meta)
}

// maybe use generics?
fn load_to_array(array: &mut [char], data: &[u8]) {
    assert!(array.len() <= data.len());

    for i in 0..array.len() {
        array[i] = data[i] as char;
    }
}


const RIFF: u32 = 0x5249_4646; // RIFF
const WAVE: u32 = 0x5741_5645; // WAVE
const FMT_: u32 = 0x666D_7420; // "riff "
const DATA: u32 = 0x6461_7461; // data
const HEADER_SIZE: u8 = 44;
const MASK_BITS_SAMPLE: u8 = 0b0000_0011;

fn build_wav_header(raw: &ItSample) -> [u8; HEADER_SIZE as usize]{
    let mut header:[u8; HEADER_SIZE as usize] = [0u8; HEADER_SIZE as usize]; 

    let file_size: u32 = HEADER_SIZE as u32 + raw.length - 8;
    let wav_scs: u32 = 16; // sec chunk size
    let wav_type: u16 = 1; // 1 = pcm
    let wav_flag_ms: u16 = 0x01; // mono/stereo 0x01 = mono, 0x02 = stereo
    let sample_frequency: u32 = raw.sample_rate;
    let bytes_sec: u32 = raw.sample_rate * 1; // sample_rate * channels
    let block_align: u16 = 0x01; // can be anything really
    let bits_sample: u16 = match raw.flags & MASK_BITS_SAMPLE {
        0b11 => 16, // 16 bit samples
        0b01 => 8, // 8- bit samples
        f => {
            println!("warning, got flag {:02b}, defaulting to 8 bits per sample", f);
            16
        }, // is this a good idea?
    };
    let size_of_chunk: u32 = raw.length;

    // println!("{}", bits_sample);
    BE::write_u32(&mut header[offset_u32!(0x0000)], RIFF);
    LE::write_u32(&mut header[offset_u32!(0x0004)], file_size);
    BE::write_u32(&mut header[offset_u32!(0x0008)], WAVE);
    BE::write_u32(&mut header[offset_u32!(0x000C)], FMT_);
    LE::write_u32(&mut header[offset_u32!(0x0010)], wav_scs);

    LE::write_u16(&mut header[offset_u16!(0x0014)], wav_type);
    LE::write_u16(&mut header[offset_u16!(0x0016)], wav_flag_ms);

    LE::write_u32(&mut header[offset_u32!(0x0018)], sample_frequency);
    LE::write_u32(&mut header[offset_u32!(0x001C)], bytes_sec);

    LE::write_u16(&mut header[offset_u16!(0x0020)], block_align);
    LE::write_u16(&mut header[offset_u16!(0x0022)], bits_sample);

    BE::write_u32(&mut header[offset_u32!(0x0024)], DATA);

    LE::write_u32(&mut header[offset_u32!(0x0028)], size_of_chunk);
    
    // println!("{:04X?}", &header[offset_u16!(0x0022)]);
    header
}



#[derive(Debug)]
pub struct ItSample {
    filename: [char; 12],
    name: [char; 26],
    length: u32,
    sample_pointer: u32,
    sample_rate: u32,
    flags: u8,
}
// #[macro_use]

// #[test]
// fn gen() {
//     let mut a = 0x0000;
//     let b = [
//         4, (12), 1, 1, 1, 1, 26,
//         1, 1,  4, 4, 4, 4, 4,
//         4, 4,  1, 1, 1, 1
//     ];
//     for i in b.iter() {
//         println!("0x{:04X} -> ", a);
//         a += i;
//     };
// }

// #[test]
// fn gen2() {
//     let mut a = 0x0000;
//     let b = [
//         4, 26, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2,
//         1, 1, 1, 1, 1, 1, 2, 4, 4, 64, 64
//     ];
//     for i in b.iter() {
//         println!("0x{:04X} ->  [u{}]", a, i*8);
//         a += i;
//     };
// }

#[test]
fn test1() {
    let a = ItFile::load("samples/Intro_Music.it").unwrap();
    a.export("./test/help.wav", 0).unwrap();
    // for i in 0..89 {
    //     println!("{}", i);
    //     println!(
    //         "sample length: {}\nsample pointer {:04X}\nsample speed: {}\nsample flags: {:08b}\n\n",
    //         &a.samples_meta[i].length,
    //         &a.samples_meta[i].sample_pointer,
    //         &a.samples_meta[i].sample_rate,
    //         &a.samples_meta[i].flags,
    //     );
    // }





    /*
    It seems that obtaining the length and sample pointer alone isn't enough
    maybe see if the sample rate is involved?
    possibly, initialy I assumed that the sample length is in bytes,

    maybe it could be sample length * (bitrate / 2), [CONFIRMED]

    for example a 16 bit sample means every sample is 2 bytes long. 
    If the sample is 12,003 long, we need to get 12,003 * (16 / 2) bytes from the buffer.  

    compared to the original sample, it is 80 bytes shorter
    ITS NOT BROKEN I USED THE WRONG FILE LMFAO

    TODO:
        * construct WAV header, we have the following:
            * samplerate
            * name
            *length

        have a look at what else is needed.

    */

    // println!("{:08X} | {0:}", &a.samples_meta[0].length);
}
