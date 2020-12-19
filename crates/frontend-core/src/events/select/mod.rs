use common_core::entities::prelude::*;

use super::*;

#[derive(Clone)]
pub struct SelectBuildingPayload {
    pub renderable: Renderable<BuildingIds>,
    pub frame: Frame,
}

pub fn on_select(frame: Frame, renderable: Renderable<BuildingIds>) -> Event {
    Event {
        event_type: EventTypes::SelectBuilding,
        payload: Payload::SelectBuilding(SelectBuildingPayload { frame, renderable }),
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct UnselectPayload {}

pub fn on_unselect() -> Event {
    Event {
        event_type: EventTypes::Unselect,
        payload: Payload::Unselect(UnselectPayload {}),
    }
}
