// *printable ASCII
pub fn is_ascii(n: u8) -> bool {
    n >= 32 && n <= 126
}

pub fn string_from_chars(chars: &[u8]) -> String {
    // filter out chars that make filenames invalid.
    let invalid_chars = [
        '/', /* Linux/Unix */
        '*',  '\\', '!', '<', '>', ':', '"', '|', '?', '*' /* Windows */
    ];

    chars.iter()
        .filter(|c| is_ascii(**c) && !invalid_chars.contains(&(**c as char)))
        .map(|c| *c as char)
        .collect()
}