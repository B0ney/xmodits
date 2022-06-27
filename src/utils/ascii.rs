pub fn is_ascii(n: u8) -> bool {
    n >= 32 && n <= 126
}

pub fn string_from_chars(chars: &[u8]) -> String {
    chars.iter()
        .filter(|c| is_ascii(**c))
        .map(|c| *c as char)
        .collect()
}