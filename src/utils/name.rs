/// Format name of exported sample.
///
/// If the sample name is empty it'll just be: $n.wav e.g 0.wav
/// 
/// If the sample does have a name, it'll be "$n - $name.wav"
pub fn name_sample(idx: usize, name: &str) -> String {
    format!(
        "{}{}.wav",
        idx + 1, // use human readable indexing.
        match name.trim() {
            x if x.is_empty() => "".to_string(),
            x => format!(
                " - {}", 
                x.split('.').collect::<Vec<&str>>()[0]
            ),
        }
    )
}

#[test]
fn test1() {
    let strw = "               ".trim();
    println!("{:?}", strw.is_empty()); 
}

#[test]
fn test2() {
    let strw = "ooga.wav".split('.').collect::<Vec<&str>>()[0];
    println!("{:?}", strw); 
}
#[test]
fn test3() { // is this desirable?
    let strw = "ooga v1.2 e.wav".split('.').collect::<Vec<&str>>()[0];
    println!("{:?}", strw); 
}