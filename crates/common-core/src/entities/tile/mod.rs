use crate::prelude::*;
use crate::theme::ThemeTypes;
use crate::theme::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TileTypes {
    Ground,
    Block,
    Building,
}

impl GlyphProperty<TileTypes> for TileTypes {
    fn as_glyph(&self) -> char {
        match self {
            TileTypes::Block => '#',
            TileTypes::Ground => '.',
            TileTypes::Building => '=',
        }
    }
}

impl ColorsProperty<TileTypes> for TileTypes {
    fn as_colors(&self, current_theme: ThemeTypes) -> (Color, Color) {
        let (background, foreground) = match self {
            TileTypes::Block => (color_11(current_theme), color_13(current_theme)),
            TileTypes::Ground => (color_11(current_theme), color_08(current_theme)),
            TileTypes::Building => (color_11(current_theme), color_11(current_theme)),
        };
        (background, foreground)
    }
}

impl RenderProperty<TileTypes> for TileTypes {
    fn as_renderable(self) -> Renderable<TileTypes> {
        self.as_renderable_with_prefix("tile".to_owned())
    }

    fn as_renderable_with_prefix(self, prefix: String) -> Renderable<TileTypes> {
        let (background, foreground) = self.as_colors(ThemeTypes::Dark);

        Renderable {
            id: create_random_id(prefix),
            current_type: self,
            foreground,
            background,
            glyph: self.as_glyph(),
        }
    }
}
