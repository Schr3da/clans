use common_core::entities::building::BuildingIds;

use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct NewBuildingPayload {
    pub id: BuildingIds,
    pub x: i32,
    pub y: i32,
}

pub fn on_new_building(id: BuildingIds, x: i32, y: i32) -> Event {
    Event {
        event_type: EventTypes::NewBuilding,
        payload: Payload::NewBuilding(NewBuildingPayload { id, x, y }),
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct RemoveSelectedBuildingPayload {}

pub fn on_remove_selected_building() -> Event {
    Event {
        event_type: EventTypes::RemoveSelectedBuilding,
        payload: Payload::RemoveSelectedBuilding(RemoveSelectedBuildingPayload{}),
    }
}
