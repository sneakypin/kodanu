use {
    kodanu_ecs::{ResMut, Resource},
    std::time::{Duration, Instant},
};

/// Provides timing information for the engine.
///
/// `Time` tracks the duration of the current frame, the total time elapsed
/// since the engine started, and the maximum delta time applied to a frame.
///
/// Time values returned by [`Self::delta`] and [`Self::elapsed`] are expressed
/// in seconds.
#[derive(Resource, Debug)]
pub struct Time {
    startup: Instant,
    last: Instant,
    delta: Duration,
    elapsed: Duration,
    max_delta: Duration,
}

impl Default for Time {
    fn default() -> Self {
        let now = Instant::now();

        Self {
            startup: now,
            last: now,
            delta: Duration::ZERO,
            elapsed: Duration::ZERO,
            max_delta: Duration::from_millis(100),
        }
    }
}

impl Time {
    /// Maximum duration that can be reported by [`Self::delta`].
    ///
    /// The default value is `100 ms`.
    pub const DEFAULT_MAX_DELTA: Duration = Duration::from_millis(100);
}

impl Time {
    pub(crate) fn time_system(mut time: ResMut<Time>) {
        let now = Instant::now();
        let delta = now.duration_since(time.last);

        time.delta = delta.min(time.max_delta);
        time.elapsed = now.duration_since(time.startup);

        time.last = now;
    }
}

impl Time {
    /// Returns the duration of the current frame in seconds.
    ///
    /// The returned value is clamped to [`Self::max_delta`].
    #[must_use]
    pub const fn delta(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    /// Returns the total time elapsed since the engine started, in seconds.
    ///
    /// Unlike [`Self::delta`], this value is not affected by
    /// [`Self::max_delta`].
    #[must_use]
    pub const fn elapsed(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }

    /// Returns the maximum duration that can be reported by [`Self::delta`].
    #[must_use]
    pub const fn max_delta(&self) -> Duration {
        self.max_delta
    }

    /// Sets the maximum duration that can be reported by [`Self::delta`].
    ///
    /// This is useful for preventing unusually large frame times from
    /// destabilizing simulation after events such as debugger pauses,
    /// window dragging, or temporary stalls.
    pub const fn set_max_delta(&mut self, max_delta: Duration) {
        self.max_delta = max_delta;
    }
}
