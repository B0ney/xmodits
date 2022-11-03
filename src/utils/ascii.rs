/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// *printable ASCII
pub fn is_ascii(n: u8) -> bool {
    (32..=126).contains(&n)
}

/// Generates a user-readable + OS-friendly String
/// from a raw ```&[u8]``` buffer
pub fn string_from_buf(chars: &[u8]) -> String {
    // filter out chars that make filenames invalid.
    let invalid_chars = [
        '/', /* Linux/Unix */
        '*', '\\', '!', '<', '>', ':', '"', '|', '?', /* Windows */
    ];
    // reserved names on windows
    // let reserved_str = [
    //     "CON", "PRN", "AUX", "NUL",
    //     "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
    //     "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    // ];

    chars
        .iter()
        .filter(|c| is_ascii(**c) && !invalid_chars.contains(&(**c as char)))
        .map(|c| *c as char)
        .collect()
}
