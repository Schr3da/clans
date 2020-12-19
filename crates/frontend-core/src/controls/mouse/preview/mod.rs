use common_core::entities::building::BuildingIds;

use crate::state::State;

use crate::entities::preview::Preview;

pub fn should_trigger_preview_event(state: &State) -> bool {
    let ecs = state.ecs.borrow();
    let preview = ecs.fetch::<Preview>();

    match preview.renderable {
        Some(_) => true,
        None => false,
    }
}

pub fn should_trigger_new_building_event(state: &State) -> Option<BuildingIds> {
    let ecs = state.ecs.borrow();
    let preview = ecs.fetch::<Preview>();

    match &preview.renderable {
        Some(renderable) => Some(renderable.current_type),
        None => None,
    }
}
