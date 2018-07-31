use data_encoding::DecodeError;
use std::io;
use std::result;
use toml::{de, ser};

// `Result` returned by this crate
pub type Result<T> = result::Result<T, Error>;

// `cloak` error
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Decoding failed for key '{}': {}", key, cause)]
    KeyDecode {
        key: String,
        cause: Box<DecodeError>,
    },

    #[fail(display = "HOME_DIR could not be found")]
    HomeDirNotFound,

    #[fail(display = "I/O error: {}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "Could not write to accounts file: {}", _0)]
    TomlSerialize(#[cause] ser::Error),

    #[fail(display = "Could not parse the accounts file: {}", _0)]
    TomlDeserialize(#[cause] de::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<de::Error> for Error {
    fn from(err: de::Error) -> Self {
        Error::TomlDeserialize(err)
    }
}

impl From<ser::Error> for Error {
    fn from(err: ser::Error) -> Self {
        Error::TomlSerialize(err)
    }
}
