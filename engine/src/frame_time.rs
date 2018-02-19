
use std::time;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct FrameTime {
    clock: time::Instant,
    previous: u64,
    elapsed: u64,
    last_ten: VecDeque<u64>,
}

impl FrameTime {
    pub fn new() -> FrameTime {
        FrameTime {
            clock: time::Instant::now(),
            previous: 0,
            elapsed: u64::max_value(),
            last_ten: VecDeque::new(),
        }
    }

    pub fn get_frame_rate(&self) -> u64 {
        if self.elapsed == 0 {
            return 0;
        }
        1_000 / self.elapsed
    }

    /// Average frame rate over 10 frames
    pub fn get_avg_frame_rate(&self) -> f32 {
        self.last_ten.iter().fold(0, |sum, &v| sum + v) as f32 / self.last_ten.len() as f32
    }

    pub fn elapsed(&self) -> u64 {
        self.get_duration()
    }

    pub fn get_last_frame_duration(&self) -> u64 {
        self.elapsed
    }

    fn get_duration(&self) -> u64 {
        let t = self.clock.elapsed();
        (t.as_secs() * 1_000) + (t.subsec_nanos() / 1_000_000) as u64
    }

    pub fn tick(&mut self) {
        let current = self.get_duration();
        self.elapsed = current - self.previous;
        self.previous = current;
    }
}
