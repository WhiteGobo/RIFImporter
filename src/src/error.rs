use std::io::Error;
use std::str::Utf8Error;
use oxrdf::IriParseError;
use std::fmt;
pub enum GenTermError {
    Error(Error),
    Utf8Error(Utf8Error),
    IriParseError(IriParseError),
}

impl fmt::Display for GenTermError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GenTermError::Error(e) => e.fmt(f),
            GenTermError::Utf8Error(e) => e.fmt(f),
            GenTermError::IriParseError(e) => e.fmt(f),
        }
    }
}
impl GenTermError {
    pub fn other(value: &str) -> Self {
        GenTermError::Error(Error::other(value))
    }
}
impl From<Utf8Error> for GenTermError {
    fn from(value: Utf8Error) -> Self {
        GenTermError::Utf8Error(value)
    }
}
impl From<IriParseError> for GenTermError {
    fn from(value: IriParseError) -> Self {
        GenTermError::IriParseError(value)
    }
}
