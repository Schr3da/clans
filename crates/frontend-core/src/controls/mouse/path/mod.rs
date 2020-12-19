use crate::state::State;

use crate::entities::path::{PathRenderer, PathStates};

pub fn should_trigger_start_path_event(state: &State) -> bool {
    let ecs = state.ecs.borrow();
    let path_renderer = ecs.fetch::<PathRenderer>();

    path_renderer.current_state == PathStates::OnSetStartPoint
}

pub fn should_trigger_move_path_event(state: &State) -> bool {
    let ecs = state.ecs.borrow();
    let path_renderer = ecs.fetch::<PathRenderer>();

    path_renderer.current_state == PathStates::OnMove
}
