use backend_core::prelude::*;

use common_core::prelude::*;

pub struct Connection {
    state: State,
}

impl Connection {
    pub fn new() -> Self {
        Connection {
            state: State::new(40, 20),
        }
    }

    pub fn send_event(&mut self, event: Event) {
        self.state.receive(event);
    }

    pub fn listen(&mut self, callback: EventCallback) {
        self.state.subscribe(callback.clone());
    }

    pub fn run_systems(&mut self) {
        self.state.run_systems();
    }
}
