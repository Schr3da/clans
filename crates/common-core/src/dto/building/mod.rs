use crate::entities::prelude::*;

pub type BuildingDto = (Frame, BuildTime, Renderable<BuildingIds>);

pub type BuildingDtos = Vec<BuildingDto>;
