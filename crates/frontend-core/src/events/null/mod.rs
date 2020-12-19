use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct NullPayload {}

pub fn on_null() -> Event {
    Event {
        event_type: EventTypes::Null,
        payload: Payload::Null(NullPayload {}),
    }
}
