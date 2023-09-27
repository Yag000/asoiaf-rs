use std::{
    fmt::{self, Display, Formatter},
    io,
};

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
    NoMorePages,
    CacheMiss,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::RequestError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::ParseError(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::RequestError(error) => write!(f, "RequestError: {}", error),
            Error::ParseError(error) => write!(f, "ParseError: {}", error),
            Error::NoMorePages => write!(f, "NoMorePages"),
            Error::CacheMiss => write!(f, "CacheMiss"),
        }
    }
}

impl std::error::Error for Error {}
