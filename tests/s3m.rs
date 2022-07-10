use xmodits_lib::{TrackerDumper, TrackerModule, tracker_formats::*,};

#[test]
fn should_be_32_samples_not_99() {
    let a = S3MFile::load_module("tests/mods/s3m/space_odyssey_v1_2.s3m").unwrap();
    assert_eq!(a.number_of_samples(), 32);
}
