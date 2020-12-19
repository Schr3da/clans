use common_core::prelude::*;

use crate::entities::renderer::RendererData;

use crate::state::RCStateEcs;

fn did_received_config(ecs: &RCStateEcs, config: Config) {
    ecs.borrow_mut().insert(config);
}

fn did_receive_map(ecs: &RCStateEcs, map: Map) {
    let scoped_ecs = ecs.borrow();
    let mut renderer_data = scoped_ecs.fetch_mut::<RendererData>();
    renderer_data.map = map;
}

fn did_receive_render_update(
    ecs: &RCStateEcs,
    payload: common_core::events::backend::render::SendRenderUpdatePayload,
) {
    let scoped_ecs = ecs.borrow_mut();
    let mut renderer_data = scoped_ecs.fetch_mut::<RendererData>();

    if let Some(map) = payload.map {
        renderer_data.map = map;
    }

    if let Some(buildings) = payload.buildings {
        renderer_data.buildings = buildings;
    }

    if let Some(units) = payload.units {
        renderer_data.units = units;
    }

    renderer_data.resources = payload.resources;
}

pub fn dispatch_backend_event(ecs: &RCStateEcs, event: Event) {
    match event.payload {
        Payload::SendConfig(payload) => did_received_config(ecs, payload.config),
        Payload::SendMap(payload) => did_receive_map(ecs, payload.map),
        Payload::SendRenderUpdate(payload) => did_receive_render_update(ecs, payload),
        _ => {}
    };
}
