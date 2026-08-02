use super::*;

// Helper function for comparing byte arrays
fn assert_bytes_eq(expected: &[u8], actual: &[u8]) {
    assert_eq!(
        expected, actual,
        "Byte arrays differ: expected {expected:?}, actual {actual:?}"
    );
}

#[test]
fn test_decode_one() {
    assert_eq!(decode(b"AA"), [0,]);
    assert_eq!(decode(b"AQ"), [1]);
}

#[test]
fn test_decode_two() {
    assert_eq!(decode(b"AAaA"), [0x00, 0x06, 0x80]);
    assert_eq!(decode(b"AQAw"), [0x01, 0x00, 0x30]);
}

#[test]
fn test_decode_three() {
    assert_eq!(decode(b"AAABAg"), [0, 0, 1, 2]);
    assert_eq!(decode(b"AAECAw"), [0, 1, 2, 3]);
}

#[test]
fn test_decode_four() {
    assert_eq!(decode(b"AAAAAA=="), [0, 0, 0, 0]);
    assert_eq!(decode(b"AAECAwQw"), [0, 1, 2, 3, 4, 0x30]);
}

#[test]
fn test_decode_invalid_char() {
    assert_eq!(decode(b"A@A@A@A@"), [0, 0, 0, 0, 0, 0]); // '@' is invalid
    assert_eq!(decode(b"A A A A "), [0, 0, 0]); // ' ' is invalid
}

#[test]
fn test_align_up() {
    assert_eq!(align_up(0), 0);
    assert_eq!(align_up(1), 4);
    assert_eq!(align_up(2), 4);
    assert_eq!(align_up(3), 4);
    assert_eq!(align_up(4), 4);
    assert_eq!(align_up(5), 8);
    assert_eq!(align_up(6), 8);
    assert_eq!(align_up(7), 8);
    assert_eq!(align_up(8), 8);
    assert_eq!(align_up(9), 12);
    assert_eq!(align_up(10), 12);
    assert_eq!(align_up(11), 12);
    assert_eq!(align_up(12), 12);
}

#[test]
fn test_three_fourths() {
    assert_eq!(three_fourths(0), 0);
    assert_eq!(three_fourths(1), 1);
    assert_eq!(three_fourths(2), 2);
    assert_eq!(three_fourths(3), 3);
    assert_eq!(three_fourths(4), 3);
    assert_eq!(three_fourths(5), 4);
    assert_eq!(three_fourths(6), 5);
    assert_eq!(three_fourths(7), 6);
    assert_eq!(three_fourths(8), 6);
    assert_eq!(three_fourths(9), 7);
    assert_eq!(three_fourths(10), 8);
    assert_eq!(three_fourths(11), 9);
    assert_eq!(three_fourths(12), 9);

    // The naive `(size * 3).div_ceil(4)` overflows here. `usize::MAX` is
    // `4q + 3` with `q = usize::MAX / 4`, so the result is `3q + 3`.
    assert_eq!(three_fourths(usize::MAX), usize::MAX / 4 * 3 + 3);
}

#[test]
fn test_decode_empty() {
    assert_bytes_eq(b"", &decode(b""));
}

#[test]
fn test_decode_basic() {
    assert_bytes_eq(b"", &decode(b""));
    assert_bytes_eq(b"f", &decode(b"Zg=="));
    assert_bytes_eq(b"fo", &decode(b"Zm8="));
    assert_bytes_eq(b"foo", &decode(b"Zm9v"));
    assert_bytes_eq(b"foob", &decode(b"Zm9vYg=="));
    assert_bytes_eq(b"fooba", &decode(b"Zm9vYmE="));
    assert_bytes_eq(b"foobar", &decode(b"Zm9vYmFy"));
}

#[test]
fn test_decode_padding() {
    assert_bytes_eq(b"f", &decode(b"Zg=="));
    assert_bytes_eq(b"fo", &decode(b"Zm8="));
}

#[test]
fn test_decode_multi_block() {
    assert_bytes_eq(b"abcdef", &decode(b"YWJjZGVm"));
    assert_bytes_eq(b"abcdefgh", &decode(b"YWJjZGVmZ2g="));
    assert_bytes_eq(b"abcdefghi", &decode(b"YWJjZGVmZ2hp"));
}

