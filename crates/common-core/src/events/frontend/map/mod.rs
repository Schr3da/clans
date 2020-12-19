use crate::events::*;

#[derive(Clone)]
pub struct RequestMapPayload {}

pub fn on_request_map() -> Event {
    Event {
        event_type: EventTypes::RequestMap,
        payload: Payload::RequestMap(RequestMapPayload {}),
    }
}
