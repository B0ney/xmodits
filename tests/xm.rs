use std::{path::{Path, PathBuf}, io, fs, fmt::format};
// use sha2::{Sha256, Digest};
use xmodits_lib::{TrackerDumper, tracker_formats::*};
mod utils;
use utils::{clean_test_export, hash_file, compare_files};

#[test]
fn xm_empty() {
    assert!(Path::new("tests/mods/xm/invalid.xm").exists());
    assert!(XMFile::load_module("tests/mods/xm/invalid.xm").is_err());
}

#[test]
fn xm_test_mod_plugin_packed() {
    assert!(Path::new("tests/mods/xm/vagyakozas.xm").exists());
    assert!(XMFile::load_module("tests/mods/xm/vagyakozas.xm").is_err());
}

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
/* ####################################################################### */

#[test]
fn xm_test_1() {
    let a = XMFile::load_module("tests/mods/xm/DEADLOCK.XM").unwrap();
    verify_sample_num(
        32,
        a.number_of_samples(),
        a.module_name()
    );
}

#[test]
fn xm_test_2() {
    let a = XMFile::load_module("tests/mods/xm/lovetrp.xm").unwrap();
    verify_sample_num(
        41,
        a.number_of_samples(),
        a.module_name()
    );
}

#[test]
fn xm_test_3() {
    let a = XMFile::load_module("tests/mods/xm/sweetdre.xm").unwrap();
    verify_sample_num(
        24,
        a.number_of_samples(),
        a.module_name()
    );
}

#[test]
fn xm_test_4() {
    let a = XMFile::load_module("tests/mods/xm/xo-sat.xm").unwrap();
    verify_sample_num(
        30,
        a.number_of_samples(),
        a.module_name()
    );
}

#[test]
fn xm_test_pat_pak_1() {
    let a = XMFile::load_module("tests/mods/xm/skuter_-_mind_validator.xm").unwrap();
    verify_sample_num(
        24,
        a.number_of_samples(),
        a.module_name()
    );
} 

#[test]
fn xm_test_pat_pak_2() {
    let a = XMFile::load_module("tests/mods/xm/skuter_-_memoirs.xm").unwrap();
    verify_sample_num(
        7, 
        a.number_of_samples(),
        a.module_name()
    )
} 

pub fn verify_sample_num(expected: usize, given: usize, modname: &str) {
    assert_eq!(
        expected, given, 
        "{}",format!("\n\nMODNAME: {}\n     EXPECTED: {} SAMPLES, GOT: {} INSTEAD\n\n",modname, expected, given)
    );
}
/* ####################################################################### */
// Verify exported samples match. this is useful when optimising functions.
// Optimising can be destructive, so thorough testing is needed.

#[test]
fn xm_test_exported() {
    let test_no: usize = 0;
    let root: &Path = Path::new("tests/export/xm/");
    let test_export_path: PathBuf = PathBuf::new().join(root).join(format!("test_export_{}/",test_no));
    let mod1 = XMFile::load_module("tests/mods/xm/lovetrp.xm").unwrap();

    clean_test_export(root, test_no).unwrap();

    mod1.export(&test_export_path, 0).unwrap();
    mod1.export(&test_export_path, 1).unwrap();
    mod1.export(&test_export_path, 17).unwrap();
    mod1.export(&test_export_path, 26).unwrap();

    let files = vec![
        ("01.wav", "smp_1_8bit"),
        ("02.wav", "smp_2_8bit"),
        ("18 - Ody-rng5.wav", "smp_1_16bit_18"),
        ("27 - Ody-lde1.wav", "smp_2_16bit_27")
    ];

    compare_files(files, test_export_path, root);
}

/* Verify samples ripped from modules with amiga freq flag
   I'm doing this to confirm, that it does nothing to the actual data.
*/
#[test]
fn xm_test_exported_amiga() {
    let test_no: usize = 1;
    let root: &Path = Path::new("tests/export/xm/");
    let test_export_path: PathBuf = PathBuf::new().join(root).join(format!("test_export_{}/",test_no));
    let mod1 = XMFile::load_module("tests/mods/xm/240-185_-_la_grenade_80s.xm").unwrap();

    clean_test_export(root, test_no).unwrap();

    mod1.export(&test_export_path, 0).unwrap();
    mod1.export(&test_export_path, 1).unwrap();
    mod1.export(&test_export_path, 19).unwrap();
    mod1.export(&test_export_path, 20).unwrap();

    let files = vec![
        ("01.wav", "smp_1_8bit_amig"),
        ("02.wav", "smp_2_8bit_amig"),
        ("20.wav", "smp_1_16bit_amig"),
        ("21.wav", "smp_2_16bit_amig")
    ];

    compare_files(files, test_export_path, root);
}