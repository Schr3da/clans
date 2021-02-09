use common_core::events;

use crate::entities::path::*;
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
    let path_renderer = ecs.fetch::<PathRenderer>();
    let start = path_renderer.start;

    if path_renderer.id == None {
        return;
    }

    let id = path_renderer.id.unwrap();
    state
        .con
        .send_event(events::frontend::path::on_new_path_calculation_request(
            id, start.x, start.y, x, y,
        ));
}

pub fn end_new_path_at(state: &mut State, x: i32, y: i32) {
    let ecs = state.ecs.borrow();
    let mut path_renderer = ecs.fetch_mut::<PathRenderer>();
    path_renderer.set_end(x, y);
}
