use bracket_algorithm_traits::prelude::*;

use common_core::prelude::*;

use crate::entities::map::Map;

// Transform represents a sector to shadow cast.
struct Transform {
    xx: i32,
    xy: i32,
    yx: i32,
    yy: i32,
}

// Viewer holds information about the location and viewing radius of the character we're
// calculating the FOV for.
struct Viewer {
    x: i32,
    y: i32,
    radius: i32,
}

// Calculates the field of view on the map using shadow casting.  See:
//
// * http://www.roguebasin.com/index.php?title=FOV_using_recursive_shadowcasting
// * http://www.roguebasin.com/index.php?title=C%2B%2B_shadowcasting_implementation
//
// Returns the vector of points visible to the viewer located at (x, y) with the given viewing
// radius.
pub fn calculate(x: i32, y: i32, radius: i32, map: &Map) -> Vec<Point> {
    let transforms: Vec<Transform> = vec![
        Transform {
            xx: 1,
            xy: 0,
            yx: 0,
            yy: 1,
        },
        Transform {
            xx: 0,
            xy: 1,
            yx: 1,
            yy: 0,
        },
        Transform {
            xx: 0,
            xy: -1,
            yx: 1,
            yy: 0,
        },
        Transform {
            xx: -1,
            xy: 0,
            yx: 0,
            yy: 1,
        },
        Transform {
            xx: -1,
            xy: 0,
            yx: 0,
            yy: -1,
        },
        Transform {
            xx: 0,
            xy: -1,
            yx: -1,
            yy: 0,
        },
        Transform {
            xx: 0,
            xy: 1,
            yx: -1,
            yy: 0,
        },
        Transform {
            xx: 1,
            xy: 0,
            yx: 0,
            yy: -1,
        },
    ];

    let viewer = Viewer { x, y, radius };

    let mut visible: Vec<Point> = Vec::new();

    // The viewer's location is always visible
    visible.push(Point { x, y });

    for transform in transforms {
        cast_light(&mut visible, map, &viewer, 1, 1.0, 0.0, &transform);
    }

    visible
}

fn cast_light(
    visible: &mut Vec<Point>,
    map: &Map,
    viewer: &Viewer,
    row: i32,
    start_slope: f32,
    end_slope: f32,
    transform: &Transform,
) {
    if start_slope < end_slope {
        return;
    }

    let radius_sq = viewer.radius * viewer.radius;

    let mut start_slope = start_slope;
    let mut next_start_slope = start_slope;

    let columns = map.columns;
    let rows = map.rows;

    for i in row..=viewer.radius {
        let mut blocked = false;
        let dy = -i;

        for dx in -i..=0 {
            let left_slope = (dx as f32 - 0.5) / (dy as f32 + 0.5);
            let right_slope = (dx as f32 + 0.5) / (dy as f32 - 0.5);

            if start_slope < right_slope {
                continue;
            }

            if end_slope > left_slope {
                break;
            }

            let sax = dx * transform.xx + dy * transform.xy;
            let say = dx * transform.yx + dy * transform.yy;
            if (sax < 0 && sax.abs() > viewer.x) || (say < 0 && say.abs() > viewer.y) {
                continue;
            }

            let ax = (viewer.x + sax) as usize;
            let ay = (viewer.y + say) as usize;
            if ax >= columns || ay >= rows {
                continue;
            }

            if dx * dx + dy * dy < radius_sq {
                visible.push(Point {
                    x: ax as i32,
                    y: ay as i32,
                });
            }

            let tile_index = coordinates_to_map_index(ax, ay, map.columns);

            if blocked {
                if map.is_opaque(tile_index) {
                    next_start_slope = right_slope;
                    continue;
                }

                blocked = false;
                start_slope = next_start_slope;
                continue;
            }

            if map.is_opaque(tile_index) {
                blocked = true;
                next_start_slope = right_slope;
                cast_light(
                    visible,
                    map,
                    viewer,
                    row + 1,
                    start_slope,
                    left_slope,
                    transform,
                );
            }
        }

        if blocked {
            break;
        }
    }
}

pub fn update_visble_tiles_for_fov_with_values(map: &mut Map, fov: &FieldOfView, value: bool) {
    for tile in fov.tiles.iter() {
        let index = coordinates_to_map_index(tile.x as usize, tile.y as usize, map.columns);
                            
        map.visited_tiles[index] = true;
        map.visible_tiles[index] = value;
        map.force_update(true);
    }
}
