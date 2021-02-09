use specs::prelude::*;
use specs_derive::*;

use common_core::prelude::*;
use common_core::theme::traits::Themeable;

#[derive(Clone, Component)]
pub struct Selection {
    pub renderable: Option<Renderable<BuildingIds>>,
    pub frame: Frame,
    needs_update: bool,
}

impl Themeable for Selection {
    fn change_theme(&mut self, next: ThemeTypes) {
        match &mut self.renderable {
            None => {}
            Some(r) => {
                r.foreground = color_12(next);
                r.background = color_06(next);
            }
        }
    }
}

impl SelectableProperty for Selection {
    fn is_selected(&self) -> bool {
        match &self.renderable {
            Some(_) => true,
            None => false,
        }
    }
}

impl Updateable for Selection {
    fn needs_update(&self) -> bool {
        return self.needs_update;
    }

    fn force_update(&mut self, value: bool) {
        self.needs_update = value;
    }
}

impl Selection {
    pub fn new() -> Self {
        Selection {
            renderable: Option::None,
            frame: Frame::zero(),
            needs_update: false,
        }
    }

    pub fn show(&mut self, frame: Frame, renderable: Renderable<BuildingIds>) {
        self.frame = frame;
        self.renderable = Option::Some(renderable);
        self.force_update(true);
    }

    pub fn hide(&mut self) {
        self.renderable = Option::None;
        self.frame = Frame::zero();
        self.force_update(true);
    }
}
