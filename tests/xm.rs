
use std::path::Path;
use xmodits_lib::{TrackerDumper, tracker_formats::*};

#[test]
fn xm_empty() {
    let a = XMFile::load_module("tests/mods/xm/invalid.xm");
    assert!(a.is_err());
}

// #[test]
// fn xm_unsupported() {
//     let a = XMFile::load_module("tests/mods/xm/synthma.xm");
//     assert!(a.is_err());
// }
#[ignore = "not yet done"]
#[test]
fn xm_no_samples() {
    let a = XMFile::load_module("tests/mods/xm/no_sample.xm").unwrap();
    let folder = "test/exports/";
    let name = "MOD-please-delete";
    let export_path = Path::new(folder).join(name);
    
    assert_eq!(a.number_of_samples(), 0);
    assert!(!export_path.exists());
    assert!(a.dump(&folder, name).is_err())
}

#[test]
fn xm_test_1() {
    let a = XMFile::load_module("tests/mods/xm/DEADLOCK.XM").unwrap();
    assert_eq!(a.number_of_samples(), 32);
}

#[test]
fn xm_test_2() {
    let a = XMFile::load_module("tests/mods/xm/lovetrp.xm").unwrap();
    assert_eq!(a.number_of_samples(), 41);
}

#[test]
fn xm_test_3() {
    let a = XMFile::load_module("tests/mods/xm/sweetdre.xm").unwrap();
    assert_eq!(a.number_of_samples(), 24);
}

#[test]
fn xm_test_4() {
    let a = XMFile::load_module("tests/mods/xm/xo-sat.xm").unwrap();
    assert_eq!(a.number_of_samples(), 30);
}


#[test]
fn xm_test_pat_pak_1() {
    let a = XMFile::load_module("tests/mods/xm/skuter_-_mind_validator.xm").unwrap();
    assert_eq!(a.number_of_samples(), 24);
} 


#[test]
fn xm_test_pat_pak_2() {
    let a = XMFile::load_module("tests/mods/xm/skuter_-_memoirs.xm").unwrap();
    assert_eq!(a.number_of_samples(), 7);
} 