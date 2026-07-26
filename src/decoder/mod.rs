pub mod map;

pub use map::*;

#[inline]
pub(crate) const fn align_up(size: usize) -> usize {
    (size + 3) & !3
}

#[inline]
pub(crate) const fn three_fourths(size: usize) -> usize {
    (size * 3).div_ceil(4)
}

/// Decodes base64url `input` into a [`Vec<u8>`].
///
/// Decoding is lenient: ASCII whitespace and `=` are skipped, and every other
/// byte is accepted (see [`decode_base64_char`]). Both the URL-safe (`-`, `_`)
/// and standard (`+`, `/`) alphabets are recognized.
///
/// The output length depends only on the number of accepted characters, never
/// on their values: every complete group of four characters yields three
/// bytes, and a trailing group of one, two, or three characters yields one,
/// one, or two bytes. Leftover bits in a trailing group are discarded.
#[must_use]
pub fn decode(input: &[u8]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::with_capacity(align_up(three_fourths(input.len())));

    let mut iterator = input
        .iter()
        .copied()
        .filter(|&byte| !byte.is_ascii_whitespace() && byte != b'=');

    loop {
        let mut value = 0;

        if let Some(b) = iterator.next() {
            value |= u32::from(DECODE_MAP[usize::from(b)]) << 18;
        } else {
            break;
        }

        if let Some(b) = iterator.next() {
            value |= u32::from(DECODE_MAP[usize::from(b)]) << 12;
        } else {
            output.push(value.to_be_bytes()[1]);
            break;
        }

        if let Some(b) = iterator.next() {
            value |= u32::from(DECODE_MAP[usize::from(b)]) << 6;
        } else {
            output.push(value.to_be_bytes()[1]);

            break;
        }

        if let Some(b) = iterator.next() {
            value |= u32::from(DECODE_MAP[usize::from(b)]);
        } else {
            let bytes = value.to_be_bytes();

            output.push(bytes[1]);
            output.push(bytes[2]);

            break;
        }

        output.extend_from_slice(&value.to_be_bytes()[1..]);
    }

    output
}

/// Decodes base64url `input` into a fixed-size `[u8; S]` buffer.
///
/// Decoding stops once `S` bytes are produced or the input is exhausted; any
/// remaining input is ignored, and any unfilled trailing bytes are left zero.
/// Leniency matches [`decode`].
#[must_use]
pub fn sized_decode<const S: usize>(input: &[u8]) -> [u8; S] {
    let mut output = [0u8; S];
    let mut iterator = input
        .iter()
        .copied()
        .filter(|&byte| !byte.is_ascii_whitespace() && byte != b'=');
    let mut decoded_bytes = 0;

    loop {
        let mut value = 0;

        if decoded_bytes >= S {
            break;
        }

        if let Some(b) = iterator.next() {
            value |= u32::from(DECODE_MAP[usize::from(b)]) << 18;
        } else {
            break;
        }

        if let Some(b) = iterator.next() {
            value |= u32::from(DECODE_MAP[usize::from(b)]) << 12;
        } else {
            output[decoded_bytes] = value.to_be_bytes()[1];
            break;
        }

        if decoded_bytes + 1 >= S {
            let bytes = value.to_be_bytes();
            output[decoded_bytes] = bytes[1];
            break;
        }

        if let Some(b) = iterator.next() {
            value |= u32::from(DECODE_MAP[usize::from(b)]) << 6;
        } else {
            let bytes = value.to_be_bytes();
            output[decoded_bytes] = bytes[1];
            output[decoded_bytes + 1] = bytes[2];
            break;
        }

        if decoded_bytes + 2 >= S {
            let bytes = value.to_be_bytes();
            output[decoded_bytes] = bytes[1];
            output[decoded_bytes + 1] = bytes[2];
            break;
        }

        if let Some(b) = iterator.next() {
            value |= u32::from(DECODE_MAP[usize::from(b)]);
        } else {
            let bytes = value.to_be_bytes();
            output[decoded_bytes] = bytes[1];
            output[decoded_bytes + 1] = bytes[2];
            output[decoded_bytes + 2] = bytes[3];
            break;
        }

        let bytes = value.to_be_bytes();

        output[decoded_bytes..decoded_bytes + 3].copy_from_slice(&bytes[1..]);

        decoded_bytes += 3;
    }
    output
}
