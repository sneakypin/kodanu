use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorldError {
    ResourceNotRegistered,
    EventNotRegistered,
}

impl Display for WorldError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::ResourceNotRegistered => {
                write!(f, "Resource not registered")
            }
            Self::EventNotRegistered => {
                write!(f, "Event not registered")
            }
        }
    }
}
