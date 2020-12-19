use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct ToggleThemePayload {}

pub fn on_toggle_theme() -> Event {
    Event {
        event_type: EventTypes::ToggleTheme,
        payload: Payload::ToggleTheme(ToggleThemePayload {}),
    }
}
