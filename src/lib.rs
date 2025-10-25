#![allow(clippy::module_name_repetitions)]

pub mod decoder;
pub mod encoder;

#[cfg(test)]
mod tests;

pub use decoder::*;
pub use encoder::*;

pub mod base64 {
    pub use crate::decoder::decode;
    pub use crate::decoder::sized_decode;
    pub use crate::encoder::encode;
    pub use crate::encoder::encode_into;
    pub use crate::encoder::sized_encode;
}
