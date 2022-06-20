use super::it::*;

#[test]
fn test1() {
    let a = ItFile::load("samples/NYCStreets_Music.it").unwrap();
    // a.export("./test/longhorn_test_5.wav", 9).unwrap();
    for i in 0..a.sample_number {
        let _ = a.export(format!("./test/{}.wav", i), i as usize);
    }
    

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
}