// *printable ASCII
pub fn is_ascii(n: u8) -> bool {
    (32..=126).contains(&n)
}

pub fn string_from_chars(chars: &[u8]) -> String {
    // filter out chars that make filenames invalid.
    let invalid_chars = [
        '/', /* Linux/Unix */
        '*',  '\\', '!', '<', '>', ':', '"', '|', '?' /* Windows */
    ];
    // reserved names on windows
    // let reserved_str = [
    //     "CON", "PRN", "AUX", "NUL", 
    //     "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
    //     "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    // ];

    chars.iter()
        .filter(|c| 
            is_ascii(**c) && 
            !invalid_chars.contains(&(**c as char))
        )
        .map(|c| *c as char)
        .collect()
}