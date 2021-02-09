use std::slice::Iter;

use super::traits::properties::*;
use super::traits::render::Updateable;

#[derive(Debug, Copy, Clone)]
pub enum ResourceTypes {
    Materials,
    Farms,
}

static ITEMS: [ResourceTypes; 2] = [ResourceTypes::Materials, ResourceTypes::Farms];

impl IdProperty<ResourceTypes> for ResourceTypes {
    fn as_id(&self) -> &'static str {
        match *self {
            ResourceTypes::Materials => "resource-materials",
            ResourceTypes::Farms => "resource-farms",
        }
    }
}

impl LabelProperty<ResourceTypes> for ResourceTypes {
    fn as_label(&self) -> &'static str {
        match *self {
            ResourceTypes::Materials => "Materials",
            ResourceTypes::Farms => "Farms",
        }
    }
}

impl EnumIterator<ResourceTypes> for ResourceTypes {
    fn iter() -> Iter<'static, ResourceTypes> {
        ITEMS.iter()
    }
}

impl ResourceTypes {
    pub fn as_value(&self) -> i32 {
        match &self {
            ResourceTypes::Farms => 10,
            ResourceTypes::Materials => 10,
        }
    }
}

pub struct ResourceManager {
    pub materials: i32,
    pub food: i32,
    needs_update: bool,
}

impl Updateable for ResourceManager {
    fn needs_update(&self) -> bool {
        self.needs_update
    }

    fn force_update(&mut self, should_update: bool) {
        self.needs_update = should_update;
    }
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            materials: 150,
            food: 100,
            needs_update: true,
        }
    }

    pub fn income(&mut self, resource: ResourceTypes) {
        let value = resource.as_value();
        self.calculate(resource, value);
    }

    pub fn buy(&mut self, item: &impl CostsProperty) {
        let (food, materials) = item.as_costs();
        self.calculate(ResourceTypes::Farms, -food);
        self.calculate(ResourceTypes::Materials, -materials);
    }

    pub fn can_buy(&self, item: &impl CostsProperty) -> bool {
        let (food, materials) = item.as_costs();
        self.food >= food && self.materials >= materials
    }

    fn calculate(&mut self, resource: ResourceTypes, value: i32) {
        match resource {
            ResourceTypes::Materials => self.materials += value,
            ResourceTypes::Farms => self.food += value,
        };

        if self.materials < 0 {
            self.materials = 0;
        }

        if self.food < 0 {
            self.food = 0;
        }

        self.force_update(true);
    }
}
