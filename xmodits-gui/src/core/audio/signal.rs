use super::core::PlayHandle;



pub struct Oscillator {
    pub sample_rate: f32,
    pub frame: u64,
    pub frequency: f32,
    pub duration: usize,
}

impl Oscillator {
    fn next_sample(&mut self) {
        self.frame = (self.frame + 1) % self.sample_rate as u64;
    }

    fn sine(&mut self) -> f32 {
        self.next_sample();
        let two_pi = 2.0 * std::f32::consts::PI;
        (self.frame as f32 * self.frequency * two_pi / self.sample_rate).sin()
    }
}

impl PlayHandle for Oscillator {
    fn next(&mut self) -> Option<[f32; 2]> {
        let s = self.sine();
        Some([s, s])
    }

    fn reset(&mut self) {}

    fn jump(&mut self, _tick: usize) {}
}

pub struct Siren {
    pub sample_rate: f32,
    pub high: f32,
    pub low: f32,
    pub frame: u64,
    pub rate: f32,
    pub switch: bool,
}

impl Siren {
    fn next_sample(&mut self) {
        self.frame = (self.frame + 1) % self.sample_rate as u64;
    }
    fn sine(&self, freq: f32) -> f32 {
        let two_pi = 2.0 * std::f32::consts::PI;
        (self.frame as f32 * freq * two_pi / self.sample_rate).sin()
    }

    fn next(&mut self) -> f32 {
        self.next_sample();
        if self.frame % (self.sample_rate / self.rate).floor() as u64 == 0 {
            self.switch = !self.switch;
        }

        match self.switch {
            true => self.sine(self.high),
            false => self.sine(self.low),
        }
    }
}

impl PlayHandle for Siren {
    fn next(&mut self) -> Option<[f32; 2]> {
        // if self.duration > 44100 {
        //     return None;
        // }
        let s = self.next();
        // self.duration += 1;
        // dbg!(s);
        Some([s, s])
    }

    fn reset(&mut self) {}

    fn jump(&mut self, _tick: usize) {}
}