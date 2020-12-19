use common_core::prelude::*;

use crate::entities::selection::Selection;
use crate::state::State;

pub fn select_building(state: &mut State, renderable: Renderable<BuildingIds>, frame: Frame) {
    let ecs = state.ecs.borrow();

    let mut selection = ecs.fetch_mut::<Selection>();
    selection.show(frame, renderable);

    let config = ecs.fetch::<Config>();
    selection.change_theme(config.current_theme);
}

pub fn unselect(state: &mut State) {
    let ecs = state.ecs.borrow();
    let mut selection = ecs.fetch_mut::<Selection>();
    selection.hide();
}
