use bracket_pathfinding::prelude::*;

use common_core::entities::frame::Frame;

use crate::entities::map::Map;

fn calculate_path(map: &Map, from: &Frame, to: &Frame) -> NavigationPath {
    let start = Point::constant(from.x, from.y);
    let end = Point::constant(to.x, to.y);

    a_star_search(map.point2d_to_index(start), map.point2d_to_index(end), map)
}

pub fn calculate_navigation_to_target(map: &Map, start: &Frame, target: &Frame) -> NavigationPath {
    let navigation = calculate_path(map, start, target);
    navigation
}
