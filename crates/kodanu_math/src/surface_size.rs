use kodanu_ecs::Event;

/// An event emitted when the rendering surface changes size.
///
/// The dimensions are specified in physical pixels.
///
/// Both dimensions are guaranteed to be at least `1`.
///
/// # Examples
///
/// ```
/// use kodanu_math::SurfaceSize;
///
/// let event = SurfaceSize::clamped(1280, 720);
///
/// assert_eq!(event.width(), 1280);
/// assert_eq!(event.height(), 720);
/// ```
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceSize {
    width: u32,
    height: u32,
}

impl SurfaceSize {
    /// Creates a surface size from the given dimensions.
    ///
    /// Each dimension is clamped to a minimum of `1` pixel.
    ///
    /// # Examples
    ///
    /// ```
    /// use kodanu_math::SurfaceSize;
    ///
    /// let size = SurfaceSize::clamped(1920, 1080);
    ///
    /// assert_eq!(size.width(), 1920);
    /// assert_eq!(size.height(), 1080);
    /// ```
    ///
    /// Zero dimensions are clamped to `1`:
    ///
    /// ```
    /// use kodanu_math::SurfaceSize;
    ///
    /// let size = SurfaceSize::clamped(0, 0);
    ///
    /// assert_eq!(size.width(), 1);
    /// assert_eq!(size.height(), 1);
    /// ```
    #[must_use]
    pub fn clamped(width: u32, height: u32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
        }
    }
}

impl SurfaceSize {
    /// Returns the surface width in physical pixels.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Returns the surface height in physical pixels.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Returns the width-to-height aspect ratio.
    ///
    /// The result is always greater than zero.
    #[must_use]
    pub fn aspect_ratio(self) -> f32 {
        self.width as f32 / self.height as f32
    }
}
