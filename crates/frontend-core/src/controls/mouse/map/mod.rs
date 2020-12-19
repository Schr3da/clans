use common_core::entities::map::coordinates_out_of_map_range;

use crate::entities::prelude::*;

use crate::state::State;

pub fn is_out_of_bounds(state: &State, x: i32, y: i32) -> bool {
    let ecs = state.ecs.borrow();
    let data = ecs.fetch::<RendererData>();
    let map = &data.map;

    coordinates_out_of_map_range(x as usize, y as usize, map.columns, map.rows)
}
