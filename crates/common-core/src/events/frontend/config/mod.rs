use crate::events::*;

#[derive(Clone)]
pub struct RequestConfigPayload {}

pub fn on_request_config() -> Event {
    Event {
        event_type: EventTypes::RequestConfig,
        payload: Payload::RequestConfig(RequestConfigPayload {}),
    }
}
