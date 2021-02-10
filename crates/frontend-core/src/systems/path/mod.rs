use common_core::entities::prelude::*;

use crate::{entities::path::*, prelude::RendererData};
use crate::state::State;

pub fn init_new_path(state: &mut State) {
    let ecs = state.ecs.borrow();
    let mut path_renderer = ecs.fetch_mut::<PathRenderer>();
    path_renderer.prepare();
}

pub fn start_new_path_at(state: &mut State, x: i32, y: i32) {
    let ecs = state.ecs.borrow();
    let mut path_renderer = ecs.fetch_mut::<PathRenderer>();
    path_renderer.set_start(x, y);
}

pub fn calculate_new_path(state: &mut State, x: i32, y: i32) {
    let ecs = state.ecs.borrow();
    let data= ecs.fetch::<RendererData>();

    let mut path_renderer = ecs.fetch_mut::<PathRenderer>();
    let start = path_renderer.start;
    let end = path_renderer.end;

    if end.x == x && end.y == y {
        return;
    }

    let start = Frame::new(start.x, start.y, 1, 1);
    let target = Frame::new(x, y, 1, 1);
    let navigation = calculate_navigation(data.map.reference(), &start, &target);


    path_renderer.update(x, y, navigation.steps);
    path_renderer.force_update(true);
}

pub fn end_new_path_at(state: &mut State, x: i32, y: i32) {
    let ecs = state.ecs.borrow();
    let mut path_renderer = ecs.fetch_mut::<PathRenderer>();
    path_renderer.set_end(x, y);
}
