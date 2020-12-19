use specs::prelude::*;

use common_core::prelude::*;

use crate::entities::prelude::*;
use crate::state::State;

pub fn toggle_theme(state: &State) {
    let ecs = state.ecs.borrow();

    let mut config = ecs.fetch_mut::<Config>();
    config.toggle_theme();
}

pub struct ThemeSystem {}

impl Default for ThemeSystem {
    fn default() -> Self {
        ThemeSystem {}
    }
}

impl<'a> System<'a> for ThemeSystem {
    type SystemData = (
        WriteExpect<'a, Config>,
        WriteExpect<'a, Preview>,
        WriteExpect<'a, Selection>,
        WriteExpect<'a, RendererData>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut config, mut preview, mut selection, mut data) = data;

        if config.needs_update() == false {
            return;
        }

        config.force_update(false);

        let next = config.current_theme;

        preview.change_theme(next);
        selection.change_theme(next);
        data.change_theme(next);
    }
}
