use std::path::{Path, PathBuf};
mod utils;
use utils::{clean_test_export, compare_files, verify_sample_num};

use xmodits_lib::{TrackerDumper, tracker_formats::*};

#[test]
fn it_empty() {
    assert!(Path::new("tests/mods/it/empty.it").exists());
    assert!(ITFile::load_module("tests/mods/it/empty.it").is_err());
}

#[test]
fn it_test_mmcmp() {
    assert!(Path::new("tests/mods/it/creagaia.it").exists());
    assert!(ITFile::load_module("tests/mods/it/creagaia.it").is_err());
}

// We don't want xmodits to create an empty folder when attempting to dump an empty module
#[test]
fn it_no_samples() {
    let a = ITFile::load_module("tests/mods/it/no_samples.it").unwrap();
    let folder = "tests/exports/";
    let name = "IT-please-delete";
    let export_path = Path::new(folder).join(name);

    assert_eq!(a.number_of_samples(), 0);
    assert!(!export_path.exists());

    assert!(a.dump(&export_path, true).is_err())
}

check_sample_number!(
    it_should_be_17_samples_not_18
    path: "tests/mods/it/17_samples.it",
    with: 17
);

check_sample_number!(
    it_test_1
    path: "tests/mods/it/asikwp_-_fc-freedrive_chiptune.it",
    with: 9
);

check_sample_number!(
    it_test_2
    path: "tests/mods/it/beyond_-_flute.it",
    with: 2
);

check_sample_number!(
    it_test_3
    path: "tests/mods/it/sm-safari.it",
    with: 19
);

check_sample_number!(
    it_test_4
    path: "tests/mods/it/songofthesky.it",
    with: 14
);

/* ####################################################################### */

#[test]
fn it_test_exported() {
    let test_no: usize = 0;
    let root: &Path = Path::new("tests/export/it/");
    let test_export_path: PathBuf = PathBuf::new().join(root).join(format!("test_export_{}/",test_no));
    let mod1 = ITFile::load_module("tests/mods/it/songofthesky.it").unwrap();

    clean_test_export(root, test_no).unwrap();

    mod1.export(&test_export_path, 0).unwrap();
    mod1.export(&test_export_path, 1).unwrap();
    mod1.export(&test_export_path, 6).unwrap();
    mod1.export(&test_export_path, 8).unwrap();

    mod1.export(&test_export_path, 0).unwrap();
    mod1.export(&test_export_path, 1).unwrap();
    mod1.export(&test_export_path, 6).unwrap();
    mod1.export(&test_export_path, 8).unwrap();

    let files = vec![
        ("01 - MEDP1_PAT.wav",      "smp_1_8bit"),
        ("02 - Left strings.wav",   "smp_2_8bit"),
        ("07 - Pad-st~1.wav",       "smp_1_16bit"),
        ("09 - Timp.wav",           "smp_2_16bit")
    ];

    compare_files(files, test_export_path, root);
}

#[test]
fn it_test_exported_compression() {
    let test_no: usize = 1;
    let root: &Path = Path::new("tests/export/it/");
    let test_export_path: PathBuf = PathBuf::new().join(root).join(format!("test_export_{}/",test_no));
    let mod1 = ITFile::load_module("tests/mods/it/before_the_explozion.it").unwrap();

    clean_test_export(root, test_no).unwrap();

    mod1.export(&test_export_path, 0).unwrap();
    mod1.export(&test_export_path, 1).unwrap();
    mod1.export(&test_export_path, 2).unwrap();
    mod1.export(&test_export_path, 4).unwrap();

    let files = vec![
        ("01 - STEPPZ_WAVV.wav", "smp_1_16bit_comp"),
        ("02 - COLONY56_IT.wav", "smp_2_16bit_comp"),
        ("03 - COLONY56_IT.wav", "smp_1_8bit_comp"),
        ("05 - PUSHMIND_IT.wav", "smp_2_8bit_comp")
    ];

    compare_files(files, test_export_path, root);
}