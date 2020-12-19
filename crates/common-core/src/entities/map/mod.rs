use super::prelude::*;
use crate::theme::prelude::*;

pub fn coordinates_to_map_index(x: usize, y: usize, columns: usize) -> usize {
    y * columns + x
}

pub fn coordinates_out_of_map_range(x: usize, y: usize, columns: usize, rows: usize) -> bool {
    x > columns || y > rows
}

pub fn map_index_to_coordinates(index: usize, columns: usize) -> (i32, i32) {
    let column = index % columns;
    let row = index / columns;

    (column as i32, row as i32)
}

pub fn has_tile_collision_for_coordinates(
    x: usize,
    y: usize,
    tiles: &Vec<Renderable<TileTypes>>,
    columns: usize,
    rows: usize,
) -> bool {
    if coordinates_out_of_map_range(x, y, columns, rows) {
        return true;
    }

    let index = coordinates_to_map_index(x, y, columns);
    tiles[index].current_type != TileTypes::Ground
}

pub fn has_tile_collision_for_frame(
    frame: &Frame,
    tiles: &Vec<Renderable<TileTypes>>,
    columns: usize,
    rows: usize,
) -> bool {
    let max_x = frame.x + frame.width;
    let max_y = frame.y + frame.height;

    for x in frame.x..=max_x {
        for y in frame.y..=max_y {
            let is_colliding =
                has_tile_collision_for_coordinates(x as usize, y as usize, tiles, columns, rows);

            if is_colliding == true {
                return true;
            }
        }
    }

    false
}

#[derive(Clone)]
pub struct Map {
    pub length: usize,
    pub columns: usize,
    pub rows: usize,
    pub tiles: Vec<Renderable<TileTypes>>,
    pub visited_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub needs_update: bool,
}

impl Default for Map {
    fn default() -> Self {
        Map {
            length: 0,
            columns: 0,
            rows: 0,
            tiles: Vec::new(),
            visible_tiles: Vec::new(),
            visited_tiles: Vec::new(),
            needs_update: false,
        }
    }
}

impl Updateable for Map {
    fn needs_update(&self) -> bool {
        self.needs_update
    }

    fn force_update(&mut self, should_update: bool) {
        self.needs_update = should_update;
    }
}

impl Themeable for Map {
    fn change_theme(&mut self, next: ThemeTypes) {
        for tile in self.tiles.iter_mut() {
            let (background, foreground) = tile.current_type.as_colors(next);
            tile.background = background;
            tile.foreground = foreground;
        }
    }
}
