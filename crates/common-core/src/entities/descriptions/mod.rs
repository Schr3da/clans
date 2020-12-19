use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Description {
    pub content: &'static str,
}
