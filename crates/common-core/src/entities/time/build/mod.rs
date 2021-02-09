use specs::prelude::*;
use specs_derive::*;

use crate::utils::time::current_timestamp;

#[derive(Clone, Component)]
pub struct BuildTime {
    start: i64,
    current: i64,
    end: i64,
    pub progress: i64,
    pub is_completed: bool,
}

impl BuildTime {
    pub fn new(seconds: i64) -> Self {
        let current_time = current_timestamp();
        match current_time + seconds == current_time {
            true => BuildTime {
                start: current_time,
                current: 0,
                end: current_time + seconds,
                progress: 100,
                is_completed: true,
            },
            false => BuildTime {
                start: current_time,
                current: 0,
                end: current_time + seconds,
                progress: 0,
                is_completed: false,
            },
        }
    }

    pub fn update(&mut self, current_time: i64) {
        if self.is_completed == true {
            return;
        }

        self.current = current_time;
        self.progress = (self.current - self.start) * 100 / (self.end - self.start);
        self.is_completed = self.progress >= 100;
    }
}
