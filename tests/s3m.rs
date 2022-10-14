use std::path::{Path, PathBuf};
use xmodits_lib::{TrackerDumper, tracker_formats::*};
mod utils;
use utils::{clean_test_export, compare_files, verify_sample_num};


#[test]
fn s3m_invalid_1() {
    assert!(Path::new("tests/mods/s3m/invalid.s3m").exists());
    assert!(S3MFile::load_module("tests/mods/s3m/invalid.s3m").is_err());
}

#[test]
fn s3m_no_samples() {
    let mut a = S3MFile::load_module("tests/mods/s3m/no_samples.s3m").unwrap();
    let folder = "tests/exports/";
    let name = "S3M-please-delete";
    let dest = Path::new(folder).join(name);

    let export_path = Path::new(folder).join(name);
    assert_eq!(a.number_of_samples(), 0);
    assert!(!export_path.exists());
    assert!(a.dump(&dest, true).is_err())
}

check_sample_number!(
    s3m_should_be_32_samples_not_99
    path: "tests/mods/s3m/space_odyssey_v1_2.s3m",
    with: 32
);

check_sample_number!(
    s3m_test_0
    path: "tests/mods/s3m/no_samples.s3m",
    with: 0
);

check_sample_number!(
    s3m_test_1
    path: "tests/mods/s3m/bluesky.s3m",
    with: 10
);

check_sample_number!(
    s3m_test_2
    path: "tests/mods/s3m/synth_city.s3m",
    with: 20
);

check_sample_number!(
    s3m_test_3
    path: "tests/mods/s3m/torq_-_some_song.s3m",
    with: 9
);

check_sample_number!(
    s3m_test_4
    path: "tests/mods/s3m/arc-cell.s3m",
    with: 6
);

/* ####################################################################### */

#[test]
fn s3m_test_exported() {
    let test_no: usize = 0;
    let root: &Path = Path::new("tests/export/s3m/");
    let test_export_path: PathBuf = PathBuf::new().join(root).join(format!("test_export_{}/",test_no));
    let mut mod1 = S3MFile::load_module("tests/mods/s3m/hip_-_640k_of_space.s3m").unwrap();
    dbg!(&test_export_path);
    clean_test_export(root, test_no).unwrap();

    mod1.export(&test_export_path, 0).unwrap();
    mod1.export(&test_export_path, 0).unwrap();

    mod1.export(&test_export_path, 1).unwrap();
    mod1.export(&test_export_path, 5).unwrap();
    mod1.export(&test_export_path, 6).unwrap();

    let files = vec![
        ("01 - PAD853.wav",   "smp_1_16bit"),
        ("02 - WIN3_1.wav",   "smp_2_16bit"),
        ("06 - CHIP2.wav",    "smp_1_8bit"),
        ("07 - CHIPB.wav",    "smp_2_8bit")
    ];

    compare_files(files, test_export_path, root);
}