#[test]
fn test_decode_url_safe() {
    // Example from RFC 4648, Section 5
    assert_bytes_eq(b"", &decode(b""));
    assert_bytes_eq(b"f", &decode(b"Zg=="));
    assert_bytes_eq(b"fo", &decode(b"Zm8="));
    assert_bytes_eq(b"foo", &decode(b"Zm9v"));
    assert_bytes_eq(b"foob", &decode(b"Zm9vYg=="));
    assert_bytes_eq(b"fooba", &decode(b"Zm9vYmE="));
    assert_bytes_eq(b"foobar", &decode(b"Zm9vYmFy"));
}

#[test]
fn test_decode_invalid_input_length() {
    // The function should still work, but the output will be truncated.
    assert_bytes_eq(b"A", &decode(b"QQ")); // len = 2
    assert_bytes_eq(b"AB", &decode(b"QUI")); // len = 3
    assert_bytes_eq(b"ABC", &decode(b"QUJD")); // len = 4
    assert_bytes_eq(b"ABCD", &decode(b"QUJDRA")); // len = 5
    assert_bytes_eq(b"ABCDE", &decode(b"QUJDREU")); // len = 6
    assert_bytes_eq(b"ABCDEF", &decode(b"QUJDREVG")); // len = 7
    assert_bytes_eq(b"ABCDEFG", &decode(b"QUJDREVGRw")); // len = 8
}

#[test]
fn test_decode_non_canonical_tail() {
    // Trailing bits that a canonical encoder would leave zero are discarded,
    // so the output length matches that of the canonical encoding.
    assert_bytes_eq(&[102], &decode(b"Zg"));
    assert_bytes_eq(&[102], &decode(b"Zh"));

    assert_bytes_eq(&[102, 108], &decode(b"Zmw"));
    assert_bytes_eq(&[102, 107], &decode(b"Zmv"));
}

#[test]
fn test_decode_stray_trailing_char() {
    // Decoding is best-effort, so a lone trailing character is widened into a
    // byte from its six bits rather than dropped. No encoder emits such a
    // tail, since an encoding is never one character longer than a multiple
    // of four.
    assert_bytes_eq(&[100], &decode(b"Z"));
    assert_bytes_eq(b"foo\0", &decode(b"Zm9vA"));
    assert_bytes_eq(b"food", &decode(b"Zm9vZ"));
}

#[test]
fn test_decode_length_is_data_independent() {
    // Every trailing group decodes to the same number of bytes, regardless of
    // the padding bits it carries.
    for &byte in ALPHABET {
        assert_eq!(decode(&[byte]).len(), 1);
        assert_eq!(decode(&[b'Z', byte]).len(), 1);
        assert_eq!(decode(&[b'Z', b'm', byte]).len(), 2);
    }
}

#[test]
fn test_decode_length_depends_only_on_input_length() {
    let encoded = encode(&pseudo_random_bytes(3_000));

    for len in 0..=encoded.len() {
        // A trailing group of one or two characters yields one byte, and a
        // trailing group of three yields two.
        let expected = len / 4 * 3 + [0, 1, 1, 2][len % 4];

        assert_eq!(decode(&encoded.as_bytes()[..len]).len(), expected, "{len}");
    }
}

#[test]
fn test_decode_large_input() {
    // Create a large input string (1000 characters)
    let input_string =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".repeat(15); // Length: 945
    let encoded_string = base64::encode(input_string.as_bytes());
    let decoded_bytes = decode(encoded_string.as_bytes());
    assert_bytes_eq(input_string.as_bytes(), &decoded_bytes);
}

#[test]
fn test_sized_decode_empty() {
    assert_bytes_eq(b"", &sized_decode::<0>(b""));
    assert_bytes_eq(b"\0", &sized_decode::<1>(b""));
    assert_bytes_eq(b"\0\0", &sized_decode::<2>(b""));
    assert_bytes_eq(b"\0\0\0", &sized_decode::<3>(b""));
}

#[test]
fn test_sized_decode_basic() {
    assert_bytes_eq(b"", &sized_decode::<0>(b"Zg=="));
    assert_bytes_eq(b"f", &sized_decode::<1>(b"Zg=="));
    assert_bytes_eq(b"f", &sized_decode::<1>(b"Zm8="));
    assert_bytes_eq(b"fo", &sized_decode::<2>(b"Zm8="));
    assert_bytes_eq(b"fo", &sized_decode::<2>(b"Zm9v"));
    assert_bytes_eq(b"foo", &sized_decode::<3>(b"Zm9v"));
    assert_bytes_eq(b"foo", &sized_decode::<3>(b"Zm9vYg=="));
    assert_bytes_eq(b"foob", &sized_decode::<4>(b"Zm9vYg=="));
    assert_bytes_eq(b"foob", &sized_decode::<4>(b"Zm9vYmE="));
    assert_bytes_eq(b"fooba", &sized_decode::<5>(b"Zm9vYmE="));
    assert_bytes_eq(b"fooba", &sized_decode::<5>(b"Zm9vYmFy"));
    assert_bytes_eq(b"foobar", &sized_decode::<6>(b"Zm9vYmFy"));
}

