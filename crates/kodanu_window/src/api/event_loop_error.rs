use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventLoopError {
    EventLoopCreation,
    EventLoopRun,
}

impl Display for EventLoopError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::EventLoopCreation => {
                write!(f, "Failed to create window event loop")
            }
            Self::EventLoopRun => {
                write!(f, "Failed to run event loop")
            }
        }
    }
}
