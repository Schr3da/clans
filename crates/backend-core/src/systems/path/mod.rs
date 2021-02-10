use common_core::entities::prelude::*;

use crate::entities::*;
use crate::state::*;

pub fn calculate_new_path(state: &mut State, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
    let map = state.ecs.fetch::<map::Map>();
    let start = Frame::new(start_x, start_y, 1, 1);
    let target = Frame::new(end_x, end_y, 1, 1);
    let navigation = calculate_navigation(map.reference(), &start, &target);

    let mut builder = state.ecs.fetch_mut::<PathBuilder>();
    builder.force_update(true);
    builder.set(navigation.steps);
}
