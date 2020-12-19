use specs::prelude::*;
use specs_derive::*;

use common_core::prelude::*;
use common_core::theme::traits::Themeable;

#[derive(Clone, Component)]
pub struct Preview {
    pub renderable: Option<Renderable<BuildingIds>>,
    pub frame: Frame,
    pub is_colliding: bool,
    needs_update: bool,
}

impl Themeable for Preview {
    fn change_theme(&mut self, next: ThemeTypes) {
        match &mut self.renderable {
            None => {}
            Some(r) => {
                r.foreground = color_05(next);
                r.background = color_06(next);
            }
        }
    }
}

impl Updateable for Preview {
    fn needs_update(&self) -> bool {
        return self.needs_update;
    }

    fn force_update(&mut self, value: bool) {
        self.needs_update = value;
    }
}

impl Preview {
    pub fn new() -> Self {
        Preview {
            renderable: Option::None,
            frame: Frame::zero(),
            is_colliding: false,
            needs_update: false,
        }
    }

    pub fn handle_collision(&mut self, is_colliding: bool) {
        self.is_colliding = is_colliding;

        match &mut self.renderable {
            None => {}
            Some(renderable) => {
                let foreground = match is_colliding {
                    true => color_03(ThemeTypes::Dark),
                    false => color_05(ThemeTypes::Dark),
                };
                renderable.change_foreground(foreground);
            }
        }
    }

    pub fn show(&mut self, id: BuildingIds, current_theme: ThemeTypes) {
        self.frame = id.as_frame();

        self.renderable = Option::Some(Renderable {
            id: create_random_id("preview".to_owned()),
            glyph: id.as_glyph(),
            foreground: color_05(current_theme),
            background: color_11(current_theme),
            current_type: id,
        });

        self.force_update(true);
    }

    pub fn update_position(&mut self, x: i32, y: i32) {
        self.frame.x = x;
        self.frame.y = y;
        self.force_update(true);
    }

    pub fn hide(&mut self) {
        self.renderable = Option::None;
        self.frame = Frame::zero();
        self.force_update(true);
    }
}
