use bracket_algorithm_traits::prelude::*;
use bracket_geometry::prelude::{DistanceAlg, Point};

use common_core::prelude::*;

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
    pub fn new(columns: usize, rows: usize) -> Self {
        let mut map = Map {
            length: columns * rows,
            columns,
            rows,
            tiles: Vec::new(),
            visited_tiles: Vec::new(),
            visible_tiles: Vec::new(),
            needs_update: true,
        };

        map.generate_tiles();
        map
    }

    pub fn has_collision(&self, x: usize, y: usize) -> bool {
        if coordinates_out_of_map_range(x, y, self.columns, self.rows) {
            return true;
        }

        let index = coordinates_to_map_index(x, y, self.columns);
        self.tiles[index].current_type != TileTypes::Ground
    }

    pub fn add_blocks_for_building(&mut self, frame: Frame) {
        for x in frame.x..=frame.x + frame.width {
            for y in frame.y..=frame.y + frame.height {
                if self.has_collision(x as usize, y as usize) == true {
                    return;
                }
            }
        }

        for x in frame.x..=frame.x + frame.width {
            for y in frame.y..=frame.y + frame.height {
                let index = coordinates_to_map_index(x as usize, y as usize, self.columns);
                self.tiles[index].current_type = TileTypes::Building;
            }
        }

        self.force_update(true);
    }

    pub fn remove_blocks_for_building(&mut self, frame: &Frame) {
        for x in frame.x..=frame.x + frame.width {
            for y in frame.y..=frame.y + frame.height {
                let index = coordinates_to_map_index(x as usize, y as usize, self.columns);
                
                if self.tiles[index].current_type != TileTypes::Building {
                    continue;
                }

                self.tiles[index].current_type = TileTypes::Ground;
                self.force_update(true);
            }
        }
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

    fn id_prefix(x: usize, y: usize) -> String {
        let mut result = "tile".to_owned();
        result.push_str("-");
        result.push_str(x.to_string().as_str());
        result.push_str("-");
        result.push_str(y.to_string().as_str());
        result.push_str("-");
        result
    }

    fn generate_tiles(&mut self) {
        let columns = self.columns;
        let rows = self.rows;

        let mut tiles = vec![TileTypes::Ground.as_renderable(); columns * rows];
        
        for x in 0..columns {
            for y in 0..rows {
                let prefix = Self::id_prefix(x, y);
                let index = coordinates_to_map_index(x, y, columns);
                tiles[index] = TileTypes::Ground.as_renderable_with_prefix(prefix);
            }
        }

        for x in 0..columns {
            let mut prefix = Self::id_prefix(x, 0);
            let top = coordinates_to_map_index(x, 0, columns);
            tiles[top] = TileTypes::Block.as_renderable_with_prefix(prefix.clone());

            prefix = Self::id_prefix(x, rows - 1);
            let bottom = coordinates_to_map_index(x, rows - 1, columns);
            tiles[bottom] = TileTypes::Block.as_renderable_with_prefix(prefix.clone());
        }

        for y in 0..rows {
            let mut prefix = Self::id_prefix(0, y);
            let left = coordinates_to_map_index(0, y, columns);
            tiles[left] = TileTypes::Block.as_renderable_with_prefix(prefix.clone());

            prefix = Self::id_prefix(columns - 1, y);
            let right = coordinates_to_map_index(columns - 1, y, columns);
            tiles[right] = TileTypes::Block.as_renderable_with_prefix(prefix.clone());
        }

        self.tiles = tiles;
        self.visited_tiles = vec![true; self.columns * self.rows];
        self.visible_tiles = vec![false; self.columns * self.rows];
    }
}
