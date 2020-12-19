use specs::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use common_core::events;
use common_core::prelude::*;

use communication_core::prelude::*;

use crate::entities::prelude::*;
use crate::events::Event;
use crate::systems::events::backend::dispatch_backend_event;
use crate::systems::events::frontend::dispatch_frontend_event;

pub type RCStateEcs = Rc<RefCell<World>>;

pub struct State {
    pub ecs: RCStateEcs,
    pub con: Connection,
    should_rerender_always: bool,
}

impl State {
    pub fn new(should_rerender_always: bool) -> Self {
        let mut ecs = World::new();
        ecs.register::<Path>();

        let preview = Preview::new();
        ecs.insert(preview);

        let path_renderer = PathRenderer::new();
        ecs.insert(path_renderer);

        let selection = Selection::new();
        ecs.insert(selection);

        let renderer_data = RendererData::new();
        ecs.insert(renderer_data);

        State {
            ecs: Rc::new(RefCell::new(ecs)),
            con: Connection::new(),
            should_rerender_always,
        }
    }

    pub fn setup(&mut self) {
        self.subscribe_to_backend_events();
        self.con.send_event(events::frontend::config::on_request_config());
        self.con.send_event(events::frontend::map::on_request_map()); 
    }

    fn subscribe_to_backend_events(&mut self) {
        let ecs = self.ecs.clone();

        let subscription: EventCallback = Rc::new(RefCell::new(Box::new(move |event| {
            dispatch_backend_event(&ecs, event)
        })));

        self.con.listen(subscription);
    }

    pub fn get_config(&self) -> Config {
        let ecs = self.ecs.borrow();
        let config = ecs.fetch::<Config>();
        *config
    }

    pub fn handle_event(&mut self, event: Event) {
        dispatch_frontend_event(self, event);
    }

    pub fn run_systems(&mut self) {
        self.ecs.borrow_mut().maintain();
        self.con.run_systems();
    }

    pub fn map_tile_renderer<F>(&self, cb: &mut F)
    where
        F: FnMut(&Renderable<TileTypes>, bool, i32, i32, usize, usize),
    {
        let ecs = self.ecs.borrow();
        let data = ecs.fetch::<RendererData>();
        let config = ecs.fetch::<Config>();

        let map = &data.map;
       
        if self.should_rerender_always == false && map.needs_update() == false {
            return;
        }

        let mut y = 0;
        let mut x = 0;
        let max_index = map.tiles.len() - 1;

        for (index, tile) in map.tiles.iter().enumerate() {
            if map.visited_tiles[index] == true {
                let is_visible_tile = map.visible_tiles[index];
                cb(&tile, is_visible_tile, x, y, index, max_index);
            }

            x += 1;
            if x > (config.columns - 1) as i32 {
                x = 0;
                y += 1;
            }
        }
    }

    pub fn building_renderer<F>(&self, cb: &mut F)
    where
        F: FnMut(&Frame, &BuildTime, &Renderable<BuildingIds>, usize, usize),
    {
        let ecs = self.ecs.borrow();
        let data = ecs.fetch::<RendererData>();
        let max_index = data.buildings.len();
        let mut index = 0;

        for (frame, time, building) in &data.buildings { 
            let map_index = coordinates_to_map_index(frame.x as usize, frame.y as usize, data.map.columns);

            index += 1;
            if data.map.visible_tiles[map_index] == false {
                continue;
            }
            
            cb(frame, time, building, index, max_index);
        }
    }

    pub fn unit_renderer<F>(&self, cb: &mut F)
    where
        F: FnMut(&Frame, &Renderable<UnitIds>, usize, usize),
    {
        let ecs = self.ecs.borrow();
        let data = ecs.fetch::<RendererData>();
        let max_index = data.units.len();
        let mut index = 0;

        for (frame, unit) in &data.units {
            index += 1;
            cb(&frame, &unit, index, max_index);
        }
    }

    pub fn preview_renderer<F>(&self, cb: &mut F)
    where
        F: FnMut(&Frame, &Option<Renderable<BuildingIds>>),
    {
        let ecs = self.ecs.borrow();
        let mut preview = ecs.fetch_mut::<Preview>();

        if self.should_rerender_always == false && preview.needs_update() == false {
            return;
        }

        preview.force_update(false);
        cb(&preview.frame, &preview.renderable);
    }

    pub fn selection_renderer<F>(&self, cb: &mut F)
    where
        F: FnMut(&Frame, &Option<Renderable<BuildingIds>>),
    {
        let ecs = self.ecs.borrow();
        let mut selection = ecs.fetch_mut::<Selection>();
    
        if self.should_rerender_always == false && selection.needs_update() == false {
            return;
        }

        selection.force_update(false);
        cb(&selection.frame, &selection.renderable);
    }

    pub fn resources_renderer<F>(&self, cb: &mut F) 
    where 
        F: FnMut(ResourcesDto) 
    {
        let ecs = self.ecs.borrow();
        let mut data = ecs.fetch_mut::<RendererData>();

        match &data.resources {
            None => return,
            Some(r) => cb(r.clone()),
        };
        
        data.resources = None;
    }

}
