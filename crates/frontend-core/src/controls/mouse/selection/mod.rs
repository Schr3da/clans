use common_core::entities::prelude::*;

use crate::entities::renderer::RendererData;
use crate::entities::selection::Selection;
use crate::state::State;

pub fn should_select_renderable(
    state: &State,
    x: i32,
    y: i32,
) -> Option<(Frame, Renderable<BuildingIds>)> {
    let ecs = state.ecs.borrow();
    let data = ecs.fetch::<RendererData>();

    for (frame, _build_time, building) in &data.buildings {
        match frame.contains_coordinates(x, y) {
            true => return Some((frame.clone(), building.clone())),
            false => {}
        };
    }

    None
}

pub fn should_unselect(state: &State) -> bool {
    let ecs = state.ecs.borrow();
    let selection = ecs.fetch::<Selection>();

    match &selection.renderable {
        Some(_) => true,
        None => false,
    }
}
