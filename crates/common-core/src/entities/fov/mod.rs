use specs::prelude::*;
use specs_derive::*;

use crate::entities::traits::render::Updateable;

use super::point::Point;

#[derive(Component)]
pub struct FieldOfView {
    pub tiles: Vec<Point>,
    pub range: i32,
    needs_update: bool,
}

impl Updateable for FieldOfView {
    fn needs_update(&self) -> bool {
        return self.needs_update;
    }

    fn force_update(&mut self, value: bool) {
        self.needs_update = value;
    }
}

impl FieldOfView {
    pub fn new(range: i32) -> Self {
        FieldOfView {
            range,
            tiles: Vec::new(),
            needs_update: range != 0,
        }
    }
}
