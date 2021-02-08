use crate::events::*;

#[derive(Clone)]
pub struct RequestCalculateNewPathPayload{
    pub start_x: i32,
    pub start_y: i32,
    pub end_x: i32,
    pub end_y: i32,
}

pub fn on_new_path_calculation_request(
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
) -> Event {
    Event {
        event_type: EventTypes::RequestCalculateNewPath,
        payload: Payload::RequestCalculateNewPath(RequestCalculateNewPathPayload{ start_x, start_y, end_x, end_y }),
    }
}