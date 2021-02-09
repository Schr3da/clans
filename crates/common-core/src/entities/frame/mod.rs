use specs::prelude::*;
use specs_derive::*;

use super::point::Point;
use super::traits::properties::Printable;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Component)]
pub struct Frame {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Printable for Frame {
    fn as_printable(&self) -> String {
        let mut formatter = String::new();
        formatter.push_str("x:");
        formatter.push_str(self.x.to_string().as_str());
        formatter.push_str(" y:");
        formatter.push_str(self.y.to_string().as_str());
        formatter.push_str(" width:");
        formatter.push_str(self.width.to_string().as_str());
        formatter.push_str(" height:");
        formatter.push_str(self.height.to_string().as_str());
        formatter
    }
}

impl Frame {
    pub fn zero() -> Self {
        Self::new(0, 0, 0, 0)
    }

    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Frame {
            x,
            y,
            width,
            height,
        }
    }

    pub fn position_to_point(self) -> Point {
        Point::new(self.x, self.y)
    }

    pub fn contains_coordinates(&self, x: i32, y: i32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    pub fn set_coordinates(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}
