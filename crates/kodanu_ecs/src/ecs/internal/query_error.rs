use std::fmt::{Display, Formatter, Result};

pub enum QueryError {
    StorageMissing,
}

impl Display for QueryError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            QueryError::StorageMissing => {
                write!(f, "storage missing")
            }
        }
    }
}
