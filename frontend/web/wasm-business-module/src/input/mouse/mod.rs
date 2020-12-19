use frontend_core::controls::mouse::*;
use frontend_core::events::Event;

use crate::data::Data;

pub fn inputs(data: &Data, is_left_click: bool, x: i32, y: i32) -> Event {
    match is_left_click {
        true => handle_click(x, y, &data.state),
        false => handle_move(x, y, &data.state),
    }
}
