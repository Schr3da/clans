use specs::prelude::*;

use common_core::prelude::*;

use crate::entities::map;
use crate::state::State;

fn map_to_dto(state: &State) -> Option<common_core::entities::map::Map> {
    let mut map = state.ecs.fetch_mut::<map::Map>();

    if map.needs_update() == false {
        return None        
    }

    let data = common_core::entities::map::Map {
        length: map.length,
        columns: map.columns,
        rows: map.rows,
        tiles: map.tiles.clone(),
        visited_tiles: map.visited_tiles.clone(),
        visible_tiles: map.visible_tiles.clone(),
        needs_update: map.needs_update(),
    };

    map.force_update(false);

    Some(data)
}

fn buildings_to_dto(state: &State) -> Option<BuildingDtos> {
    let frames = state.ecs.read_storage::<Frame>();
    let times = state.ecs.read_storage::<BuildTime>();
    let buildings = state.ecs.read_storage::<Renderable<BuildingIds>>();
    let map = state.ecs.fetch::<map::Map>();

    let mut dtos = Vec::<BuildingDto>::new();

    for (frame, time, building) in (&frames, &times, &buildings).join() {
        let index = coordinates_to_map_index(frame.x as usize, frame.y as usize, map.columns);
        if map.visible_tiles[index] == false {
            continue;
        }

        let dto = (frame.clone(), time.clone(), building.clone());
        dtos.push(dto);
    }

    Some(dtos)
}

fn units_to_dto(state: &State) -> Option<UnitDtos> {
    let frames = state.ecs.read_storage::<Frame>();
    let units = state.ecs.read_storage::<Renderable<UnitIds>>();

    let mut dtos = Vec::<UnitDto>::new();

    for (frame, unit) in (&frames, &units).join() {
        let dto = (frame.clone(), unit.clone());
        dtos.push(dto);
    }

    Some(dtos)
}

fn resources_to_dto(state: &State) -> Option<ResourcesDto> {
    let mut resource_manager = state.ecs.fetch_mut::<ResourceManager>();

    if resource_manager.needs_update() == false {
        return None        
    }

    resource_manager.force_update(false);
    
    Some(ResourcesDto {
        food: resource_manager.food,
        materials: resource_manager.materials,
    })
}

fn send_render_update(state: &State) {
    let map = map_to_dto(state);
    let buildings = buildings_to_dto(state);
    let units = units_to_dto(state);
    let resources = resources_to_dto(state);

    state.send(common_core::events::backend::render::on_send_render_update(
        map, buildings, units, resources
    ));
}

pub fn handle_subscriptions(state: &mut State) {
    let config = state.ecs.fetch::<Config>();
    state.subscription_timer.update(config.current_time);

    if state.subscription_timer_override == true {
        state.subscription_timer_override = false;
        return send_render_update(state);
    }

    if state.subscription_timer.needs_update() == false {
        return send_render_update(state);
    }
}
