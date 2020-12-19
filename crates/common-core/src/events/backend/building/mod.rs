use crate::entities::prelude::*;

use crate::events::*;

#[derive(Copy, Clone)]
pub struct NewBuildingPayload {
    pub frame: Frame,
    pub current_type: BuildingIds,
}

pub fn on_new_building(id: BuildingIds, x: i32, y: i32) -> Event {
    Event {
        event_type: EventTypes::NewBuilding,
        payload: Payload::NewBuilding(NewBuildingPayload {
            frame: id.as_frame_for_position(x, y),
            current_type: id,
        }),
    }
}

#[derive(Copy, Clone)]
pub struct RemoveBuildingPayload {
    pub x: i32,
    pub y: i32
}

pub fn on_remove_building(x: i32, y: i32) -> Event {
    Event {
        event_type: EventTypes::RemoveBuilding,
        payload: Payload::RemoveBuilding(RemoveBuildingPayload {
            x, y, 
        }),
    }
}
