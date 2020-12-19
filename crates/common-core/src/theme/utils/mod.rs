extern crate hsluv;

use hsluv::*;

use crate::entities::color::Color;

pub fn hsl_to_rb_float(h: f64, s: f64, l: f64) -> Color {
    let (r, g, b) = hsluv_to_rgb((h, s, l));
    Color::new(r as f32, g as f32, b as f32)
}
