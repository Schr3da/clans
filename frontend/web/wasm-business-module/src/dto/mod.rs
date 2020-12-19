use std::marker::Copy;
use wasm_bindgen::prelude::*;

use common_core::entities::renderable::Renderable;
use common_core::entities::frame::Frame;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum MouseEventType {
    pub ButtonDown,
    pub ButtonUp,
    pub Move,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MouseButtonType {
    pub Left = 0,
    pub Middle = 1,
    pub Right = 2,
    pub None = 100,
}

#[wasm_bindgen]
pub struct MouseEventDto {
    pub event_type: MouseEventType,
    pub button_type: MouseButtonType, 
    pub x: i32,
    pub y: i32,
}

#[wasm_bindgen]
impl MouseEventDto {
    pub fn new(event_type: MouseEventType, button_type: MouseButtonType, x: i32,  y: i32) -> Self {
        MouseEventDto {
            event_type, 
            button_type,
            x, 
            y,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum KeyboardEventType {
    pub KeyDown,
    pub KeyUp,
}

#[wasm_bindgen]
pub struct KeyboardEventDto {
    pub event_type: KeyboardEventType,
    pub keycode: char,
}

#[wasm_bindgen]
impl KeyboardEventDto {
    pub fn new(event_type: KeyboardEventType, keycode: char) -> Self {
        KeyboardEventDto {
            event_type,
            keycode,
        }
    }
}

#[wasm_bindgen]
pub struct RenderItemDto {
    pub glyph: char,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl RenderItemDto {
    pub fn to_dto_with_frame<T>(frame: &Frame, renderable: &Renderable<T>) -> Self 
    where T: Send + Sync + 'static,
    {
        RenderItemDto {
            glyph: renderable.glyph,
            x: frame.x,
            y: frame.y,
            width: frame.width,
            height: frame.height,
        }
    }

    pub fn to_dto<T>(renderable: &Renderable<T>, x: i32, y: i32) -> Self 
    where T: Send + Sync + 'static,
    {
        RenderItemDto {
            glyph: renderable.glyph,
            x,
            y,
            width: 1,
            height: 1,
        }
    }
}
