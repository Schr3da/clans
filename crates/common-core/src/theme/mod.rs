mod dark;
mod light;
mod utils;

pub mod traits;

use crate::entities::color::Color;

#[derive(Copy, Clone, Debug)]
pub enum ThemeTypes {
    Dark,
    Light,
}

pub fn color_01(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_01(),
        ThemeTypes::Light => light::color_01(),
    }
}

pub fn color_02(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_02(),
        ThemeTypes::Light => light::color_02(),
    }
}

pub fn color_03(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_03(),
        ThemeTypes::Light => light::color_03(),
    }
}

pub fn color_04(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_04(),
        ThemeTypes::Light => light::color_04(),
    }
}

pub fn color_05(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_05(),
        ThemeTypes::Light => light::color_05(),
    }
}

pub fn color_06(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_06(),
        ThemeTypes::Light => light::color_06(),
    }
}

pub fn color_07(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_07(),
        ThemeTypes::Light => light::color_07(),
    }
}

pub fn color_08(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_08(),
        ThemeTypes::Light => light::color_08(),
    }
}

pub fn color_09(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_09(),
        ThemeTypes::Light => light::color_09(),
    }
}

pub fn color_10(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_10(),
        ThemeTypes::Light => light::color_10(),
    }
}

pub fn color_11(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_11(),
        ThemeTypes::Light => light::color_11(),
    }
}

pub fn color_12(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_12(),
        ThemeTypes::Light => light::color_12(),
    }
}

pub fn color_13(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_13(),
        ThemeTypes::Light => light::color_13(),
    }
}

pub fn color_14(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_14(),
        ThemeTypes::Light => light::color_14(),
    }
}

pub fn color_15(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_15(),
        ThemeTypes::Light => light::color_15(),
    }
}

pub fn color_16(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_16(),
        ThemeTypes::Light => light::color_16(),
    }
}

pub fn color_17(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_17(),
        ThemeTypes::Light => light::color_17(),
    }
}

pub fn color_18(current_type: ThemeTypes) -> Color {
    match current_type {
        ThemeTypes::Dark => dark::color_18(),
        ThemeTypes::Light => light::color_18(),
    }
}

pub mod prelude {
    pub use super::traits::*;
    pub use super::*;
}
