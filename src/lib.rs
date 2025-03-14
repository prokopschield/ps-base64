#![allow(clippy::module_name_repetitions)]

pub mod decoder;
pub mod encoder;

pub use decoder::*;
pub use encoder::*;

pub mod base64 {
    pub use crate::decoder::decode;
    pub use crate::encoder::encode;
    pub use crate::encoder::sized_encode;
}
