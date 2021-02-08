use crate::events::*;

#[derive(Clone)]
pub struct SendNewCalculatedPathPayload{
    pub steps: Vec<usize>,
}

pub fn on_send_new_path_calculation(
    steps: Vec<usize>,
) -> Event {
    Event {
        event_type: EventTypes::SendNewCalculatedPath,
        payload: Payload::SendNewCalculatedPath(SendNewCalculatedPathPayload{ steps }),
    }
}