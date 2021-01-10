use libc::wchar_t;

use common_core::entities::renderable::Renderable;
use common_core::entities::frame::Frame;

#[repr(C)]
pub struct RenderItemDto {
    pub glyph: wchar_t,
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
            glyph: renderable.glyph as wchar_t,
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
            glyph: renderable.glyph as wchar_t,
            x,
            y,
            width: 1,
            height: 1,
        }
    }

}
