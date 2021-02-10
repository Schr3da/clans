use bracket_geometry::prelude::{DistanceAlg, Point};
use bracket_pathfinding::prelude::*;

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

pub fn calculate_navigation<T>(map: &T, from: &Frame, to: &Frame) -> NavigationPath
where
    T: Algorithm2D,
{
    let start = Point::constant(from.x, from.y);
    let end = Point::constant(to.x, to.y);

    a_star_search(map.point2d_to_index(start), map.point2d_to_index(end), map)
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

impl BaseMap for Map {
    fn is_opaque(&self, index: usize) -> bool {
        let tile = &self.tiles[index];
        tile.current_type == TileTypes::Block
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) {
            exits.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) {
            exits.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) {
            exits.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) {
            exits.push((idx, 1.0))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let p1 = self.index_to_point2d(idx1);
        let p2 = self.index_to_point2d(idx2);

        DistanceAlg::Pythagoras.distance2d(Point::new(p1.x, p1.y), Point::new(p2.x, p2.y))
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.columns, self.rows)
    }
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

impl Map {

    pub fn reference(&self) -> &Self {
        self
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) == false {
            return None;
        }

        let index = self.point2d_to_index(destination);
        if self.tiles[index].current_type != TileTypes::Ground {
            return None;
        }

        Some(index)
    }
}
