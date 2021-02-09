use specs::prelude::*;

use common_core::prelude::*;

use crate::entities::map::Map;

pub struct UnitSystem {}

impl Default for UnitSystem {
    fn default() -> Self {
        UnitSystem {}
    }
}

impl<'a> System<'a> for UnitSystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadStorage<'a, IntervalTimer>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, Frame>,
        WriteStorage<'a, Path>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, intervals, mut fovs, mut frames, mut paths) = data;

        for (interval, fov, frame, path) in (&intervals, &mut fovs, &mut frames, &mut paths).join()
        {
            if interval.needs_update() == false {
                continue;
            }

            path.move_forward();

            let step = path.current_step();
            if step == None {
                continue;
            }

            let (x, y) = map_index_to_coordinates(step.unwrap(), map.columns);
            frame.set_coordinates(x, y);

            fov.force_update(true);
        }
    }
}
