/// Format name of exported sample.
///
/// If the sample name is empty it'll just be: $n.wav e.g 0.wav
/// 
/// If the sample does have a name, it'll be "$n - $name.wav"
pub fn name_sample(idx: usize, name: &str) -> String {
    format!(
        "{}{}.wav",
        idx,
        match name.trim() {
            x if x.is_empty() => "".to_string(),
            x => format!(" - {}", x),
        }
    )
}

#[test]
fn test1() {
    let strw = "               ".trim();
    println!("{:?}", strw.is_empty()); 
}