use xmodits::{TrackerDumper, tracker_formats::*,};
// Currenlty experimenting with designing an api
fn main() {
    println!("Hello, world!");
    let p = "samples/s3m/city_on_a_stick.s3m";
    let a = 4;

    let tracker_module = match a {
        2 => ITFile::load_module(p),
        3 => MODFile::load_module(p),
        4 => S3MFile::load_module(p),
        // 5 => ITFile::load_module(p),
        // 6 => ITFile::load_module(p),
        _ => todo!()
    };

    tracker_module.unwrap().export(&"./test/", 0).unwrap()
} 
