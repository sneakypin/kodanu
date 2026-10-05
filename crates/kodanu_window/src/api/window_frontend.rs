use kodanu_math::SurfaceSize;

pub trait WindowFrontend {
    fn redraw(&self);

    fn size(&self) -> SurfaceSize;
}
