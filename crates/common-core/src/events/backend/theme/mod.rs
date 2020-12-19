use crate::events::*;

#[derive(Clone)]
pub struct SendRefreshThemePayload {}

pub fn on_send_refresh_theme() -> Event {
    Event {
        event_type: EventTypes::SendRefreshTheme,
        payload: Payload::SendRefreshTheme(SendRefreshThemePayload {}),
    }
}
