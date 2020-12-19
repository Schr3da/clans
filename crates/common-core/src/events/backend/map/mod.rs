use crate::entities::prelude::*;

use crate::events::*;

#[derive(Clone)]
pub struct SendMapPayload {
    pub map: Map,
}

pub fn on_send_map(map: Map) -> Event {
    Event {
        event_type: EventTypes::SendMap,
        payload: Payload::SendMap(SendMapPayload { map }),
    }
}
