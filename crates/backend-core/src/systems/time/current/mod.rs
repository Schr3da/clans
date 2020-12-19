use specs::prelude::*;

use common_core::prelude::*;

pub struct CurrentTimeSystem {}

impl Default for CurrentTimeSystem {
    fn default() -> Self {
        CurrentTimeSystem {}
    }
}

impl<'a> System<'a> for CurrentTimeSystem {
    type SystemData = WriteExpect<'a, Config>;

    fn run(&mut self, data: Self::SystemData) {
        let mut config = data;
        config.current_time = current_timestamp(); 
    }
}