#[test]
fn test_sized_decode_padding() {
    assert_bytes_eq(b"", &sized_decode::<0>(b"Zg=="));
    assert_bytes_eq(b"f", &sized_decode::<1>(b"Zg=="));
    assert_bytes_eq(b"f", &sized_decode::<1>(b"Zm8="));
    assert_bytes_eq(b"fo", &sized_decode::<2>(b"Zm8="));
}

#[test]
fn test_sized_decode_multi_block() {
    assert_bytes_eq(b"", &sized_decode::<0>(b"YWJjZGVm"));
    assert_bytes_eq(b"a", &sized_decode::<1>(b"YWJjZGVm"));
    assert_bytes_eq(b"ab", &sized_decode::<2>(b"YWJjZGVm"));
    assert_bytes_eq(b"abc", &sized_decode::<3>(b"YWJjZGVm"));
    assert_bytes_eq(b"abcd", &sized_decode::<4>(b"YWJjZGVm"));
    assert_bytes_eq(b"abcde", &sized_decode::<5>(b"YWJjZGVm"));
    assert_bytes_eq(b"abcdef", &sized_decode::<6>(b"YWJjZGVm"));
    assert_bytes_eq(b"abcdef", &sized_decode::<6>(b"YWJjZGVmZ2g="));
    assert_bytes_eq(b"abcdefg", &sized_decode::<7>(b"YWJjZGVmZ2h="));
    assert_bytes_eq(b"abcdefgh", &sized_decode::<8>(b"YWJjZGVmZ2h="));
    assert_bytes_eq(b"abcdefgh", &sized_decode::<8>(b"YWJjZGVmZ2hp"));
    assert_bytes_eq(b"abcdefghi", &sized_decode::<9>(b"YWJjZGVmZ2hp"));
}

fn as_vec<const S: usize>(arr: [u8; S]) -> Vec<u8> {
    arr.to_vec()
}

#[test]
fn test_sized_decode_empty_input() {
    let out: [u8; 0] = sized_decode(b"");
    assert_eq!(out, []);
}

#[test]
fn test_sized_decode_one_byte() {
    let out: [u8; 1] = sized_decode(b"Zg==");
    assert_eq!(as_vec(out), b"f");
}

#[test]
fn test_sized_decode_two_bytes() {
    let out: [u8; 2] = sized_decode(b"Zm8=");
    assert_eq!(as_vec(out), b"fo");
}

#[test]
fn test_sized_decode_three_bytes() {
    let out: [u8; 3] = sized_decode(b"Zm9v");
    assert_eq!(as_vec(out), b"foo");
}

#[test]
fn test_sized_decode_four_bytes() {
    let out: [u8; 4] = sized_decode(b"Zm9vYg==");
    assert_eq!(as_vec(out), b"foob");
}

#[test]
fn test_sized_decode_six_bytes() {
    let out: [u8; 6] = sized_decode(b"Zm9vYmFy");
    assert_eq!(as_vec(out), b"foobar");
}

#[test]
fn test_sized_decode_with_whitespace() {
    let input = b" Z m 9 v \n Y m F y  ";
    let out: [u8; 6] = sized_decode(input);
    assert_eq!(as_vec(out), b"foobar");
}

#[test]
fn test_sized_decode_no_padding() {
    let out: [u8; 8] = sized_decode(b"YWJjZGVmZ2g=");
    assert_eq!(as_vec(out), b"abcdefgh");
}

#[test]
fn test_sized_decode_truncated_output() {
    let out: [u8; 3] = sized_decode(b"Zm9vYmFy");
    assert_eq!(as_vec(out), b"foo");
}

#[test]
fn test_sized_decode_larger_buffer() {
    let out: [u8; 8] = sized_decode(b"Zm9vYmFy");
    let mut expected = b"foobar".to_vec();
    expected.extend_from_slice(&[0, 0]);
    assert_eq!(as_vec(out), expected);
}

