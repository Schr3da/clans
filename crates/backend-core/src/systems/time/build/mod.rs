use specs::prelude::*;

use common_core::prelude::*;

pub struct BuildTimeSystem {}

impl Default for BuildTimeSystem {
    fn default() -> Self {
        BuildTimeSystem {}
    }
}

impl<'a> System<'a> for BuildTimeSystem {
    type SystemData = (ReadExpect<'a, Config>, WriteStorage<'a, BuildTime>);

    fn run(&mut self, data: Self::SystemData) {
        let (config, mut build_times) = data;

        for build_time in (&mut build_times).join() {
            if build_time.is_completed == true {
                continue;
            }

            build_time.update(config.current_time);
        }
    }
}
