pub mod backend;
pub mod frontend;

use backend::prelude::*;
use frontend::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EventTypes {
    NewBuilding,
    RequestMap,
    SendMap,
    RequestConfig,
    SendConfig,
    SendRenderUpdate,
    SendRefreshTheme,
    RemoveBuilding,
}

#[derive(Clone)]
pub enum Payload {
    NewBuilding(NewBuildingPayload),
    RemoveBuilding(RemoveBuildingPayload),
    RequestMap(RequestMapPayload),
    SendMap(SendMapPayload),
    RequestConfig(RequestConfigPayload),
    SendConfig(SendConfigPayload),
    SendRenderUpdate(SendRenderUpdatePayload),
    SendRefreshTheme(SendRefreshThemePayload),
}

#[derive(Clone)]
pub struct Event {
    pub event_type: EventTypes,
    pub payload: Payload,
}

pub type EventCallback = Rc<RefCell<Box<dyn FnMut(Event)>>>;

pub mod prelude {
    pub use super::backend::prelude::*;
    pub use super::frontend::prelude::*;
    pub use super::*;
}
