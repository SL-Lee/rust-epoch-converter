use std::{error, fmt};

pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error { kind }
    }

    #[allow(dead_code)]
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl error::Error for Error {}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidDate => {
                write!(f, "<date> must be in the YYYY-MM-DDTHH:MM:SSZ format")
            }
            ErrorKind::InvalidEpoch => write!(
                f,
                "<epoch> must be an integer representing the number of seconds \
                elapsed since 1970-01-01T00:00:00Z",
            ),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidDate => {
                write!(f, "<date> must be in the YYYY-MM-DDTHH:MM:SSZ format")
            }
            ErrorKind::InvalidEpoch => write!(
                f,
                "<epoch> must be an integer representing the number of seconds \
                elapsed since 1970-01-01T00:00:00Z",
            ),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidDate,
    InvalidEpoch,
}
