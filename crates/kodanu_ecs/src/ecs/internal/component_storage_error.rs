use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentStorageError {
    TypeMismatch,
}

impl Display for ComponentStorageError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::TypeMismatch => {
                write!(f, "Component storage type mismatch")
            }
        }
    }
}
