use common_core::events::backend::building::*;

use common_core::entities::prelude::*;

use crate::state::State;

use crate::entities::selection::Selection;

use super::general::clean_up_state_after_event;

pub fn new_building(state: &mut State, id: BuildingIds, x: i32, y: i32) {
    state.con.send_event(on_new_building(id, x, y));
    clean_up_state_after_event(state);
}

pub fn remove_selected_building(state: &mut State) {
    {
        let ecs = state.ecs.borrow();
        let selection = ecs.fetch::<Selection>();

        if selection.is_selected() == false {
            return;
        }

        state.con.send_event(on_remove_building(selection.frame.x, selection.frame.y));
    }
    
    clean_up_state_after_event(state);
}
