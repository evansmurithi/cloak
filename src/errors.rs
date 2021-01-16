use data_encoding::DecodeError;
use std::io;
use std::result;
use thiserror::Error as ThisError;
use toml::{de, ser};

// `Result` returned by this crate
pub type Result<T> = result::Result<T, Error>;

// `cloak` error
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Decoding failed for key '{}': {}", key, cause)]
    KeyDecode {
        key: String,
        cause: Box<DecodeError>,
    },

    #[error("Cloak directory not found")]
    CloakDirNotFound,

    #[error("I/O error: {}", _0)]
    Io(#[from] io::Error),

    #[error("Could not write to accounts file: {}", _0)]
    TomlSerialize(#[from] ser::Error),

    #[error("Could not parse the accounts file: {}", _0)]
    TomlDeserialize(#[from] de::Error),
}
