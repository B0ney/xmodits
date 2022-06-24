use xmodits::{TrackerDumper, ITFile,MODFile};
// Currenlty experimenting with designing an api
fn main() {
    println!("Hello, world!");
    let p = "samples/comp/worldies.it";
    let a = 2;

    let tracker_module = match a {
        2 => ITFile::load_module(p),
        3 => MODFile::load_module(p),
        4 => ITFile::load_module(p),
        5 => ITFile::load_module(p),
        6 => ITFile::load_module(p),
        _ => todo!()
    };

    tracker_module.unwrap().export(&"./test/testapi.wav", 0).unwrap()

    // match ITFile::load_module("samples/comp/worldies.it") {
    //     Ok(dumpy) => {
    //         if let Err(e) = dumpy.export(&"./test/testapi.wav", 0) {
    //             println!("{:?}",e)
    //         };
    //     },
    //     Err(e) => {println!("{:?}",e);}
    // };
} 
