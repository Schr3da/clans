use std::clone::Clone;
use std::marker::Copy;

use crate::theme::ThemeTypes;

use super::traits::render::Updateable;

use crate::utils::time::current_timestamp;

#[derive(Copy, Clone, Debug)]
pub struct Config {
    pub columns: usize,
    pub rows: usize,
    pub current_theme: ThemeTypes,
    pub current_time: i64,
    needs_update: bool,
}

impl Updateable for Config {
    fn needs_update(&self) -> bool {
        return self.needs_update;
    }

    fn force_update(&mut self, value: bool) {
        self.needs_update = value;
    }
}

impl Config {
    pub fn new(columns: usize, rows: usize) -> Self {
        Config {
            columns,
            rows,
            current_theme: ThemeTypes::Dark,
            current_time: current_timestamp(),
            needs_update: true,
        }
    }

    pub fn bounds(&self) -> (usize, usize) {
        (self.columns, self.rows)
    }

    pub fn toggle_theme(&mut self) {
        self.current_theme = match &self.current_theme {
            ThemeTypes::Dark => ThemeTypes::Light,
            ThemeTypes::Light => ThemeTypes::Dark,
        };
        self.force_update(true);
    }
}
