use specs::prelude::*;
use specs_derive::*;

use crate::utils::time::current_timestamp;

#[derive(Clone, Component)]
pub struct SpawnTime {
    interval: i64,
    start: i64,
    current: i64,
    end: i64,
    pub progress: i64,
    pub should_spawn: bool,
}

impl SpawnTime {
    pub fn new(seconds: i64) -> Self {
        let current_time = current_timestamp();
        SpawnTime {
            interval: seconds,
            start: current_time,
            current: current_time,
            end: current_time + seconds,
            progress: 0,
            should_spawn: false,
        }
    }

    pub fn update(&mut self, current_time: i64) {
        match self.should_spawn {
            true => self.reset(current_time),
            false => {
                self.current = current_time;
                self.progress = (self.current - self.start) * 100 / (self.end - self.start);
                self.should_spawn = self.progress >= 100;
            }
        }
    }

    fn reset(&mut self, current_time: i64) {
        self.start = current_time;
        self.current = current_time;
        self.end = current_time + self.interval;
        self.progress = 0;
        self.should_spawn = false;
    }
}
