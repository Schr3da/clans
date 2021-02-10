use std::slice::Iter;

use specs::prelude::*;
use specs_derive::*;

use common_core::prelude::*;
use common_core::theme::traits::Themeable;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PathStates {
    OnStandby,
    OnPrepare,
    OnSetStartPoint,
    OnMove,
    OnSetEndPoint,
}

static ITEMS: [PathStates; 5] = [
    PathStates::OnStandby,
    PathStates::OnPrepare,
    PathStates::OnSetStartPoint,
    PathStates::OnMove,
    PathStates::OnSetEndPoint,
];

impl IdProperty<PathStates> for PathStates {
    fn as_id(&self) -> &'static str {
        match self {
            PathStates::OnStandby => "on-standby",
            PathStates::OnPrepare => "on-prepare",
            PathStates::OnSetStartPoint => "on-set-start-point",
            PathStates::OnMove => "move-path",
            PathStates::OnSetEndPoint => "on-set-end-point",
        }
    }
}

impl LabelProperty<PathStates> for PathStates {
    fn as_label(&self) -> &'static str {
        match self {
            PathStates::OnStandby => "",
            PathStates::OnPrepare => "Select start for path",
            PathStates::OnSetStartPoint => "Start point selected. Move to destination",
            PathStates::OnMove => "define destination",
            PathStates::OnSetEndPoint => "End point selected",
        }
    }
}

impl EnumIterator<PathStates> for PathStates {
    fn iter() -> Iter<'static, PathStates> {
        ITEMS.iter()
    }
}

#[derive(Clone, Component)]
pub struct PathRenderer {
    pub renderables: Vec<(Frame, Renderable<PathStates>)>,
    pub start: Frame,
    pub end: Frame,
    pub steps: Vec<usize>,
    pub is_colliding: bool,
    pub current_state: PathStates,
    needs_update: bool,
}

impl Themeable for PathRenderer {
    fn change_theme(&mut self, next: ThemeTypes) {
        for (_, renderable) in self.renderables.iter_mut() {
            renderable.foreground = color_05(next);
            renderable.background = color_06(next);
        }
    }
}

impl Updateable for PathRenderer {
    fn needs_update(&self) -> bool {
        return self.needs_update;
    }

    fn force_update(&mut self, value: bool) {
        self.needs_update = value;
    }
}

impl PathRenderer {
    pub fn new() -> Self {
        PathRenderer {
            renderables: Vec::new(),
            steps: Vec::new(),
            start: Frame::new(0, 0, 1, 1),
            end: Frame::new(0, 0, 1, 1),
            is_colliding: false,
            needs_update: true,
            current_state: PathStates::OnStandby,
        }
    }

    fn reset(&mut self) {
        self.renderables.clear();
        self.steps.clear();
        self.is_colliding = false;
        self.needs_update = true;
        self.current_state = PathStates::OnStandby;
    }

    pub fn prepare(&mut self) {
        if self.current_state != PathStates::OnStandby {
            return;
        }

        self.current_state = PathStates::OnSetStartPoint;
    }

    pub fn set_start(&mut self, x: i32, y: i32) {
        self.reset();
        self.start.x = x;
        self.start.y = y;
        self.current_state = PathStates::OnMove;
    }

    pub fn update(&mut self, x: i32, y: i32, steps: Vec<usize>) {
        if self.current_state != PathStates::OnMove {
            return;
        }

        self.steps = steps;
        self.end.x = x;
        self.end.y = y;
    }

    pub fn set_end(&mut self, x: i32, y: i32) {
        self.current_state = PathStates::OnStandby;
        self.end.x = x;
        self.end.y = y;
    }

    pub fn handle_collision(&mut self, is_colliding: bool) {
        self.is_colliding = is_colliding;

        for (_, renderable ) in self.renderables.iter_mut() {
            let foreground = match is_colliding {
                true => color_03(ThemeTypes::Dark),
                false => color_05(ThemeTypes::Dark),
            };
            renderable.change_foreground(foreground);
        }

        self.force_update(false);
    }

    pub fn create_renderables(&mut self, columns: usize) {
        self.renderables = self.steps.clone().into_iter().map(|index| {
            let (x, y) = map_index_to_coordinates(index, columns); 
            let frame = Frame::new(x, y, 1, 1);

            let prefix = "path-".to_owned();
            let renderable = Renderable {
                id: create_random_id(prefix),
                glyph: 'x',
                foreground: color_08(ThemeTypes::Dark),
                background: color_10(ThemeTypes::Dark),
                current_type: PathStates::OnMove 
            };
            
            (frame, renderable)
        }).collect();
    }
}
