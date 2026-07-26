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

/// Sentinel marking a byte that decoding skips. No 6-bit value has this bit
/// set, so it cannot collide with a decoded character.
const SKIP: u8 = 0x80;

/// Like [`DECODE_MAP`], but the bytes decoding skips (ASCII whitespace and `=`)
/// map to [`SKIP`] rather than to a 6-bit value.
///
/// This lets a whole four-character group be tested for skipped bytes with a
/// single mask instead of a predicate per byte. It is deliberately private:
/// [`DECODE_MAP`] is public and documented to map every byte to a 6-bit value.
const SKIP_MAP: [u8; 256] = {
    let mut map = [0u8; 256];
    let mut i = 0;

    loop {
        map[i as usize] = match i {
            b' ' | b'\t' | b'\n' | b'\x0C' | b'\r' | b'=' => SKIP,
            _ => decode_base64_char(i),
        };

        if i < 255 {
            i += 1;
        } else {
            break;
        }
    }

    map
};

/// Decodes base64url `input` into a [`Vec<u8>`].
///
/// Decoding is lenient: ASCII whitespace and `=` are skipped, and every other
/// byte is accepted (see [`decode_base64_char`]). Both the URL-safe (`-`, `_`)
/// and standard (`+`, `/`) alphabets are recognized.
///
/// The output length depends only on the number of accepted characters, never
/// on their values. Every complete group of four characters yields three
/// bytes, and a trailing group yields one byte for one or two characters, or
/// two bytes for three characters. Leftover bits in a trailing group are
/// discarded.
#[must_use]
pub fn decode(input: &[u8]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::with_capacity(align_up(three_fourths(input.len())));

    // SAFETY (for every write below): let `k` be the number of bytes of `input`
    // that are not skipped, so `k <= input.len()`. This function writes exactly
    // `k / 4 * 3 + [0, 1, 1, 2][k % 4]` bytes, which is at most
    // `three_fourths(k) <= three_fourths(input.len())`, and that is the capacity
    // reserved above (before rounding it up). Every write therefore lands inside
    // the allocation, and `out` stays valid because `output` is never grown.
    let out = output.as_mut_ptr();
    let mut len = 0usize;
    let mut index = 0usize;

    for chunk in input.chunks_exact(4) {
        let a = SKIP_MAP[usize::from(chunk[0])];
        let b = SKIP_MAP[usize::from(chunk[1])];
        let c = SKIP_MAP[usize::from(chunk[2])];
        let d = SKIP_MAP[usize::from(chunk[3])];

        // Fall back to the byte-at-a-time loop at the first skipped byte, which
        // is where groups stop lining up with four-character boundaries.
        if (a | b | c | d) & SKIP != 0 {
            break;
        }

        let group = [(a << 2) | (b >> 4), (b << 4) | (c >> 2), (c << 6) | d];

        unsafe { core::ptr::copy_nonoverlapping(group.as_ptr(), out.add(len), 3) };

        len += 3;
        index += 4;
    }

    let mut iterator = input[index..]
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
            unsafe { out.add(len).write(value.to_be_bytes()[1]) };

            len += 1;

            break;
        }

        if let Some(b) = iterator.next() {
            value |= u32::from(DECODE_MAP[usize::from(b)]) << 6;
        } else {
            unsafe { out.add(len).write(value.to_be_bytes()[1]) };

            len += 1;

            break;
        }

        if let Some(b) = iterator.next() {
            value |= u32::from(DECODE_MAP[usize::from(b)]);
        } else {
            let bytes = value.to_be_bytes();

            unsafe { core::ptr::copy_nonoverlapping(bytes[1..].as_ptr(), out.add(len), 2) };

            len += 2;

            break;
        }

        let bytes = value.to_be_bytes();

        unsafe { core::ptr::copy_nonoverlapping(bytes[1..].as_ptr(), out.add(len), 3) };

        len += 3;
    }

    unsafe { output.set_len(len) };

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
