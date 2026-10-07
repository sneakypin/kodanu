use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstanceError {
    SurfaceCreate,
    AdapterRequest,
    DeviceCreate,
}

impl Display for InstanceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::SurfaceCreate => {
                write!(f, "Failed to create surface")
            }
            Self::AdapterRequest => {
                write!(f, "Failed to request adapter")
            }
            Self::DeviceCreate => {
                write!(f, "Failed to create device")
            }
        }
    }
}
