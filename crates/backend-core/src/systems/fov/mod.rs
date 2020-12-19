use specs::prelude::*;

use common_core::prelude::*;

use crate::algorithms::fov;
use crate::entities::map::Map;

pub struct FieldOfViewSystem {}

impl Default for FieldOfViewSystem {
    fn default() -> Self {
        FieldOfViewSystem {}
    }
}

impl<'a> System<'a> for FieldOfViewSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, Frame>,
        ReadStorage<'a, Renderable<BuildingIds>>,
        ReadStorage<'a, Renderable<UnitIds>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut fovs, frames, buildings, units) = data;

        for (entity, fov, frame) in (&entities, &mut fovs, &frames).join() {
            if fov.needs_update() == false {
                continue;
            }

            fov.force_update(false);
            fov.tiles.clear();
            fov.tiles = fov::calculate(
                frame.x,
                frame.y,
                fov.range,
                &map,
            );

            match buildings.get(entity) {
                None => {},
                Some(_) => fov::update_visble_tiles_for_fov_with_values(&mut map, &fov, true),
            }

            match units.get(entity) {
                None => {},
                Some(_) => fov::update_visble_tiles_for_fov_with_values(&mut map, &fov, true),
            }
        }
    }
}
