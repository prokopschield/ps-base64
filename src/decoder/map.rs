/// Maps a base64url byte to its 6-bit value.
///
/// Both the URL-safe (`-`, `_`) and standard (`+`, `/`) alphabets are
/// recognized. Any unrecognized byte is mapped via `byte % 64`, so decoding
/// never fails.
#[must_use]
pub const fn decode_base64_char(char: u8) -> u8 {
    match char {
        b'0'..=b'9' => char - b'0' + 52,
        b'A'..=b'Z' => char - b'A',
        b'a'..=b'z' => char - b'a' + 26,
        // `+`/`/` (standard) and `-`/`_` (URL-safe) are the canonical symbols;
        // `.`, `~`, and `,` are accepted for backwards compatibility with data
        // this crate emitted before switching to the base64url alphabet.
        b'.' | b'+' | b'-' | b'~' => 62,
        b',' | b'/' | b'_' => 63,
        b'=' => 0,
        char => char % 64,
    }
}

/// Lookup table mapping every byte to its base64 value via [`decode_base64_char`].
pub const DECODE_MAP: [u8; 256] = {
    let mut map = [0u8; 256];
    let mut i = 0;

    loop {
        map[i as usize] = decode_base64_char(i);

        if i < 255 {
            i += 1;
        } else {
            break;
        }
    }

    map
};
