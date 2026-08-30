//! Ultra-optimized, highly lenient base64url encoding and decoding.
//!
//! Encoding produces unpadded [base64url] output. Decoding is lenient: it skips
//! ASCII whitespace and `=`, accepts both the URL-safe and standard alphabets,
//! and never fails.
//!
//! [base64url]: https://datatracker.ietf.org/doc/html/rfc4648#section-5

#![allow(clippy::module_name_repetitions)]

pub mod decoder;
pub mod encoder;

#[cfg(test)]
mod tests;

pub use decoder::*;
pub use encoder::*;

pub mod base64 {
    pub use crate::decoder::decode;
    pub use crate::decoder::decode_into;
    pub use crate::decoder::sized_decode;
    pub use crate::encoder::display;
    pub use crate::encoder::encode;
    pub use crate::encoder::encode_into;
    pub use crate::encoder::sized_encode;
}
