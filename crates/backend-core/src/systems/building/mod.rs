use specs::prelude::*;

use common_core::prelude::*;

use crate::algorithms::prelude::*;
use crate::entities::map::Map;
use crate::state::State;

fn calculate_path(state: &mut State, frame: &Frame) -> Path {
    let map = state.ecs.fetch::<Map>();
    let target = Frame::new((map.columns / 2) as i32, (map.rows / 2) as i32, 1, 1);
    let navigation = calculate_navigation_to_target(&map, &frame, &target);

    Path::new(navigation.steps)
}

fn can_build(state: &mut State, id: &BuildingIds, frame: &Frame) -> bool {
    let resource_manager = state.ecs.fetch::<ResourceManager>();

    if resource_manager.can_buy(id) == false {
        return false;
    }

    let map = state.ecs.fetch_mut::<Map>();
    let is_colliding = has_tile_collision_for_frame(&frame, &map.tiles, map.columns, map.rows);

    if is_colliding == true {
        return false;
    }

    return true;
}

fn check_and_buy_if_possible(state: &mut State, id: &BuildingIds, frame: &Frame) -> bool {
    if can_build(state, id, frame) == false {
        return false;
    }

    let mut map = state.ecs.fetch_mut::<Map>();
    map.add_blocks_for_building(*frame);

    let mut resource_manager = state.ecs.fetch_mut::<ResourceManager>();
    resource_manager.buy(id);

    return true;
}

pub fn new_building(state: &mut State, id: &BuildingIds, frame: Frame) {
    if check_and_buy_if_possible(state, id, &frame) == false {
        return;
    }

    {
        let mut config = state.ecs.fetch_mut::<Config>();
        config.force_update(true);
    }

    let path = calculate_path(state, &frame);

    let prefix = "building-".to_owned();

    let mut entity = state
        .ecs
        .create_entity()
        .with(frame)
        .with(path)
        .with(id.as_renderable_with_prefix(prefix))
        .with(id.as_description())
        .with(id.as_build_time())
        .with(id.as_field_of_view());

    if let Some(spawner) = id.as_spawn_time() {
        entity = entity.with(spawner);
    }

    entity.build();
}

pub fn remove_building(state: &mut State, x: i32, y: i32) {
    let mut matched_entity = Option::None;

    {
        let data: (
            Entities,
            WriteExpect<Map>,
            ReadStorage<Frame>,
            ReadStorage<FieldOfView>,
            ReadStorage<Renderable<BuildingIds>>,
        ) = state.ecs.system_data();

        let (entities, mut map, frames, fovs, buildings) = data;

        for (entity, frame, fov, _) in (&entities, &frames, &fovs, &buildings).join() {
            if frame.contains_coordinates(x, y) == false {
                continue;
            }

            map.remove_blocks_for_building(frame);
            update_visble_tiles_for_fov_with_values(&mut map, &fov, false);

            matched_entity = Some(entity);
            break;
        }
    }

    if matched_entity == None {
        return;
    }

    match state.ecs.delete_entity(matched_entity.unwrap()) {
        Ok(_) => println!("removed building successful"),
        _ => println!("failed to remove building"),
    }
}
