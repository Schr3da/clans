use specs::prelude::*;
use specs_derive::*;

use crate::utils::time::current_timestamp;

use crate::entities::traits::render::Updateable;

#[derive(Clone, Component)]
pub struct IntervalTimer {
    interval: i64,
    current: i64,
    end: i64,
    needs_update: bool,
}

impl Updateable for IntervalTimer {
    fn needs_update(&self) -> bool {
        self.needs_update
    }

    fn force_update(&mut self, should_update: bool) {
        self.needs_update = should_update;
    }
}

impl IntervalTimer {
    pub fn new(seconds: i64) -> Self {
        let current_time = current_timestamp();
        IntervalTimer {
            interval: seconds,
            current: 0,
            end: current_time + seconds,
            needs_update: false,
        }
    }

    pub fn update(&mut self, current: i64) {
        match self.needs_update() {
            true => self.reset(current),
            false => {
                self.current = current;
                self.force_update(self.current >= self.end);
            }
        }
    }

    fn reset(&mut self, current: i64) {
        self.current = self.current;
        self.end = current + self.interval;

        self.force_update(false);
    }
}
