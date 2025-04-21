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
