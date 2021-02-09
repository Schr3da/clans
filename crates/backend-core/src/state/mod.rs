use specs::prelude::*;

use common_core::prelude::*;

use crate::entities::map;
use crate::systems::events::dispatch_event;
use crate::systems::prelude::*;

pub struct State {
    pub ecs: World,
    pub subscription_timer_override: bool,
    pub subscription_timer: IntervalTimer,
    subscription_callback: Option<EventCallback>,
}

impl State {
    pub fn new(columns: usize, rows: usize) -> Self {
        let mut ecs = World::new();
        ecs.register::<BuildTime>();
        ecs.register::<Description>();
        ecs.register::<FieldOfView>();
        ecs.register::<Frame>();
        ecs.register::<Renderable<BuildingIds>>();
        ecs.register::<Renderable<UnitIds>>();
        ecs.register::<SpawnTime>();
        ecs.register::<IntervalTimer>();
        ecs.register::<Path>();

        let config = Config::new(columns, rows);
        ecs.insert(config);

        let map = map::Map::new(columns, rows);
        ecs.insert(map);

        let resource_manager = ResourceManager::new();
        ecs.insert(resource_manager);

        let path_builder = PathBuilder::new(Vec::new());
        ecs.insert(path_builder);

        State {
            ecs,
            subscription_callback: Option::None,
            subscription_timer: IntervalTimer::new(1),
            subscription_timer_override: false,
        }
    }

    pub fn run_systems(&mut self) {
        let mut time_system = CurrentTimeSystem::default();
        time_system.run_now(&self.ecs);

        let mut build_time_system = BuildTimeSystem::default();
        build_time_system.run_now(&self.ecs);

        let mut spawn_system = SpawnSystem::default();
        spawn_system.run_now(&self.ecs);

        let mut interval_timer_system = IntervalSystem::default();
        interval_timer_system.run_now(&self.ecs);

        let mut fov_system = FieldOfViewSystem::default();
        fov_system.run_now(&self.ecs);

        let mut unit_system = UnitSystem::default();
        unit_system.run_now(&self.ecs);

        handle_subscriptions(self);

        self.ecs.maintain();
    }

    pub fn subscribe(&mut self, subscription_callback: EventCallback) {
        self.subscription_callback = Some(subscription_callback);
    }

    pub fn send(&self, event: Event) {
        match &self.subscription_callback {
            Some(cb) => (cb.borrow_mut())(event),
            None => {}
        }
    }

    pub fn receive(&mut self, event: Event) {
        dispatch_event(self, event);
    }
}
