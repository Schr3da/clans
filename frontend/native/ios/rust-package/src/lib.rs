mod data;
mod input;
mod dto;

use data::Data;
use dto::RenderItemDto;

#[no_mangle]
pub extern "C" fn init_game_state(
    map_tile_renderer: extern fn(RenderItemDto, bool),
    building_renderer: extern fn(RenderItemDto, i64),
    unit_renderer: extern fn(RenderItemDto),
    preview_renderer: extern fn(RenderItemDto),
    selection_renderer: extern fn(RenderItemDto)
) -> *mut Data {
    Box::into_raw(Box::new(Data::new(
        map_tile_renderer,
        building_renderer,
        unit_renderer,
        preview_renderer,
        selection_renderer
    )))
}

#[no_mangle]
pub unsafe extern "C" fn update_state(data: *mut Data) {
    assert!(data.is_null() == false);

    let mut state = Box::from_raw(data);
    state.update();
    std::mem::forget(state);
}

#[no_mangle]
pub unsafe extern "C" fn render_state(
    data: *mut Data,
) {
    assert!(data.is_null() == false);

    let mut state = Box::from_raw(data);
    state.render();
    std::mem::forget(state);
}

#[no_mangle]
pub unsafe extern "C" fn free_game_state(data: *mut Data) {
    Box::from_raw(data);
}

#[no_mangle]
pub unsafe extern "C" fn on_touch_down(data: *mut Data, is_left_click: bool, x: i32, y: i32) {
    assert!(data.is_null() == false);

    let mut state = Box::from_raw(data);
    state.handle_mouse_down(is_left_click, x, y);
    std::mem::forget(state);
}

#[no_mangle]
pub unsafe extern "C" fn on_touch_move(data: *mut Data, x: i32, y: i32) {
    assert!(data.is_null() == false);
    
    let mut state = Box::from_raw(data);
    state.handle_mouse_move(x, y);
    std::mem::forget(state);
}

#[no_mangle]
pub extern "C" fn on_key_up() {
    println!("on_key_up");
}
