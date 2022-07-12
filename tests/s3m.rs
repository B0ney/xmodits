use std::path::Path;
use xmodits_lib::{TrackerDumper, tracker_formats::*};

#[test]
fn s3m_invalid_1() {
    let a = S3MFile::load_module("tests/mods/s3m/invalid.s3m");
    assert!(a.is_err());
}

#[test]
fn s3m_no_samples() {
    let a = S3MFile::load_module("tests/mods/s3m/no_samples.s3m").unwrap();
    let folder = "test/exports/";
    let name = "S3M-please-delete";
    let export_path = Path::new(folder).join(name);
    assert_eq!(a.number_of_samples(), 0);
    assert!(!export_path.exists());
    assert!(a.dump(&folder, name).is_err())
}

#[test]
fn s3m_should_be_32_samples_not_99() {
    let a = S3MFile::load_module("tests/mods/s3m/space_odyssey_v1_2.s3m").unwrap();
    assert_eq!(a.number_of_samples(), 32);
}

#[test]
fn s3m_test_1() {
    let a = S3MFile::load_module("tests/mods/s3m/bluesky.s3m").unwrap();
    assert_eq!(a.number_of_samples(), 10);    
}

#[test]
fn s3m_test_2() {
    let a = S3MFile::load_module("tests/mods/s3m/synth_city.s3m").unwrap();
    assert_eq!(a.number_of_samples(), 20);    
}

#[test]
fn s3m_test_3() {
    let a = S3MFile::load_module("tests/mods/s3m/torq_-_some_song.s3m").unwrap();
    assert_eq!(a.number_of_samples(), 9);    
}

#[test]
fn s3m_test_4() {
    let a = S3MFile::load_module("tests/mods/s3m/arc-cell.s3m").unwrap();
    assert_eq!(a.number_of_samples(), 6);    
}