
use std::{path::Path, fs};
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

/* ####################################################################### */

#[test]
fn xm_test_export_1_8bit() {}

#[test]
fn xm_test_export_2_8bit() {}

#[test]
fn xm_test_export_1_16bit() {}

#[test]
fn xm_test_export_2_16bit() {}

/* Verify samples ripped from modules with amiga freq flag
   I'm doing this to confirm, that it does nothing to the actual data.
*/

// #[test]
// fn xm_test_export_1_8bit_amig() {
//     let mod1 = XMFile::load_module("tests/mods/xm/240-185_-_la_grenade_80s.xm").unwrap();
//     mod1.export(&"tests/export/xm/test_export/", 0).unwrap();
//     mod1.export(&"tests/export/xm/test_export/", 1).unwrap();

// }

// #[test]
// fn xm_test_export_2_8bit_amig() {
    
// }

// #[test]
// fn xm_test_export_1_16bit_amig() {
//     let mod1 = XMFile::load_module("tests/mods/xm/240-185_-_la_grenade_80s.xm").unwrap();
//     mod1.export(&"tests/export/xm/test_export/", 19).unwrap();
//     mod1.export(&"tests/export/xm/test_export/", 20).unwrap();

// }

// #[test]
// fn xm_test_export_2_16bit_amig() {}