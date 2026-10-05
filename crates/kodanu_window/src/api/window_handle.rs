use crate::SurfaceTarget;

pub trait WindowHandle {
    fn target(&self) -> SurfaceTarget;
}
