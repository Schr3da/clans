use wasm_bindgen::prelude::*;

use frontend_core::prelude::*;

use crate::dto::*;

use crate::input::keyboard;
use crate::input::mouse;

#[wasm_bindgen]
extern "C" {
    fn map_tile_renderer(id: &str, item: RenderItemDto, visible: bool, index: usize, total: usize);
    fn building_renderer(id: &str, item: RenderItemDto, progress: i64, index: usize, total: usize);
    fn unit_renderer(id: &str, item: RenderItemDto, index: usize, total: usize);
    fn preview_renderer(item: Option<RenderItemDto>);
    fn path_builder_renderer(item: Option<Vec<usize>>);
    fn selection_renderer(item: Option<RenderItemDto>);
    fn resources_renderer(food: i32, materials: i32);
    fn render_cycle_completed();
    fn mouse_event() -> Option<MouseEventDto>;
    fn keyboard_event() -> Option<KeyboardEventDto>;
}

pub struct Data {
    pub state: State,
}

impl Data {
    pub fn new() -> Self {
        let should_always_rerender = false;
        let mut state = State::new(should_always_rerender);
        state.setup();

        Data { state }
    }

    fn handle_mouse_up(&mut self, button_type: MouseButtonType, x: i32, y: i32) {
        let is_left_button_clicked = button_type == MouseButtonType::Left;
        let mouse_event = mouse::inputs(self, is_left_button_clicked, x, y);
        self.state.handle_event(mouse_event);
    }

    fn handle_mouse_move(&mut self, x: i32, y: i32) {
        let mouse_event = mouse::inputs(self, false, x, y);
        self.state.handle_event(mouse_event);
    }

    fn handle_key_up(&mut self, keycode: String) {
        let keyboard_event = keyboard::inputs(self, keycode);
        self.state.handle_event(keyboard_event);
    }

    pub fn inputs(&mut self) {
        if let Some(e) = keyboard_event() {
            match e.event_type {
                KeyboardEventType::KeyUp => self.handle_key_up(e.keycode.to_string()),
                KeyboardEventType::KeyDown => {}
            }
        }

        if let Some(e) = mouse_event() {
            match e.event_type {
                MouseEventType::ButtonDown => {}
                MouseEventType::Move => self.handle_mouse_move(e.x, e.y),
                MouseEventType::ButtonUp => self.handle_mouse_up(e.button_type, e.x, e.y),
            }
        }
    }

    pub fn update(&mut self) {
        self.state.run_systems();
    }

    pub fn render(&mut self) {
        self.state
            .map_tile_renderer(&mut |tile, is_visible, x, y, index, total| {
                let item = RenderItemDto::to_dto(tile, x, y);
                map_tile_renderer(tile.id.as_str(), item, is_visible, index, total);
            });

        self.state
            .building_renderer(&mut |frame, time, building, index, total| {
                let item = RenderItemDto::to_dto_with_frame(frame, building);
                building_renderer(building.id.as_str(), item, time.progress, index, total);
            });

        self.state.unit_renderer(&mut |frame, unit, index, total| {
            let item = RenderItemDto::to_dto_with_frame(frame, unit);
            unit_renderer(unit.id.as_str(), item, index, total);
        });

        self.state.preview_renderer(&mut |frame, preview| {
            match preview {
                None => preview_renderer(Option::None),
                Some(data) => {
                    let item = RenderItemDto::to_dto_with_frame(frame, data);
                    preview_renderer(Option::Some(item));
                }
            };
        });

        self.state.selection_renderer(&mut |frame, selection| {
            match selection {
                None => selection_renderer(Option::None),
                Some(data) => {
                    let item = RenderItemDto::to_dto_with_frame(frame, data);
                    selection_renderer(Option::Some(item));
                }
            };
        });

        self.state.path_builder_renderer(&mut |data| {
            path_builder_renderer(data);
        });

        self.state.resources_renderer(&mut |resources| {
            resources_renderer(resources.food, resources.materials);
        });

        render_cycle_completed();
    }
}
