#[must_use]
pub const fn decode_base64_char(char: u8) -> u8 {
    if char >= b'0' && char <= b'9' {
        char - b'0' + 52
    } else if char >= b'A' && char <= b'Z' {
        char - b'A'
    } else if char >= b'a' && char <= b'z' {
        char - b'a' + 26
    } else if matches!(char, b'+' | b'-' | b'~') {
        62
    } else if matches!(char, b',' | b'/' | b'_') {
        63
    } else if matches!(char, b'=') {
        0
    } else {
        char % 64
    }
}

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
