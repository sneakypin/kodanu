use std::fmt::{Display, Formatter, Result};

pub enum EntityError {
    GenerationOverflow,
}

impl Display for EntityError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::GenerationOverflow => write!(f, "entity generation overflow"),
        }
    }
}
