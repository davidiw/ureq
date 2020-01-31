use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Io(io::Error),
    Http11Parser(httparse::Error),
    Http(http::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Message(v) => write!(f, "{}", v),
            Error::Io(v) => fmt::Display::fmt(v, f),
            Error::Http11Parser(v) => write!(f, "http11 parser: {}", v),
            Error::Http(v) => write!(f, "http api: {}", v),
        }
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Message(s)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        Error::Message(s.to_owned())
    }
}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<httparse::Error> for Error {
    fn from(e: httparse::Error) -> Self {
        Error::Http11Parser(e)
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        Error::Http(e)
    }
}