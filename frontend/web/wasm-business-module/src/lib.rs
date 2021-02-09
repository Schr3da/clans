mod data;
mod dom;
mod dto;
mod input;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::cell::RefCell;
use std::rc::Rc;

use data::Data;

#[wasm_bindgen]
pub fn init_new_game() {
    let state = Rc::new(RefCell::new(Data::new()));

    {
        let state_retained = state.clone();
        let on_update = Closure::wrap(Box::new(move || {
            let mut scoped_state = state_retained.borrow_mut();
            scoped_state.inputs();
            scoped_state.update();
            scoped_state.render();
        }) as Box<dyn Fn()>);

        dom::window()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                on_update.as_ref().unchecked_ref(),
                16,
            )
            .unwrap();
        on_update.forget();
    }
}
