use std::time::{Duration, Instant};

use kodanu_ecs::{ResMut, Resource};

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
    pub(crate) fn time_system(mut time: ResMut<Time>) {
        let now = Instant::now();
        let delta = now.duration_since(time.last);

        time.delta = delta.min(time.max_delta);
        time.elapsed = now.duration_since(time.startup);

        time.last = now;
    }
}

impl Time {
    pub fn delta(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    pub fn elapsed(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }
}
