use core::fmt::Display;

use serde::{de, ser};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("expected a single character, found a string of length {0}")]
    CharTooLong(usize),
    #[error("expected a single character, found nothing")]
    CharMissing,
    #[error("value too large")]
    ValueTooLarge,
    #[error("invalid vlq: {0}")]
    InvalidVlq(u64),
    #[error("deserialize_any is not implemented")]
    DeserializeAnyNotImplemented,
    #[error("{0}")]
    Custom(String),
}
impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}
impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}
