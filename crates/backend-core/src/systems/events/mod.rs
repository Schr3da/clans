use common_core::events;
use common_core::prelude::*;

use crate::entities::map;
use crate::state::State;
use crate::systems::building::*;
use crate::systems::path::*;

fn handle_new_building(state: &mut State, payload: events::backend::building::NewBuildingPayload) {
    let id = payload.current_type;
    new_building(
        state,
        &id,
        id.as_frame_for_position(payload.frame.x, payload.frame.y),
    );
}

fn handle_remove_building(
    state: &mut State,
    payload: events::backend::building::RemoveBuildingPayload,
) {
    remove_building(state, payload.x, payload.y);
}

fn handle_request_config(state: &mut State) {
    let config = state.ecs.fetch::<Config>();
    state.send(events::backend::config::on_send_config(*config));
}

fn handle_request_map(state: &mut State) {
    let map = state.ecs.fetch::<map::Map>();
    state.send(events::backend::map::on_send_map({
        common_core::entities::map::Map {
            length: map.length,
            columns: map.columns,
            rows: map.rows,
            tiles: map.tiles.clone(),
            visited_tiles: map.visited_tiles.clone(),
            visible_tiles: map.visible_tiles.clone(),
            needs_update: map.needs_update(),
        }
    }));
}

fn handle_calculate_new_path(
    state: &mut State,
    payload: events::frontend::path::RequestCalculateNewPathPayload,
) {
    calculate_new_path(
        state,
        payload.start_x,
        payload.start_y,
        payload.end_x,
        payload.end_y,
    );
}

pub fn dispatch_event(state: &mut State, event: Event) {
    match event.payload {
        Payload::NewBuilding(payload) => handle_new_building(state, payload),
        Payload::RequestConfig(_) => handle_request_config(state),
        Payload::RequestMap(_) => handle_request_map(state),
        Payload::RequestCalculateNewPath(payload) => handle_calculate_new_path(state, payload),
        Payload::RemoveBuilding(payload) => handle_remove_building(state, payload),
        _ => {}
    }
}
