use crate::state::State;

use super::preview::hide_preview;
use super::selection::unselect;

pub fn cancel(state: &mut State) {
    clean_up_state_after_event(state);
}

pub fn clean_up_state_after_event(state: &mut State) {
    hide_preview(state);
    unselect(state);
}
