use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowHandlerError {
    RawWindowHandle,
    RawDisplayHandle,
}

impl Display for WindowHandlerError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::RawWindowHandle => {
                write!(f, "Failed to get window handle")
            }
            Self::RawDisplayHandle => {
                write!(f, "Failed to get display handle")
            }
        }
    }
}

impl Error for WindowHandlerError {}
