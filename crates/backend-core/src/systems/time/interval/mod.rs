use specs::prelude::*;

use common_core::prelude::*;

pub struct IntervalSystem {}

impl Default for IntervalSystem {
    fn default() -> Self {
        IntervalSystem {}
    }
}

impl<'a> System<'a> for IntervalSystem {
    type SystemData = (ReadExpect<'a, Config>, WriteStorage<'a, IntervalTimer>);

    fn run(&mut self, data: Self::SystemData) {
        let (config, mut intervals) = data;

        for interval in (&mut intervals).join() {
            interval.update(config.current_time);
        }
    }
}
