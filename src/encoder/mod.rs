/// The base64url output alphabet, indexed by 6-bit value.
pub const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// Returns the length of the unpadded base64url encoding of `len` input
/// bytes.
///
/// Every complete group of three bytes yields four characters, and a trailing
/// byte yields two characters, or three for two bytes.
#[inline]
#[must_use]
pub const fn encoded_len(len: usize) -> usize {
    len / 3 * 4
        + match len % 3 {
            0 => 0,
            1 => 2,
            _ => 3,
        }
}

/// Encodes `input` as base64url into a fixed-size `[u8; S]` buffer.
///
/// The buffer is right-padded with `=` when the encoding is shorter than `S`,
/// and the encoding is silently truncated when it is longer than `S`.
#[inline]
#[must_use]
pub fn sized_encode<const S: usize>(input: &[u8]) -> [u8; S] {
    let mut output = [b'='; S];
    let mut index = 0;

    let mut push = |byte: u8| {
        if index < S {
            output[index] = byte;
            index += 1;
        }
    };

    for chunk in input.chunks_exact(3) {
        push(ALPHABET[(chunk[0] >> 2) as usize]);
        push(ALPHABET[(((chunk[0] & 0x3) << 4) | (chunk[1] >> 4)) as usize]);
        push(ALPHABET[(((chunk[1] & 0xf) << 2) | (chunk[2] >> 6)) as usize]);
        push(ALPHABET[(chunk[2] & 0x3f) as usize]);
    }

    let remainder = input.len() % 3;

    if remainder == 1 {
        let index: usize = input.len() - 1;

        push(ALPHABET[(input[index] >> 2) as usize]);
        push(ALPHABET[((input[index] & 3) << 4) as usize]);
    } else if remainder == 2 {
        let index: usize = input.len() - 2;

        push(ALPHABET[(input[index] >> 2) as usize]);
        push(ALPHABET[(((input[index] & 0x3) << 4) | (input[index + 1] >> 4)) as usize]);
        push(ALPHABET[((input[index + 1] & 0xf) << 2) as usize]);
    }

    output
}

/// Encodes `input` as unpadded base64url and returns it as a [`String`].
///
/// The output uses the URL-safe alphabet (`-` and `_` for 62 and 63) and
/// carries no `=` padding.
#[must_use]
pub fn encode(input: &[u8]) -> String {
    let mut output = Vec::with_capacity((input.len() * 4).div_ceil(3));

    for chunk in input.chunks_exact(3) {
        output.extend_from_slice(&[
            ALPHABET[(chunk[0] >> 2) as usize],
            ALPHABET[(((chunk[0] & 0x3) << 4) | (chunk[1] >> 4)) as usize],
            ALPHABET[(((chunk[1] & 0xf) << 2) | (chunk[2] >> 6)) as usize],
            ALPHABET[(chunk[2] & 0x3f) as usize],
        ]);
    }

    let remainder = input.len() % 3;
    let len = input.len();

    if remainder == 1 {
        output.push(ALPHABET[(input[len - 1] >> 2) as usize]);
        output.push(ALPHABET[((input[len - 1] & 3) << 4) as usize]);
    } else if remainder == 2 {
        output.push(ALPHABET[(input[len - 2] >> 2) as usize]);
        output.push(ALPHABET[(((input[len - 2] & 0x3) << 4) | (input[len - 1] >> 4)) as usize]);
        output.push(ALPHABET[((input[len - 1] & 0xf) << 2) as usize]);
    }

    // ALPHABET does not contain invalid utf8 chars
    unsafe { String::from_utf8_unchecked(output) }
}

/// Returns an adapter that encodes `input` as unpadded base64url when
/// formatted with [`core::fmt::Display`].
///
/// This lets an encoding be embedded directly in `format_args!`-style macros,
/// streaming into the formatter via [`encode_into`] without an intermediate
/// allocation.
#[must_use]
pub fn display(input: &[u8]) -> impl core::fmt::Display + '_ {
    struct Adapter<'a>(&'a [u8]);

    impl core::fmt::Display for Adapter<'_> {
        fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            encode_into(self.0, formatter)
        }
    }

    Adapter(input)
}

/// Encodes `input` and writes directly into a [`core::fmt::Write`].
///
/// # Errors
///
/// Errors are passed from [`core::fmt::Write::write_str`].
#[inline]
pub fn encode_into<W>(input: &[u8], mut sink: W) -> core::fmt::Result
where
    W: core::fmt::Write,
{
    let mut push = |s: &[u8]| sink.write_str(unsafe { str::from_utf8_unchecked(s) });

    for chunk in input.chunks_exact(3) {
        let a = ALPHABET[(chunk[0] >> 2) as usize];
        let b = ALPHABET[(((chunk[0] & 0x3) << 4) | (chunk[1] >> 4)) as usize];
        let c = ALPHABET[(((chunk[1] & 0xf) << 2) | (chunk[2] >> 6)) as usize];
        let d = ALPHABET[(chunk[2] & 0x3f) as usize];

        push(&[a, b, c, d])?;
    }

    match input.len() % 3 {
        1 => {
            let c = input[input.len() - 1];

            push(&[
                ALPHABET[(c >> 2) as usize],
                ALPHABET[((c & 3) << 4) as usize],
            ])?;
        }
        2 => {
            let (a, b) = {
                let len = input.len();
                (input[len - 2], input[len - 1])
            };

            push(&[
                ALPHABET[(a >> 2) as usize],
                ALPHABET[(((a & 0x3) << 4) | (b >> 4)) as usize],
                ALPHABET[((b & 0xf) << 2) as usize],
            ])?;
        }
        _ => {}
    }

    Ok(())
}
