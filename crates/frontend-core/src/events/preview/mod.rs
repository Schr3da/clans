use common_core::entities::building::BuildingIds;

use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct ShowPreviewPayload {
    pub current_type: BuildingIds,
}

pub fn on_show_preview(current_type: BuildingIds) -> Event {
    Event {
        event_type: EventTypes::ShowPreview,
        payload: Payload::ShowPreview(ShowPreviewPayload { current_type }),
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct MovePreviewPayload {
    pub x: i32,
    pub y: i32,
}

pub fn on_move_preview(x: i32, y: i32) -> Event {
    Event {
        event_type: EventTypes::MovePreview,
        payload: Payload::MovePreview(MovePreviewPayload { x, y }),
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct HidePreviewPayload {}

pub fn on_hide_preview() -> Event {
    Event {
        event_type: EventTypes::HidePreview,
        payload: Payload::HidePreview(HidePreviewPayload {}),
    }
}
