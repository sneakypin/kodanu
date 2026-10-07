use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PipelineLayoutStorageError {
    PipelineNotFound,
}

impl Display for PipelineLayoutStorageError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::PipelineNotFound => {
                write!(f, "Pipeline not found")
            }
        }
    }
}
