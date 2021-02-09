use std::slice::Iter;

use crate::entities::prelude::*;
use crate::entities::traits::properties::*;
use crate::theme::*;

use crate::theme::ThemeTypes;
use crate::utils::id::create_random_id;

#[derive(Debug, Copy, Clone)]
pub enum UnitIds {
    Tank,
}

static ITEMS: [UnitIds; 1] = [UnitIds::Tank];

impl IdProperty<UnitIds> for UnitIds {
    fn as_id(&self) -> &'static str {
        match self {
            UnitIds::Tank => "unit-tank",
        }
    }
}

impl LabelProperty<UnitIds> for UnitIds {
    fn as_label(&self) -> &'static str {
        match self {
            UnitIds::Tank => "Tank",
        }
    }
}

impl GlyphProperty<UnitIds> for UnitIds {
    fn as_glyph(&self) -> char {
        match self {
            UnitIds::Tank => 't',
        }
    }
}

impl ColorsProperty<UnitIds> for UnitIds {
    fn as_colors(&self, current_theme: ThemeTypes) -> (Color, Color) {
        let (background, foreground) = match self {
            UnitIds::Tank => (color_11(current_theme), color_10(current_theme)),
        };

        (background, foreground)
    }
}

impl FrameProperty<UnitIds> for UnitIds {
    fn as_frame(&self) -> Frame {
        Frame::new(0, 0, 1, 1)
    }

    fn as_frame_for_position(&self, x: i32, y: i32) -> Frame {
        Frame::new(x, y, 1, 1)
    }
}

impl FieldOfViewProperty<UnitIds> for UnitIds {
    fn as_field_of_view(&self) -> FieldOfView {
        match self {
            UnitIds::Tank => FieldOfView::new(2),
        }
    }
}

impl RenderProperty<UnitIds> for UnitIds {
    fn as_renderable(self) -> Renderable<UnitIds> {
        self.as_renderable_with_prefix("unit".to_owned())
    }

    fn as_renderable_with_prefix(self, prefix: String) -> Renderable<UnitIds> {
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

impl EnumIterator<UnitIds> for UnitIds {
    fn iter() -> Iter<'static, UnitIds> {
        ITEMS.iter()
    }
}
