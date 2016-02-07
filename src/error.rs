use std::io;
use std::result;
use toml;

pub type Result<A> = result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    TOMLDecode(toml::DecodeError),
    TOMLParse(Vec<toml::ParserError>),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}

impl From<toml::DecodeError> for Error {
    fn from(e: toml::DecodeError) -> Error {
        Error::TOMLDecode(e)
    }
}
