use specs::prelude::*;

use common_core::prelude::*;

pub struct SpawnSystem {}

impl Default for SpawnSystem {
    fn default() -> Self {
        SpawnSystem {}
    }
}

impl<'a> System<'a> for SpawnSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Config>,
        WriteExpect<'a, ResourceManager>,
        ReadStorage<'a, BuildTime>,
        ReadStorage<'a, Renderable<BuildingIds>>,
        ReadStorage<'a, Path>,
        WriteStorage<'a, SpawnTime>,
        WriteStorage<'a, Frame>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            config,
            mut resource_manager,
            build_times,
            buildings_renderables,
            paths,
            mut spawn_times,
            mut frames,
            lazy,
        ) = data;

        for (build_time, spawn_time, buildings_renderable, path, frame) in (
            &build_times,
            &mut spawn_times,
            &buildings_renderables,
            &paths,
            &mut frames,
        )
            .join()
        {
            if build_time.is_completed == false {
                continue;
            }

            spawn_time.update(config.current_time);

            if spawn_time.should_spawn == false {
                continue;
            }

            if let Some(id) = buildings_renderable.current_type.as_spawn_resource() {
                return resource_manager.income(id);
            }

            if let Some(id) = buildings_renderable.current_type.as_spawn_entity() {
                return new_unit(id, path, frame, &entities, &lazy);
            }
        }

        fn new_unit(
            id: UnitIds,
            path: &Path,
            building_frame: &Frame,
            entities: &specs::Read<specs::world::EntitiesRes>,
            lazy: &specs::Read<specs::LazyUpdate>,
        ) {
            let prefix = "unit-".to_owned();

            let frame = id.as_frame_for_position(building_frame.x, building_frame.y);
            let entity = lazy.create_entity(&entities);

            entity
                .with(path.clone())
                .with(frame)
                .with(id.as_renderable_with_prefix(prefix))
                .with(IntervalTimer::new(1))
                .with(id.as_field_of_view())
                .build();
        }
    }
}
