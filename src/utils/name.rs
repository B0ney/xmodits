/// Format name of exported sample.
///
/// If the sample name is empty it'll just be: $n.wav e.g 0.wav
/// 
/// If the sample does have a name, it'll be "$n - $name.wav"
pub fn name_sample(sample: &crate::TrackerSample, idx: usize) -> String {
    format!(
        "{:02}{}.wav",
        idx + 1, // use human readable indexing.
        match &sample.filename.trim() {
            x if x.is_empty() => "".to_string(),
            x => format!(
                " - {}", 
                x.replace(".wav", "").replace(".", "_")
            ),
        }
    )
}