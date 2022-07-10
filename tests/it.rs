use xmodits_lib::{Error, TrackerDumper, TrackerModule, tracker_formats::*,};

#[test]
fn should_be_17_samples_not_18() {
    let a = ITFile::load_module("tests/mods/it/17_samples.it").unwrap();
    assert_eq!(a.number_of_samples(), 17);
}
