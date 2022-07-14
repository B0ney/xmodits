use std::path::Path;
use xmodits_lib::{TrackerDumper, tracker_formats::*};

#[ignore = "todo"]
#[test]
fn mod_empty() {
    let a = MODFile::load_module("tests/mods/mod/empty.mod");
    assert!(a.is_err());
}

#[test]
fn mod_unsupported() {
    let a = MODFile::load_module("tests/mods/mod/synthmat.mod");
    assert!(a.is_err());
}

#[test]
fn mod_no_samples() {
    let a = MODFile::load_module("tests/mods/mod/no_samples.mod").unwrap();
    let folder = "test/exports/";
    let name = "MOD-please-delete";
    let export_path = Path::new(folder).join(name);
    
    assert_eq!(a.number_of_samples(), 0);
    assert!(!export_path.exists());
    assert!(a.dump(&folder, name).is_err())
}

#[test]
fn mod_test_1() {
    let a = MODFile::load_module("tests/mods/mod/echobea3.mod").unwrap();
    assert_eq!(a.number_of_samples(), 15);
}

#[test]
fn mod_test_2() {
    let a = MODFile::load_module("tests/mods/mod/slash-kill-maim-hit.mod").unwrap();
    assert_eq!(a.number_of_samples(), 19);
}

#[test]
fn mod_test_3() {
    let a = MODFile::load_module("tests/mods/mod/chop.mod").unwrap();
    assert_eq!(a.number_of_samples(), 5);
}

#[test]
fn mod_test_4() {
    let a = MODFile::load_module("tests/mods/mod/sleep.mod").unwrap();
    assert_eq!(a.number_of_samples(), 9);
}