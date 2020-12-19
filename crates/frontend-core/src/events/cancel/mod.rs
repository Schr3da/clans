use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct CancelPayload {}

pub fn on_cancel() -> Event {
    Event {
        event_type: EventTypes::Cancel,
        payload: Payload::Cancel(CancelPayload {}),
    }
}
