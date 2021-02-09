use specs::prelude::*;
use specs_derive::*;

use std::marker::{Send, Sync};

use super::color::Color;

use crate::theme::traits::Themeable;
use crate::theme::ThemeTypes;

use super::traits::properties::*;

#[derive(Clone, Component)]
pub struct Renderable<T>
where
    T: Send + Sync + 'static,
{
    pub id: String,
    pub glyph: char,
    pub foreground: Color,
    pub background: Color,
    pub current_type: T,
}

impl<T> Themeable for Renderable<T>
where
    T: ColorsProperty<T> + Send + Sync + 'static,
{
    fn change_theme(&mut self, next: ThemeTypes) {
        let (background, foreground) = self.current_type.as_colors(next);
        self.background = background;
        self.foreground = foreground;
    }
}

impl<T> Renderable<T>
where
    T: Send + Sync + 'static,
{
    pub fn change_foreground(&mut self, foreground: Color) {
        self.foreground = foreground;
    }
}

impl<T> Printable for Renderable<T>
where
    T: ColorsProperty<T> + Send + Sync + 'static,
{
    fn as_printable(&self) -> String {
        let mut formatter = String::new();
        formatter.push_str("id ");
        formatter.push_str(self.id.to_string().as_str());
        formatter
    }
}
