use crate::systems::prelude::*;

use crate::events::*;

use crate::state::State;

pub fn dispatch_frontend_event(state: &mut State, event: Event) {
    match event.payload {
        Payload::OutOfMapBounds(_) => common_core::utils::log::print_string("out of map bounds".to_owned()),
        Payload::ToggleTheme(_) => toggle_theme(state),
        Payload::InitNewPath(_) => init_new_path(state),
        Payload::StartNewPath(payload) => start_new_path_at(state, payload.x, payload.y),
        Payload::CalculateNewPath(payload) => calculate_new_path(state, payload.x, payload.y),
        Payload::EndNewPath(payload) => end_new_path_at(state, payload.x, payload.y),
        Payload::ShowPreview(payload) => show_preview(state, payload.current_type),
        Payload::MovePreview(payload) => move_preview(state, payload.x, payload.y),
        Payload::HidePreview(_) => hide_preview(state),
        Payload::NewBuilding(payload) => new_building(state, payload.id, payload.x, payload.y),
        Payload::RemoveSelectedBuilding(_) => remove_selected_building(state),
        Payload::SelectBuilding(payload) => select_building(state, payload.renderable, payload.frame),
        Payload::Unselect(_) => unselect(state),
        Payload::Cancel(_) => cancel(state),
        Payload::Null(_) => {}
    }
}
