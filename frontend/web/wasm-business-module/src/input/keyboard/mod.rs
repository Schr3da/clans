use common_core::entities::building::BuildingIds;

use frontend_core::events::*;

use crate::data::Data;

pub fn inputs(_data: &mut Data, keycode: String) -> Event {
    match keycode.as_str() {
        "t" => theme::on_toggle_theme(),
        "m" => preview::on_show_preview(BuildingIds::Mine),
        "h" => preview::on_show_preview(BuildingIds::Farm),
        "f" => preview::on_show_preview(BuildingIds::Factory),
        "p" => path::on_init_new_path(),
        "Escape" => cancel::on_cancel(),
        _ => null::on_null(),
    }
}
