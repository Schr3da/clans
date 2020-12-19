use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct OutOfMapBoundsPayload {}

pub fn on_out_of_map_bounds() -> Event {
    Event {
        event_type: EventTypes::OutOfMapBounds,
        payload: Payload::OutOfMapBounds(OutOfMapBoundsPayload {}),
    }
}
