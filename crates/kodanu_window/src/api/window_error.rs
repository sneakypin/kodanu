use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowError {
    WindowCreation,
}

impl Display for WindowError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::WindowCreation => {
                write!(f, "Failed to create window")
            }
        }
    }
}

impl Error for WindowError {}
