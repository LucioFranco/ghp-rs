use git2;
use std::{result, error, fmt};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Git(git2::Error),
    Other(String),
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Error {
        Error::Git(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Git(ref err) => write!(f, "git error: {}", err),
            Error::Other(ref err) => write!(f, "error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Git(ref err) => err.description(),
            Error::Other(ref err) => err,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Git(ref err) => Some(err),
            Error::Other(_) => None,
        }
    }
}
