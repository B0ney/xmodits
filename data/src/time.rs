use std::time::{Instant, Duration};

pub struct Time {
    start: Instant,
    duration: Duration,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            duration: Default::default(),
        }
    }
}

impl Time {
    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    pub fn stop(&mut self) {
        self.duration = self.start.elapsed();
    }

    pub fn elapsed(&self) -> f32 {
        self.duration.as_secs_f32()
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.duration.as_secs_f32() {
            s if s < 60.0 => format!("{} second(s)", s),
            s if s < 60.0 * 60.0 => format!("{} minute(s)", s / 60.0),
            s => format!("{} hour(s)", s / (60.0 * 60.0)),
        };
        write!(f, "Took {}", s)
    }
}