#[cfg(not(target_arch = "wasm32"))]
use chrono::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn current_timestamp() -> i64 {
    Utc::now().timestamp()
}

#[cfg(target_arch = "wasm32")]
use js_sys::Date;

#[cfg(target_arch = "wasm32")]
pub fn current_timestamp() -> i64 {
    let date = Date::now();
    (date / 1000.0) as i64
}
