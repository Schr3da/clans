use crate::entities::prelude::*;

use crate::events::*;
#[derive(Clone)]
pub struct SendConfigPayload {
    pub config: Config,
}

pub fn on_send_config(config: Config) -> Event {
    Event {
        event_type: EventTypes::SendConfig,
        payload: Payload::SendConfig(SendConfigPayload { config }),
    }
}
