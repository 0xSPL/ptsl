//! Traits for working with protobuf types.

mod decode;
mod encode;

pub use self::decode::Decode;
pub use self::encode::Encode;