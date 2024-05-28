pub const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789~_";

pub fn encode(input: &[u8]) -> String {
    let mut output = Vec::with_capacity(input.len() * 4 / 3 + 1);

    for chunk in input.chunks_exact(3) {
        output.push(ALPHABET[(chunk[0] >> 2) as usize]);
        output.push(ALPHABET[(((chunk[0] & 0x3) << 4) | (chunk[1] >> 4)) as usize]);
        output.push(ALPHABET[(((chunk[1] & 0xf) << 2) | (chunk[2] >> 6)) as usize]);
        output.push(ALPHABET[(chunk[2] & 0x3f) as usize]);
    }

    let remainder = input.len() % 3;

    if remainder == 1 {
        let index: usize = input.len() - 1;

        output.push(ALPHABET[(input[index] >> 2) as usize]);
        output.push(ALPHABET[((input[index] & 3) << 4) as usize]);
    } else if remainder == 2 {
        let index: usize = input.len() - 2;

        output.push(ALPHABET[(input[index] >> 2) as usize]);
        output.push(ALPHABET[(((input[index] & 0x3) << 4) | (input[index + 1] >> 4)) as usize]);
        output.push(ALPHABET[((input[index + 1] & 0xf) << 2) as usize]);
    }

    // ALPHABET does not contain invalid utf8 chars
    unsafe { String::from_utf8_unchecked(output) }
}
