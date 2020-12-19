pub mod building;
pub mod cancel;
pub mod map;
pub mod null;
pub mod path;
pub mod preview;
pub mod select;
pub mod theme;

use building::*;
use cancel::*;
use map::*;
use null::*;
use path::*;
use preview::*;
use select::*;
use theme::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventTypes {
    ToggleTheme,
    InitNewPath,
    StartNewPath,
    CalculateNewPath,
    EndNewPath,
    OutOfMapBounds,
    ShowPreview,
    MovePreview,
    HidePreview,
    NewBuilding,
    RemoveSelectedBuilding,
    SelectBuilding,
    Unselect,
    Cancel,
    Null,
}

#[derive(Clone)]
pub enum Payload {
    OutOfMapBounds(OutOfMapBoundsPayload),
    ToggleTheme(ToggleThemePayload),
    InitNewPath(InitNewPathPayload),
    StartNewPath(StartNewPathPayload),
    CalculateNewPath(CalculateNewPathPayload),
    EndNewPath(EndNewPathPayload),
    ShowPreview(ShowPreviewPayload),
    MovePreview(MovePreviewPayload),
    HidePreview(HidePreviewPayload),
    NewBuilding(NewBuildingPayload),
    RemoveSelectedBuilding(RemoveSelectedBuildingPayload),
    SelectBuilding(SelectBuildingPayload),
    Unselect(UnselectPayload),
    Cancel(CancelPayload),
    Null(NullPayload),
}

#[derive(Clone)]
pub struct Event {
    pub event_type: EventTypes,
    pub payload: Payload,
}
