mod map;
mod path;
mod preview;
mod selection;

use map::*;
use path::*;
use preview::*;
use selection::*;

use crate::events;

use crate::state::State;

pub fn handle_move(x: i32, y: i32, state: &State) -> events::Event {
    if is_out_of_bounds(state, x, y) == true {
        return events::map::on_out_of_map_bounds();
    }

    if should_trigger_move_path_event(state) == true {
        return events::path::on_calculate_new_path(x, y);
    }

    if should_trigger_preview_event(state) == true {
        return events::preview::on_move_preview(x, y);
    }

    events::null::on_null()
}

pub fn handle_click(x: i32, y: i32, state: &State) -> events::Event {
    if is_out_of_bounds(state, x, y) == true {
        return events::map::on_out_of_map_bounds();
    }

    if let Some(id) = should_trigger_new_building_event(state) {
        return events::building::on_new_building(id, x, y);
    }

    if should_trigger_start_path_event(state) {
        return events::path::on_start_new_path(x, y);
    }

    if let Some((frame, renderable)) = should_select_renderable(state, x, y) {
        return events::select::on_select(frame, renderable);
    }

    if should_unselect(state) {
        return events::select::on_unselect();
    }

    events::null::on_null()
}
