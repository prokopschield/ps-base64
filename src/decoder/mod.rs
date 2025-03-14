pub mod map;

pub use map::*;

#[allow(clippy::identity_op)]
#[must_use]
pub fn decode_base64_block(input: &[u8]) -> [u8; 3] {
    let mut value: u32 = 0;

    for index in 0..input.len() {
        value <<= 6;
        value += u32::from(DECODE_MAP[input[index] as usize]);
    }

    for _ in input.len()..4 {
        value <<= 6;
    }

    [
        ((value >> 0x10) & 0xFF) as u8,
        ((value >> 0x08) & 0xFF) as u8,
        ((value >> 0x00) & 0xFF) as u8,
    ]
}

#[inline]
const fn align_up(size: usize) -> usize {
    let remainder = size % 4;

    if remainder != 0 {
        size + 4 - remainder
    } else {
        size
    }
}

#[inline]
const fn three_fourths(size: usize) -> usize {
    align_up(size) * 3 / 4
}

#[must_use]
pub fn decode(input: &[u8]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::with_capacity(three_fourths(input.len()));

    for chunk in input.chunks(4) {
        output.extend(decode_base64_block(chunk).iter());
    }

    output
}
