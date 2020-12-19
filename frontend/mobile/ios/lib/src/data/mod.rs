use frontend_core::prelude::*;

use crate::input::keyboard;
use crate::input::mouse;
use crate::dto::RenderItemDto;

pub struct Data {
    pub state: State,
    map_tile_renderer: extern fn(RenderItemDto, bool),
    building_renderer: extern fn(RenderItemDto, i64),
    unit_renderer: extern fn(RenderItemDto),
    preview_renderer: extern fn(RenderItemDto),
    selection_renderer: extern fn(RenderItemDto)
}

impl Data {
    pub fn new(
        map_tile_renderer: extern fn(RenderItemDto, bool),
        building_renderer: extern fn(RenderItemDto, i64),
        unit_renderer: extern fn(RenderItemDto),
        preview_renderer: extern fn(RenderItemDto),
        selection_renderer: extern fn(RenderItemDto)
    ) -> Self {
        let should_always_rerender = true;
        let mut state = State::new(should_always_rerender);
        state.setup();

        Data {
            state,
            map_tile_renderer,
            building_renderer,
            unit_renderer,
            preview_renderer,
            selection_renderer
        }
    }

    pub fn handle_mouse_down(&mut self, is_left_click: bool, x: i32, y: i32) {
        let mouse_event = mouse::inputs(self, is_left_click, x, y);
        self.state.handle_event(mouse_event);
        println!("handle click");
    }

    pub fn handle_mouse_move(&mut self, x: i32, y: i32) {
        let mouse_event = mouse::inputs(self, false, x, y);
        self.state.handle_event(mouse_event);
    }

    pub fn handle_key_up(&mut self, keycode: String) {
        let keyboard_event = keyboard::inputs(self, keycode);
        self.state.handle_event(keyboard_event);
    }

    pub fn update(&mut self) {
        self.state.run_systems();
    }

    pub fn render(&mut self) {
        self.state.map_tile_renderer(&mut |tile, is_visible, x, y| {
            let item = RenderItemDto::to_dto(tile, x, y);
            (self.map_tile_renderer)(item, is_visible);
        });

        self.state.building_renderer(&mut |frame, time, building| {
            let item = RenderItemDto::to_dto_with_frame(frame, building);
            (self.building_renderer)(item, time.progress);
        });

        self.state.unit_renderer(&mut |frame, unit| {
            let item = RenderItemDto::to_dto_with_frame(frame, unit);
            (self.unit_renderer)(item);
        });

        self.state.preview_renderer(&mut |frame, preview| {
            let item = RenderItemDto::to_dto_with_frame(frame, preview);
            (self.preview_renderer)(item);
        });

        self.state.selection_renderer(&mut |frame, selection| {
            let item = RenderItemDto::to_dto_with_frame(frame, selection);
            (self.selection_renderer)(item);
        });
    }
}