#[test]
fn test_sized_decode_all_byte_values() {
    let raw: Vec<u8> = (0u8..=255).collect();
    let b64 = base64::encode(&raw);
    let out: [u8; 256] = sized_decode(b64.as_bytes());
    assert_eq!(as_vec(out), raw);
}

#[test]
fn test_encode_basic() {
    // base64url, unpadded (RFC 4648 §5 vectors without '=' padding)
    assert_eq!(encode(b""), "");
    assert_eq!(encode(b"f"), "Zg");
    assert_eq!(encode(b"fo"), "Zm8");
    assert_eq!(encode(b"foo"), "Zm9v");
    assert_eq!(encode(b"foob"), "Zm9vYg");
    assert_eq!(encode(b"fooba"), "Zm9vYmE");
    assert_eq!(encode(b"foobar"), "Zm9vYmFy");
}

#[test]
fn test_encode_url_safe_alphabet() {
    // Index 62 is '-', and index 63 is '_'.
    assert_eq!(encode(&[0xf8, 0x00, 0x00]), "-AAA");
    assert_eq!(encode(&[0xff, 0xff, 0xff]), "____");
}

#[test]
fn test_encode_roundtrip() {
    for len in 0u8..=64 {
        let input: Vec<u8> = (0..len).collect();

        assert_bytes_eq(&input, &decode(encode(&input).as_bytes()));
    }
}

#[test]
fn test_encode_into_matches_encode() -> core::fmt::Result {
    for len in 0u8..=64 {
        let input: Vec<u8> = (0..len).collect();

        let mut sink = String::new();

        encode_into(&input, &mut sink)?;

        assert_eq!(sink, encode(&input));
    }

    Ok(())
}

#[test]
fn test_sized_encode_exact() {
    assert_bytes_eq(b"Zg", &sized_encode::<2>(b"f"));
    assert_bytes_eq(b"Zm8", &sized_encode::<3>(b"fo"));
    assert_bytes_eq(b"Zm9v", &sized_encode::<4>(b"foo"));
    assert_bytes_eq(b"Zm9vYmFy", &sized_encode::<8>(b"foobar"));
}

#[test]
fn test_sized_encode_pads_with_equals() {
    // A buffer larger than the encoding is right-padded with '='.
    assert_bytes_eq(b"Zg==", &sized_encode::<4>(b"f"));
    assert_bytes_eq(b"Zm9v====", &sized_encode::<8>(b"foo"));
}

#[test]
fn test_sized_encode_truncates() {
    // A buffer smaller than the encoding silently truncates.
    assert_bytes_eq(b"", &sized_encode::<0>(b"foobar"));
    assert_bytes_eq(b"Zm", &sized_encode::<2>(b"foobar"));
}

// Deterministic pseudo-random bytes (xorshift64). A plain counter only ever
// exercises a narrow, highly regular set of byte values.
fn pseudo_random_bytes(len: usize) -> Vec<u8> {
    let mut state: u64 = 0x2545_f491_4f6c_dd1d;

    (0..len)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;

            state.to_le_bytes()[0]
        })
        .collect()
}

// Length of the unpadded base64url encoding of `len` bytes.
fn encoded_len(len: usize) -> usize {
    len / 3 * 4
        + match len % 3 {
            0 => 0,
            1 => 2,
            _ => 3,
        }
}

// Every input length from 0 to 256, which covers all three residues modulo 3
// and every position within a four-character group, plus a few larger sizes.
fn end_to_end_lengths() -> impl Iterator<Item = usize> {
    (0..=256).chain([1_000, 4_095, 4_096, 4_097])
}

#[test]
fn test_roundtrip_every_length() {
    for len in end_to_end_lengths() {
        let input = pseudo_random_bytes(len);
        let encoded = encode(&input);

        assert_eq!(encoded.len(), encoded_len(len), "encoding of {len} bytes");
        assert_eq!(
            decode(encoded.as_bytes()),
            input,
            "roundtrip of {len} bytes"
        );
    }
}

#[test]
fn test_roundtrip_every_length_via_encode_into() -> core::fmt::Result {
    for len in end_to_end_lengths() {
        let input = pseudo_random_bytes(len);

        let mut encoded = String::new();

        encode_into(&input, &mut encoded)?;

        assert_eq!(encoded, encode(&input), "encoding of {len} bytes");
        assert_eq!(
            decode(encoded.as_bytes()),
            input,
            "roundtrip of {len} bytes"
        );
    }

    Ok(())
}

