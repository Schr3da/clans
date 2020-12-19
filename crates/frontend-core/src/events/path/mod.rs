use crate::entities::path::*;

use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct InitNewPathPayload {
    pub path_state: PathStates,
}

pub fn on_init_new_path() -> Event {
    Event {
        event_type: EventTypes::StartNewPath,
        payload: Payload::InitNewPath(InitNewPathPayload {
            path_state: PathStates::OnPrepare,
        }),
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct StartNewPathPayload {
    pub path_state: PathStates,
    pub x: i32,
    pub y: i32,
}

pub fn on_start_new_path(x: i32, y: i32) -> Event {
    Event {
        event_type: EventTypes::StartNewPath,
        payload: Payload::StartNewPath(StartNewPathPayload {
            path_state: PathStates::OnSetStartPoint,
            x,
            y,
        }),
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct CalculateNewPathPayload {
    pub path_state: PathStates,
    pub x: i32,
    pub y: i32,
}

pub fn on_calculate_new_path(x: i32, y: i32) -> Event {
    Event {
        event_type: EventTypes::CalculateNewPath,
        payload: Payload::CalculateNewPath(CalculateNewPathPayload {
            path_state: PathStates::OnMove,
            x,
            y,
        }),
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct EndNewPathPayload {
    pub path_state: PathStates,
    pub x: i32,
    pub y: i32,
}

pub fn on_end_new_path(x: i32, y: i32) -> Event {
    Event {
        event_type: EventTypes::EndNewPath,
        payload: Payload::EndNewPath(EndNewPathPayload {
            path_state: PathStates::OnSetEndPoint,
            x,
            y,
        }),
    }
}
