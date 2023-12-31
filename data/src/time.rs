use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
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

    pub fn init() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let round_100th = |x: f32| (x * 100.0).round() / 100.0;
        let round_1000th = |x: f32| (x * 1000.0).round() / 1000.0;

        let plural = |x: f32| if x == 1.0 { "" } else { "s" };

        let s = match self.duration.as_secs_f32() {
            s if s < 60.0 => format!("{} second{}", round_1000th(s), plural(s)),
            s => {
                let minute = round_100th(s / 60.0);
                format!("{} minute{}", minute, plural(minute))
            }
        };
        write!(f, "Took {}", s)
    }
}
