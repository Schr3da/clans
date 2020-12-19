use std::slice::Iter;

use crate::theme::*;
use crate::utils::id::create_random_id;

use super::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BuildingIds {
    Mine,
    Farm,
    Factory,
}

static ITEMS: [BuildingIds; 3] = [BuildingIds::Mine, BuildingIds::Farm, BuildingIds::Factory];

impl IdProperty<BuildingIds> for BuildingIds {
    fn as_id(&self) -> &'static str {
        match self {
            BuildingIds::Mine => "building-mine",
            BuildingIds::Farm => "building-farm",
            BuildingIds::Factory => "building-factory",
        }
    }
}

impl LabelProperty<BuildingIds> for BuildingIds {
    fn as_label(&self) -> &'static str {
        match self {
            BuildingIds::Mine => "Mine",
            BuildingIds::Farm => "Farm",
            BuildingIds::Factory => "Factory",
        }
    }
}

impl GlyphProperty<BuildingIds> for BuildingIds {
    fn as_glyph(&self) -> char {
        match self {
            BuildingIds::Farm => 'H',
            BuildingIds::Mine => 'M',
            BuildingIds::Factory => 'F',
        }
    }
}

impl ColorsProperty<BuildingIds> for BuildingIds {
    fn as_colors(&self, current_theme: ThemeTypes) -> (Color, Color) {
        let (background, foreground) = match self {
            BuildingIds::Farm => (color_11(current_theme), color_14(current_theme)),
            BuildingIds::Mine => (color_11(current_theme), color_10(current_theme)),
            BuildingIds::Factory => (color_11(current_theme), color_10(current_theme)),
        };

        (background, foreground)
    }
}

impl DescriptionProperty<BuildingIds> for BuildingIds {
    fn as_description(&self) -> Description {
        Description {
            content: self.as_label(),
        }
    }
}

impl FrameProperty<BuildingIds> for BuildingIds {
    fn as_frame(&self) -> Frame {
        self.as_frame_for_position(0, 0)
    }

    fn as_frame_for_position(&self, x: i32, y: i32) -> Frame {
        match self {
            BuildingIds::Mine => Frame::new(x, y, 1, 1),
            BuildingIds::Farm => Frame::new(x, y, 1, 1),
            BuildingIds::Factory => Frame::new(x, y, 1, 1),
        }
    }
}

impl FieldOfViewProperty<BuildingIds> for BuildingIds {
    fn as_field_of_view(&self) -> FieldOfView {
        match self {
            BuildingIds::Mine => FieldOfView::new(6),
            BuildingIds::Farm => FieldOfView::new(1),
            BuildingIds::Factory => FieldOfView::new(2),
        }
    }
}

impl TimeProperty<BuildingIds> for BuildingIds {
    fn as_build_time(&self) -> BuildTime {
        match self {
            BuildingIds::Mine => BuildTime::new(18),
            BuildingIds::Farm => BuildTime::new(6),
            BuildingIds::Factory => BuildTime::new(10),
        }
    }
}

impl SpawnProperty<UnitIds> for BuildingIds {
    fn as_spawn_time(&self) -> Option<SpawnTime> {
        match self {
            BuildingIds::Farm => Some(SpawnTime::new(5)), 
            BuildingIds::Mine => Some(SpawnTime::new(10)),
            BuildingIds::Factory => Some(SpawnTime::new(5)),
        }
    }

    fn as_spawn_entity(&self) -> Option<UnitIds> {
        match self {
            BuildingIds::Mine => None,
            BuildingIds::Farm => None,
            BuildingIds::Factory => Some(UnitIds::Tank),
        }
    }

    fn as_spawn_resource(&self) -> Option<ResourceTypes> {
        match self {
            BuildingIds::Mine => Some(ResourceTypes::Materials),
            BuildingIds::Farm => Some(ResourceTypes::Farms),
            _ => None,
        }
    }
}

impl RenderProperty<BuildingIds> for BuildingIds {
    fn as_renderable(self) -> Renderable<BuildingIds> {
        self.as_renderable_with_prefix("building".to_owned())
    }

    fn as_renderable_with_prefix(self, prefix: String) -> Renderable<BuildingIds> {
        let (background, foreground) = self.as_colors(ThemeTypes::Dark);

        Renderable {
            id: create_random_id(prefix),
            current_type: self,
            glyph: self.as_glyph(),
            foreground,
            background,
        }
    }
}

impl CostsProperty for BuildingIds {
    fn as_costs(&self) -> (i32, i32) {
        match self {
            BuildingIds::Mine => (50, 50),
            BuildingIds::Farm => (20, 0),
            BuildingIds::Factory => (40, 10),
        }
    }
}

impl EnumIterator<BuildingIds> for BuildingIds {
    fn iter() -> Iter<'static, BuildingIds> {
        ITEMS.iter()
    }
}
