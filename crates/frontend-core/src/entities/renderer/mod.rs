use specs::prelude::*;
use specs_derive::*;

use common_core::prelude::*;

#[derive(Component)]
pub struct RendererData {
    pub map: Map,
    pub buildings: BuildingDtos,
    pub units: UnitDtos,
    pub resources: Option<ResourcesDto>,
}

impl Themeable for RendererData {
    fn change_theme(&mut self, next: ThemeTypes) {
        self.map.change_theme(next);

        for (_frame, _time, building) in &mut self.buildings {
            building.change_theme(next);
        }

        for (_frame, unit) in &mut self.units {
            unit.change_theme(next);
        }
    }
}

impl RendererData {
    pub fn new() -> Self {
        RendererData {
            map: Map::default(),
            buildings: Vec::<BuildingDto>::new(),
            units: Vec::<UnitDto>::new(),
            resources: None, 
        }
    }
}
