use common_core::prelude::*;

use crate::entities::prelude::*;
use crate::state::State;

pub fn show_preview(state: &mut State, id: BuildingIds) {
    let ecs = state.ecs.borrow();
    let config = ecs.fetch::<Config>();

    let mut preview = ecs.fetch_mut::<Preview>();
    preview.show(id, config.current_theme);
}

pub fn move_preview(state: &mut State, x: i32, y: i32) {
    let ecs = state.ecs.borrow();
    let data = ecs.fetch::<RendererData>();
    let map = &data.map;

    let mut preview = ecs.fetch_mut::<Preview>();
    preview.update_position(x, y);

    let is_colliding =
        has_tile_collision_for_frame(&preview.frame, &map.tiles, map.columns, map.rows);
    preview.handle_collision(is_colliding);
}

pub fn hide_preview(state: &State) {
    let ecs = state.ecs.borrow();
    let mut preview = ecs.fetch_mut::<Preview>();
    preview.hide();
}
