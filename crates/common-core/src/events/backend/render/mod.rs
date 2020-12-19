use crate::prelude::*;

#[derive(Clone)]
pub struct SendRenderUpdatePayload {
    pub map: Option<Map>,
    pub buildings: Option<BuildingDtos>,
    pub units: Option<UnitDtos>,
    pub resources: Option<ResourcesDto>,
}

pub fn on_send_render_update(
    map: Option<Map>,
    buildings: Option<BuildingDtos>, 
    units: Option<UnitDtos>,
    resources: Option<ResourcesDto>
) -> Event {
    Event {
        event_type: EventTypes::SendRenderUpdate,
        payload: Payload::SendRenderUpdate(SendRenderUpdatePayload {
            map,
            buildings,
            units,
            resources,
        }),
    }
}