#[test]
fn test_roundtrip_every_length_with_padding() {
    for len in end_to_end_lengths() {
        let input = pseudo_random_bytes(len);

        let mut encoded = encode(&input).into_bytes();

        while !encoded.len().is_multiple_of(4) {
            encoded.push(b'=');
        }

        assert_eq!(decode(&encoded), input, "roundtrip of {len} bytes");
    }
}

#[test]
fn test_roundtrip_every_length_with_whitespace() {
    for len in end_to_end_lengths() {
        let input = pseudo_random_bytes(len);

        let mut encoded = vec![b'\n'];

        for (index, byte) in encode(&input).bytes().enumerate() {
            encoded.push(byte);

            if index % 5 == 4 {
                encoded.push(b' ');
            }
        }

        encoded.push(b'\t');

        assert_eq!(decode(&encoded), input, "roundtrip of {len} bytes");
    }
}

// The bytes decoding skips: ASCII whitespace and '='.
const SKIPPED: [u8; 6] = [b' ', b'\t', b'\n', b'\x0C', b'\r', b'='];

// Straightforward reference decoder: drop the skipped bytes, then decode what
// is left four characters at a time, following the documented tail rule.
fn reference_decode(input: &[u8]) -> Vec<u8> {
    let accepted: Vec<u8> = input
        .iter()
        .copied()
        .filter(|&byte| !byte.is_ascii_whitespace() && byte != b'=')
        .collect();

    let mut output = Vec::new();

    for chunk in accepted.chunks(4) {
        let value = |index: usize| decode_base64_char(chunk[index]);

        match chunk.len() {
            1 => output.push(value(0) << 2),
            2 => output.push((value(0) << 2) | (value(1) >> 4)),
            3 => output.extend_from_slice(&[
                (value(0) << 2) | (value(1) >> 4),
                ((value(1) & 0xf) << 4) | (value(2) >> 2),
            ]),
            _ => output.extend_from_slice(&[
                (value(0) << 2) | (value(1) >> 4),
                ((value(1) & 0xf) << 4) | (value(2) >> 2),
                ((value(2) & 0x3) << 6) | value(3),
            ]),
        }
    }

    output
}

#[test]
fn test_decode_skip_at_every_position() {
    // A single skipped byte at every position of every encoding walks the
    // fast-path breakpoint through all four group offsets after every
    // possible number of complete groups.
    for len in 0..=120 {
        let encoded = encode(&pseudo_random_bytes(len)).into_bytes();

        assert_eq!(
            decode(&encoded),
            reference_decode(&encoded),
            "clean, len {len}"
        );

        for position in 0..=encoded.len() {
            for skip in SKIPPED {
                let mut probe = encoded.clone();

                probe.insert(position, skip);

                assert_eq!(
                    decode(&probe),
                    reference_decode(&probe),
                    "len {len}, position {position}, skip {:?}",
                    char::from(skip)
                );
            }
        }
    }
}

#[test]
fn test_decode_adjacent_skips() {
    // Two adjacent skipped bytes make the fallback resume on a boundary that
    // is offset from the group grid by two rather than one.
    for len in 0..=60 {
        let encoded = encode(&pseudo_random_bytes(len)).into_bytes();

        for position in 0..=encoded.len() {
            let mut probe = encoded.clone();

            probe.insert(position, b'\n');
            probe.insert(position, b'\r');

            assert_eq!(
                decode(&probe),
                reference_decode(&probe),
                "len {len}, position {position}"
            );
        }
    }
}

#[test]
fn test_decode_line_wrapped_input() {
    // MIME wraps at 76 characters and PEM at 64; both land on group offset 0,
    // well after the fast path has been running.
    for width in [64, 76] {
        for len in 0..=400 {
            let input = pseudo_random_bytes(len);

            let mut wrapped = Vec::new();

            for (index, byte) in encode(&input).bytes().enumerate() {
                if index > 0 && index % width == 0 {
                    wrapped.extend_from_slice(b"\r\n");
                }

                wrapped.push(byte);
            }

            assert_eq!(decode(&wrapped), input, "wrap {width}, len {len}");
        }
    }
}

#[test]
fn test_decode_only_skipped_bytes() {
    for len in 0..=32 {
        for skip in SKIPPED {
            let probe = vec![skip; len];

            assert!(
                decode(&probe).is_empty(),
                "len {len}, skip {:?}",
                char::from(skip)
            );
        }
    }
}
