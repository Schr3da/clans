use std::slice::Iter;

use crate::entities::color::*;
use crate::entities::prelude::*;
use crate::theme::ThemeTypes;

pub trait IdProperty<T: 'static> {
    fn as_id(&self) -> &'static str;
}

pub trait LabelProperty<T: 'static> {
    fn as_label(&self) -> &'static str;
}

pub trait GlyphProperty<T: 'static> {
    fn as_glyph(&self) -> char;
}

pub trait ColorsProperty<T: 'static> {
    fn as_colors(&self, current_theme: ThemeTypes) -> (Color, Color);
}

pub trait DescriptionProperty<T>
where
    T: LabelProperty<T> + 'static,
{
    fn as_description(&self) -> Description;
}

pub trait RenderProperty<T>
where
    T: Send + Sync + 'static,
{
    fn as_renderable(self) -> Renderable<T>;
    fn as_renderable_with_prefix(self, prefix: String) -> Renderable<T>;
}

pub trait FrameProperty<T> {
    fn as_frame(&self) -> Frame;

    fn as_frame_for_position(&self, x: i32, y: i32) -> Frame;
}

pub trait FieldOfViewProperty<T> {
    fn as_field_of_view(&self) -> FieldOfView;
}

pub trait SpawnProperty<T: 'static>
where
    T: LabelProperty<T>,
{
    fn as_spawn_time(&self) -> Option<SpawnTime>;
    fn as_spawn_entity(&self) -> Option<T>;
    fn as_spawn_resource(&self) -> Option<ResourceTypes>;
}

pub trait TimeProperty<T> {
    fn as_build_time(&self) -> BuildTime;
}

pub trait SelectableProperty {
    fn is_selected(&self) -> bool;
}

pub trait WalkableProperty {
    fn move_forward(&mut self);
    fn did_reach_end(&self) -> bool;
    fn current_step(&self) -> Option<usize>;
}

pub trait CostsProperty {
    fn as_costs(&self) -> (i32, i32);
}

pub trait Printable {
    fn as_printable(&self) -> String;
}

pub trait EnumIterator<T: 'static>
where
    T: IdProperty<T> + LabelProperty<T>,
{
    fn iter() -> Iter<'static, T>;

    fn find(to_match: &'static str) -> Option<&T> {
        for item in Self::iter() {
            let id = item.as_id();
            let value = item.as_label();

            if id == to_match || value == to_match {
                return Option::Some(&item);
            }
        }

        Option::None
    }
}
