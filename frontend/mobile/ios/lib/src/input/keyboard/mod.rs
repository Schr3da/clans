use common_core::entities::building::BuildingIds;

use frontend_core::events::*;

use crate::data::Data;

pub fn inputs(_data: &mut Data, keycode: String) -> Event {
    match keycode.as_str() {
        "t" => theme::on_toggle_theme(),
        "f" => preview::on_show_preview(BuildingIds::Factory),
        "m" => preview::on_show_preview(BuildingIds::MechFactory),
        "p" => path::on_init_new_path(),
        "esc" => cancel::on_cancel(),
        _ => null::on_null(),
    }
}
