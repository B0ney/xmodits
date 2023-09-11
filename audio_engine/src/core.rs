
pub trait PlayHandle: Send + Sync {
    fn next(&mut self) -> Option<[f32; 2]>;
    fn reset(&mut self);
    fn jump(&mut self, tick: usize);
    fn write(&mut self, frames: &mut [[f32; 2]]) -> Option<usize> {
        let mut written: usize = 0;

        for frame in frames.iter_mut() {
            match self.next() {
                Some(f) => {
                    frame[0] += f[0]; // todo: mix or overwrite?
                    frame[1] += f[1];
                    written += 1;
                }
                None => {
                    written += 1;
                }
            }
        }
        Some(written)
    }
}

impl PlayHandle for Box<dyn PlayHandle> {
    fn next(&mut self) -> Option<[f32; 2]> {
        (**self).next()
    }
    fn reset(&mut self) {
        (**self).reset()
    }
    fn jump(&mut self, tick: usize) {
        (**self).jump(tick)
    }
}
