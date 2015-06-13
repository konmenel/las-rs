//! Read ASPRS las files.

extern crate byteorder;
#[macro_use]
extern crate enum_primitive;
extern crate num;
extern crate rustc_serialize;

use std::result;

pub mod header;
pub mod point;
pub mod reader;

pub use header::Header;
pub use point::Point;
pub use reader::Reader;

#[derive(Debug)]
enum Error {
    ByteorderError(byteorder::Error),
    CharacterAfterNullByte,
    IoError(std::io::Error),
    ReadError(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        Error::ByteorderError(err)
    }
}

pub type Result<T> = result::Result<T, Error>;
